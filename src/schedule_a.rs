use us_tax_brackets::FilingStatus;

use crate::constants::{
    MEDICAL_THRESHOLD_BPS, MORTGAGE_PROPERTY_TAX_LIMIT, STANDARD_DEDUCTION_HOH,
    STANDARD_DEDUCTION_MFJ, STANDARD_DEDUCTION_MFS, STANDARD_DEDUCTION_QSS,
    STANDARD_DEDUCTION_SINGLE,
};

/// NC standard deduction amounts for tax year 2025.
pub fn nc_standard_deduction(
    filing_status: FilingStatus,
    spouse_itemizes: bool,
    eligible_for_federal_standard_deduction: bool,
) -> u64 {
    if !eligible_for_federal_standard_deduction {
        return 0;
    }
    match filing_status {
        FilingStatus::Single => STANDARD_DEDUCTION_SINGLE,
        FilingStatus::HeadOfHousehold => STANDARD_DEDUCTION_HOH,
        FilingStatus::MarriedFilingJointly => STANDARD_DEDUCTION_MFJ,
        FilingStatus::QualifyingSurvivingSpouse => STANDARD_DEDUCTION_QSS,
        FilingStatus::MarriedFilingSeparately => {
            if spouse_itemizes {
                0
            } else {
                STANDARD_DEDUCTION_MFS
            }
        }
    }
}

/// Manual inputs for Schedule A (NC Itemized Deductions).
#[derive(Debug, Clone, Default)]
pub struct ScheduleAInput {
    // Header
    pub name: String,
    pub ssn: String,

    /// Line 1: Home mortgage interest.
    pub home_mortgage_interest: u64,
    /// Line 2: Real estate / property taxes.
    pub real_estate_property_taxes: u64,
    /// Line 6: Charitable contributions.
    pub charitable_contributions: u64,
    /// Line 7a: Medical and dental expenses (before limitation).
    pub medical_dental_expenses_before_limitation: u64,
    /// Line 8: Repayment of claim-of-right income.
    pub repayment_claim_of_right_income: u64,
}

/// D-400 Schedule A — NC Itemized Deductions.
#[derive(Debug, Clone, Default)]
pub struct ScheduleA {
    // Header
    pub name: String,
    pub ssn: String,

    /// Line 1: Home mortgage interest.
    pub home_mortgage_interest: u64,
    /// Line 2: Real estate / property taxes.
    pub real_estate_property_taxes: u64,
    /// Line 3: Sum of lines 1 + 2.
    pub home_mortgage_and_property_taxes_before_limitation: u64,
    /// Line 4: Limitation ($20,000).
    pub home_mortgage_property_tax_limitation: u64,
    /// Line 5: Lesser of line 3 or line 4.
    pub home_mortgage_property_taxes_after_limitation: u64,
    /// Line 6: Charitable contributions.
    pub charitable_contributions: u64,
    /// Line 7a: Medical and dental expenses before limitation.
    pub medical_dental_expenses_before_limitation: u64,
    /// Line 7b: Federal AGI (from D-400 Line 6).
    pub federal_agi_from_d400_line_6: u64,
    /// Line 7c: Medical/dental threshold (7b × 7.5%, zero floor).
    pub medical_dental_threshold: u64,
    /// Line 7d: Medical/dental after limitation (max(7a − 7c, 0)).
    pub medical_dental_after_limitation: u64,
    /// Line 8: Repayment of claim-of-right income.
    pub repayment_claim_of_right_income: u64,
    /// Line 9: Reserved for future use.
    pub reserved_for_future_use: (),
    /// Line 10: Total NC itemized deductions (5 + 6 + 7d + 8) → D-400 Line 11.
    pub total_nc_itemized_deductions: u64,
}

impl ScheduleA {
    pub fn is_valid(&self) -> bool {
        self.home_mortgage_and_property_taxes_before_limitation
            == self.home_mortgage_interest + self.real_estate_property_taxes
            && self.home_mortgage_property_taxes_after_limitation
                == self
                    .home_mortgage_and_property_taxes_before_limitation
                    .min(self.home_mortgage_property_tax_limitation)
            && self.medical_dental_threshold
                == (self.federal_agi_from_d400_line_6 * MEDICAL_THRESHOLD_BPS + 5_000) / 10_000
            && self.medical_dental_after_limitation
                == self
                    .medical_dental_expenses_before_limitation
                    .saturating_sub(self.medical_dental_threshold)
            && self.total_nc_itemized_deductions
                == self.home_mortgage_property_taxes_after_limitation
                    + self.charitable_contributions
                    + self.medical_dental_after_limitation
                    + self.repayment_claim_of_right_income
    }

