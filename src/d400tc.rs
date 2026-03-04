use crate::nc_478::Nc478;
use crate::nc_rehab::NcRehab;

/// Manual inputs for D-400TC (Tax Credits).
#[derive(Debug, Clone, Default)]
pub struct D400TcInput {
    // Header
    pub name: String,
    pub ssn: String,

    // Part 1: Credit for Income Tax Paid to Another State
    /// Line 1: Total income from all sources as modified by NC.
    pub total_income_all_sources_modified_by_nc: u64,
    /// Line 2: Portion of Line 1 income taxed by another state.
    pub portion_taxed_by_another_state: u64,
    /// Line 6: Net tax actually paid to other state(s).
    pub net_tax_paid_other_state: u64,
    /// Line 7b: Number of states for which credit is claimed.
    pub number_of_states_claimed: u32,

    // Part 2: Credits for Rehabilitating Historic Structures
    // Lines 8–9 come from NC-Rehab when provided.
    // Lines 10–13 (mill/3L) remain manual since they come from separate forms.
    /// Line 10a: Income-producing mill facility (Article 3H) — expenditure.
    pub income_producing_mill_facility_3h_expenditure: u64,
    /// Line 10b: Income-producing mill facility (Article 3H) — credit.
    pub income_producing_mill_facility_3h_credit: u64,
    /// Line 11a: Nonincome-producing mill facility (Article 3H) — expenditure.
    pub nonincome_producing_mill_facility_3h_expenditure: u64,
    /// Line 11b: Nonincome-producing mill facility (Article 3H) — credit.
    pub nonincome_producing_mill_facility_3h_credit: u64,
    /// Line 12: Income-producing historic structure (Article 3L).
    pub income_producing_historic_structure_3l: u64,
    /// Line 13: Nonincome-producing historic structure (Article 3L).
    pub nonincome_producing_historic_structure_3l: u64,

    // Part 3: Additional credits
    /// Line 14: Tax credits carried from previous years, excluding carryover of income tax credits taken on NC-Rehab or NC-478
    pub tax_credits_carried_from_previous_years: u64,
    // Line 19 comes from NC-478 when provided.
}

/// D-400TC — NC Tax Credits.
#[derive(Debug, Clone, Default)]
pub struct D400Tc {
    // Header
    pub name: String,
    pub ssn: String,

    // Part 1: Credit for Income Tax Paid to Another State
    /// Line 1
    pub total_income_all_sources_modified_by_nc: u64,
    /// Line 2
    pub portion_taxed_by_another_state: u64,
    /// Line 3: Ratio of line 2 to line 1 (rounded to 4 decimal places, stored as 0–10000).
    pub ratio_line2_to_line1: u16,
    /// Line 4: NC income tax from D-400 Line 15.
    pub nc_income_tax_from_d400_line_15: u64,
    /// Line 5: Line 4 × Line 3.
    pub multiply_line4_by_line3: u64,
    /// Line 6: Net tax paid to other state.
    pub net_tax_paid_other_state: u64,
    /// Line 7a: Credit — lesser of line 5 or line 6.
    pub credit_income_tax_other_state: u64,
    /// Line 7b: Number of states claimed.
    pub number_of_states_claimed: u32,

    // Part 2: Historic structure credits
    /// Line 8
    pub income_producing_historic_structure_3d: u64,
    /// Line 9
    pub nonincome_producing_historic_structure_3d: u64,
    /// Line 10a
    pub income_producing_mill_facility_3h_expenditure: u64,
    /// Line 10b
    pub income_producing_mill_facility_3h_credit: u64,
    /// Line 11a
    pub nonincome_producing_mill_facility_3h_expenditure: u64,
    /// Line 11b
    pub nonincome_producing_mill_facility_3h_credit: u64,
    /// Line 12
    pub income_producing_historic_structure_3l: u64,
    /// Line 13
    pub nonincome_producing_historic_structure_3l: u64,

    // Part 3: Computation of Total Tax Credits
    /// Line 14
    pub tax_credits_carried_from_previous_years: u64,
    /// Line 15: Reserved for future use.
    pub reserved_for_future_use: (),
    /// Line 16: Sum of lines 7a + 8 + 9 + 10b + 11b + 12 + 13 + 14.
    pub total_credits_sum: u64,
    /// Line 17: NC income tax from D-400 Line 15.
    pub nc_income_tax_from_d400_line_15_part3: u64,
    /// Line 18: Lesser of line 16 or line 17.
    pub lesser_of_line_16_or_17: u64,
    /// Line 19: Business incentive and energy tax credits.
    pub business_incentive_energy_tax_credits: u64,
    /// Line 20: Total tax credits (line 18 + line 19), cannot exceed D-400 Line 15.
    pub total_tax_credits: u64,
}

impl D400Tc {
    pub fn is_valid(&self) -> bool {
        let expected_ratio = if self.total_income_all_sources_modified_by_nc == 0 {
            0u16
        } else {
            let num = self.portion_taxed_by_another_state;
            let den = self.total_income_all_sources_modified_by_nc;
            ((num * 10_000 + den / 2) / den) as u16
        };

        let expected_line5 =
            (self.nc_income_tax_from_d400_line_15 * self.ratio_line2_to_line1 as u64 + 5_000)
                / 10_000;

        let expected_credit = expected_line5.min(self.net_tax_paid_other_state);

        let expected_total_credits_sum: u64 = [
            self.credit_income_tax_other_state,
            self.income_producing_historic_structure_3d,
            self.nonincome_producing_historic_structure_3d,
            self.income_producing_mill_facility_3h_credit,
            self.nonincome_producing_mill_facility_3h_credit,
            self.income_producing_historic_structure_3l,
            self.nonincome_producing_historic_structure_3l,
            self.tax_credits_carried_from_previous_years,
        ]
        .into_iter()
        .sum();

        let expected_lesser = self
            .total_credits_sum
            .min(self.nc_income_tax_from_d400_line_15_part3);

        let expected_total = (self.lesser_of_line_16_or_17
            + self.business_incentive_energy_tax_credits)
            .min(self.nc_income_tax_from_d400_line_15_part3);

        self.ratio_line2_to_line1 == expected_ratio
            && self.ratio_line2_to_line1 <= 10_000
            && self.multiply_line4_by_line3 == expected_line5
            && self.credit_income_tax_other_state == expected_credit
            && self.total_credits_sum == expected_total_credits_sum
            && self.lesser_of_line_16_or_17 == expected_lesser
            && self.total_tax_credits == expected_total
            && self.total_tax_credits <= self.nc_income_tax_from_d400_line_15_part3
    }