    /// Compute Schedule A from inputs and federal AGI.
    pub fn compute(input: &ScheduleAInput, federal_agi: u64) -> Self {
        let mut a = ScheduleA::default();

        // Header
        a.name = input.name.clone();
        a.ssn = input.ssn.clone();

        // Lines 1–2
        a.home_mortgage_interest = input.home_mortgage_interest;
        a.real_estate_property_taxes = input.real_estate_property_taxes;

        // Line 3
        a.home_mortgage_and_property_taxes_before_limitation =
            a.home_mortgage_interest + a.real_estate_property_taxes;

        // Line 4 (constant)
        a.home_mortgage_property_tax_limitation = MORTGAGE_PROPERTY_TAX_LIMIT;

        // Line 5: min(line 3, line 4)
        a.home_mortgage_property_taxes_after_limitation = a
            .home_mortgage_and_property_taxes_before_limitation
            .min(a.home_mortgage_property_tax_limitation);

        // Line 6
        a.charitable_contributions = input.charitable_contributions;

        // Line 7a
        a.medical_dental_expenses_before_limitation =
            input.medical_dental_expenses_before_limitation;

        // Line 7b
        a.federal_agi_from_d400_line_6 = federal_agi;

        // Line 7c: AGI × 7.5% (dollars * 750 / 10000)
        a.medical_dental_threshold = (federal_agi * MEDICAL_THRESHOLD_BPS + 5_000) / 10_000;

        // Line 7d: max(7a − 7c, 0)
        a.medical_dental_after_limitation = a
            .medical_dental_expenses_before_limitation
            .saturating_sub(a.medical_dental_threshold);

        // Line 8
        a.repayment_claim_of_right_income = input.repayment_claim_of_right_income;

        // Line 10: 5 + 6 + 7d + 8
        a.total_nc_itemized_deductions = [
            a.home_mortgage_property_taxes_after_limitation,
            a.charitable_contributions,
            a.medical_dental_after_limitation,
            a.repayment_claim_of_right_income,
        ]
        .into_iter()
        .sum();

        a
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn standard_deduction_single() {
        assert_eq!(
            nc_standard_deduction(FilingStatus::Single, false, true),
            12_750
        );
    }

    #[test]
    fn standard_deduction_hoh() {
        assert_eq!(
            nc_standard_deduction(FilingStatus::HeadOfHousehold, false, true),
            19_125
        );
    }

    #[test]
    fn standard_deduction_mfj() {
        assert_eq!(
            nc_standard_deduction(FilingStatus::MarriedFilingJointly, false, true),
            25_500
        );
    }

    #[test]
    fn standard_deduction_qss() {
        assert_eq!(
            nc_standard_deduction(FilingStatus::QualifyingSurvivingSpouse, false, true),
            25_500
        );
    }

    #[test]
    fn standard_deduction_mfs_spouse_does_not_itemize() {
        assert_eq!(
            nc_standard_deduction(FilingStatus::MarriedFilingSeparately, false, true),
            12_750
        );
    }

    #[test]
    fn standard_deduction_mfs_spouse_itemizes() {
        assert_eq!(
            nc_standard_deduction(FilingStatus::MarriedFilingSeparately, true, true),
            0
        );
    }

    #[test]
    fn standard_deduction_not_eligible_for_federal() {
        assert_eq!(nc_standard_deduction(FilingStatus::Single, false, false), 0);
    }

    #[test]
    fn itemized_mortgage_property_tax_capped_at_20000() {
        let input = ScheduleAInput {
            home_mortgage_interest: 15_000,
            real_estate_property_taxes: 10_000,
            ..Default::default()
        };
        let a = ScheduleA::compute(&input, 100_000);
        // 15000 + 10000 = 25000, capped at 20000
        assert_eq!(a.home_mortgage_property_taxes_after_limitation, 20_000);
    }

    #[test]
    fn itemized_mortgage_property_tax_under_limit() {
        let input = ScheduleAInput {
            home_mortgage_interest: 8_000,
            real_estate_property_taxes: 3_000,
            ..Default::default()
        };
        let a = ScheduleA::compute(&input, 100_000);
        assert_eq!(a.home_mortgage_property_taxes_after_limitation, 11_000);
    }

    #[test]
    fn medical_dental_threshold_75_percent() {
        let input = ScheduleAInput {
            medical_dental_expenses_before_limitation: 10_000,
            ..Default::default()
        };
        let a = ScheduleA::compute(&input, 100_000);
        // 7.5% of 100000 = 7500; 10000 - 7500 = 2500
        assert_eq!(a.medical_dental_threshold, 7_500);
        assert_eq!(a.medical_dental_after_limitation, 2_500);
    }

    #[test]
    fn medical_dental_below_threshold_is_zero() {
        let input = ScheduleAInput {
            medical_dental_expenses_before_limitation: 5_000,
            ..Default::default()
        };
        let a = ScheduleA::compute(&input, 100_000);
        // 7.5% of 100000 = 7500 > 5000 → 0
        assert_eq!(a.medical_dental_after_limitation, 0);
    }

    #[test]
    fn total_itemized_deductions() {
        let input = ScheduleAInput {
            home_mortgage_interest: 8_000,
            real_estate_property_taxes: 4_000,
            charitable_contributions: 3_000,
            medical_dental_expenses_before_limitation: 10_000,
            repayment_claim_of_right_income: 500,
            ..Default::default()
        };
        let a = ScheduleA::compute(&input, 80_000);
        assert_eq!(a.total_nc_itemized_deductions, 19_500);
    }

    #[test]
    fn computed_schedule_a_is_valid() {
        let input = ScheduleAInput {
            home_mortgage_interest: 15_000,
            real_estate_property_taxes: 10_000,
            charitable_contributions: 5_000,
            medical_dental_expenses_before_limitation: 10_000,
            repayment_claim_of_right_income: 500,
            ..Default::default()
        };
        let a = ScheduleA::compute(&input, 100_000);
        assert!(a.is_valid());
    }

    #[test]
    fn inconsistent_schedule_a_is_invalid() {
        let mut a = ScheduleA::compute(&ScheduleAInput::default(), 50_000);
        a.total_nc_itemized_deductions = 999;
        assert!(!a.is_valid());
    }
}