    /// Compute D-400TC from inputs and NC income tax (D-400 Line 15).
    pub fn compute(
        input: &D400TcInput,
        nc_income_tax: u64,
        nc_rehab: Option<&NcRehab>,
        nc_478: Option<&Nc478>,
    ) -> Self {
        let mut tc = D400Tc::default();

        // Header
        tc.name = input.name.clone();
        tc.ssn = input.ssn.clone();

        // Part 1: Credit for Income Tax Paid to Another State
        tc.total_income_all_sources_modified_by_nc = input.total_income_all_sources_modified_by_nc;
        tc.portion_taxed_by_another_state = input.portion_taxed_by_another_state;

        // Line 3: ratio rounded to 4 decimal places, stored as 0–10000
        tc.ratio_line2_to_line1 = if tc.total_income_all_sources_modified_by_nc == 0 {
            0
        } else {
            // (numerator * 10000 + denominator/2) / denominator for rounding
            let num = tc.portion_taxed_by_another_state;
            let den = tc.total_income_all_sources_modified_by_nc;
            ((num * 10_000 + den / 2) / den) as u16
        };

        // Line 4
        tc.nc_income_tax_from_d400_line_15 = nc_income_tax;

        // Line 5: line 4 × line 3 (dollars × ratio / 10000)
        tc.multiply_line4_by_line3 =
            (nc_income_tax * tc.ratio_line2_to_line1 as u64 + 5_000) / 10_000;

        // Line 6
        tc.net_tax_paid_other_state = input.net_tax_paid_other_state;

        // Line 7a: min(line 5, line 6)
        tc.credit_income_tax_other_state =
            tc.multiply_line4_by_line3.min(tc.net_tax_paid_other_state);

        tc.number_of_states_claimed = input.number_of_states_claimed;

        // Part 2: Historic structure credits
        // Lines 8–9 come from NC-Rehab if provided
        if let Some(rehab) = nc_rehab {
            tc.income_producing_historic_structure_3d = rehab.line_23_total_income_producing.income;
            tc.nonincome_producing_historic_structure_3d =
                rehab.line_26_total_nonincome_producing.income;
        }
        tc.income_producing_mill_facility_3h_expenditure =
            input.income_producing_mill_facility_3h_expenditure;
        tc.income_producing_mill_facility_3h_credit =
            input.income_producing_mill_facility_3h_credit;
        tc.nonincome_producing_mill_facility_3h_expenditure =
            input.nonincome_producing_mill_facility_3h_expenditure;
        tc.nonincome_producing_mill_facility_3h_credit =
            input.nonincome_producing_mill_facility_3h_credit;
        tc.income_producing_historic_structure_3l = input.income_producing_historic_structure_3l;
        tc.nonincome_producing_historic_structure_3l =
            input.nonincome_producing_historic_structure_3l;

        // Part 3
        tc.tax_credits_carried_from_previous_years = input.tax_credits_carried_from_previous_years;

        // Line 16
        tc.total_credits_sum = [
            tc.credit_income_tax_other_state,
            tc.income_producing_historic_structure_3d,
            tc.nonincome_producing_historic_structure_3d,
            tc.income_producing_mill_facility_3h_credit,
            tc.nonincome_producing_mill_facility_3h_credit,
            tc.income_producing_historic_structure_3l,
            tc.nonincome_producing_historic_structure_3l,
            tc.tax_credits_carried_from_previous_years,
        ]
        .into_iter()
        .sum();

        // Line 17
        tc.nc_income_tax_from_d400_line_15_part3 = nc_income_tax;

        // Line 18: min(line 16, line 17)
        tc.lesser_of_line_16_or_17 = tc
            .total_credits_sum
            .min(tc.nc_income_tax_from_d400_line_15_part3);

        // Line 19: from NC-478 if provided
        if let Some(nc478) = nc_478 {
            tc.business_incentive_energy_tax_credits = nc478.total_credits_taken.income;
        }

        // Line 20: (line 18 + line 19), cannot exceed D-400 Line 15
        tc.total_tax_credits = (tc.lesser_of_line_16_or_17
            + tc.business_incentive_energy_tax_credits)
            .min(nc_income_tax);

        tc
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn credit_cannot_exceed_nc_income_tax() {
        let input = D400TcInput {
            total_income_all_sources_modified_by_nc: 100_000,
            portion_taxed_by_another_state: 50_000,
            net_tax_paid_other_state: 99_999,
            ..Default::default()
        };
        let nc_tax = 4_250;
        // Provide a mock NC-478 with large credits to test the cap
        use crate::nc_478::Nc478;
        use crate::nc_rehab::FranchiseIncome;
        let nc_478 = Nc478 {
            total_credits_taken: FranchiseIncome {
                franchise: 0,
                income: 99_999,
            },
            ..Default::default()
        };
        let tc = D400Tc::compute(&input, nc_tax, None, Some(&nc_478));
        assert_eq!(tc.total_tax_credits, nc_tax);
    }

    #[test]
    fn ratio_rounds_to_4_decimal_places() {
        let input = D400TcInput {
            total_income_all_sources_modified_by_nc: 100_000,
            portion_taxed_by_another_state: 33_333,
            ..Default::default()
        };
        let tc = D400Tc::compute(&input, 4_250, None, None);
        // 33333/100000 = 0.33333, rounded to 4 dp = 3333
        assert_eq!(tc.ratio_line2_to_line1, 3333);
    }

    #[test]
    fn credit_is_lesser_of_line5_and_line6() {
        let input = D400TcInput {
            total_income_all_sources_modified_by_nc: 100_000,
            portion_taxed_by_another_state: 50_000,
            net_tax_paid_other_state: 1_000,
            ..Default::default()
        };
        let nc_tax = 4_250;
        let tc = D400Tc::compute(&input, nc_tax, None, None);
        // Line 5: 4250 * 5000 / 10000 = 2125
        // Line 6: 1000
        // Line 7a: min(2125, 1000) = 1000
        assert_eq!(tc.credit_income_tax_other_state, 1_000);
    }

    #[test]
    fn zero_income_gives_zero_ratio() {
        let input = D400TcInput::default();
        let tc = D400Tc::compute(&input, 1_000, None, None);
        assert_eq!(tc.ratio_line2_to_line1, 0);
        assert_eq!(tc.credit_income_tax_other_state, 0);
    }

    #[test]
    fn computed_d400tc_is_valid() {
        let input = D400TcInput {
            total_income_all_sources_modified_by_nc: 100_000,
            portion_taxed_by_another_state: 50_000,
            net_tax_paid_other_state: 1_000,
            tax_credits_carried_from_previous_years: 200,
            ..Default::default()
        };
        let tc = D400Tc::compute(&input, 4_250, None, None);
        assert!(tc.is_valid());
    }

    #[test]
    fn inconsistent_d400tc_is_invalid() {
        let mut tc = D400Tc::compute(&D400TcInput::default(), 1_000, None, None);
        tc.total_credits_sum = 999;
        assert!(!tc.is_valid());
    }

    #[test]
    fn total_credits_sum_includes_all_components() {
        use crate::nc_rehab::{FranchiseIncome, NcRehab};
        let input = D400TcInput {
            total_income_all_sources_modified_by_nc: 100_000,
            portion_taxed_by_another_state: 50_000,
            net_tax_paid_other_state: 500,
            income_producing_mill_facility_3h_credit: 300,
            nonincome_producing_mill_facility_3h_credit: 150,
            income_producing_historic_structure_3l: 50,
            nonincome_producing_historic_structure_3l: 75,
            tax_credits_carried_from_previous_years: 125,
            ..Default::default()
        };
        let nc_rehab = NcRehab {
            line_23_total_income_producing: FranchiseIncome {
                franchise: 0,
                income: 100,
            },
            line_26_total_nonincome_producing: FranchiseIncome {
                franchise: 0,
                income: 200,
            },
            ..Default::default()
        };
        let nc_tax = 10_000;
        let tc = D400Tc::compute(&input, nc_tax, Some(&nc_rehab), None);
        // Line 7a: min(line5, 500) = 500 (since line5 = 10000*0.5 = 5000 > 500)
        // Line 16: 500 + 100 + 200 + 300 + 150 + 50 + 75 + 125 = 1500
        assert_eq!(tc.total_credits_sum, 1_500);
    }
}
