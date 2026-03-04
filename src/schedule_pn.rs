use crate::residency_status::ResidencyStatus;

/// Two-column line: Column A (total from all sources) and Column B (attributable to N.C.).
#[derive(Debug, Clone, Copy, Default)]
pub struct TwoColumnLine {
    pub col_a: u64,
    pub col_b: u64,
}

/// Manual inputs for D-400 Schedule PN.
#[derive(Debug, Clone, Default)]
pub struct SchedulePnInput {
    // Header
    pub last_name: String,
    pub ssn: String,

    // Part A: Residency Status
    pub taxpayer_residency: ResidencyStatus,
    pub taxpayer_nc_residency_began: String,
    pub taxpayer_nc_residency_ended: String,
    pub spouse_residency: ResidencyStatus,
    pub spouse_nc_residency_began: String,
    pub spouse_nc_residency_ended: String,

    // Part B: Allocation of Income (Lines 1–15, two-column)
    /// Line 1: Wages, Salaries, Tips, Etc.
    pub wages_salaries_tips: TwoColumnLine,
    /// Line 2: Taxable Interest
    pub taxable_interest: TwoColumnLine,
    /// Line 3: Taxable Dividends
    pub taxable_dividends: TwoColumnLine,
    /// Line 4: Taxable Refunds, Credits, or Offsets of State and Local Income Taxes
    pub taxable_refunds_credits_offsets: TwoColumnLine,
    /// Line 5: Alimony Received
    pub alimony_received: TwoColumnLine,
    /// Line 6: Business Income or (Loss)
    pub business_income: TwoColumnLine,
    /// Line 7: Capital Gain or (Loss)
    pub capital_gain: TwoColumnLine,
    /// Line 8: Other Gains or (Losses)
    pub other_gains: TwoColumnLine,
    /// Line 9: Taxable Amount of IRA Distributions
    pub taxable_ira_distributions: TwoColumnLine,
    /// Line 10: Taxable Amount of Pensions and Annuities
    pub taxable_pensions_annuities: TwoColumnLine,
    /// Line 11: Rental Real Estate, Royalties, Partnerships, S-Corps, Estates, Trusts, Etc.
    pub rental_real_estate_royalties: TwoColumnLine,
    /// Line 12: Farm Income or (Loss)
    pub farm_income: TwoColumnLine,
    /// Line 13: Unemployment Compensation
    pub unemployment_compensation: TwoColumnLine,
    /// Line 14: Taxable Portion of Social Security and Railroad Retirement Benefits
    pub taxable_social_security_railroad: TwoColumnLine,
    /// Line 15: Other Income
    pub other_income: TwoColumnLine,

    // NC Adjustments — Additions (Lines 17a–17e)
    /// Line 17a: Interest Income From Obligations of States Other Than N.C.
    pub addition_interest_other_states: TwoColumnLine,
    /// Line 17b: Deferred Gains Reinvested Into an Opportunity Fund
    pub addition_deferred_gains_opportunity_fund: TwoColumnLine,
    /// Line 17c: Bonus Depreciation
    pub addition_bonus_depreciation: TwoColumnLine,
    /// Line 17d: IRC Section 179 Expense
    pub addition_irc_179_expense: TwoColumnLine,
    // NC Adjustments — Deductions (Lines 19a–19h)
    /// Line 19a: State or Local Income Tax Refund
    pub deduction_state_local_tax_refund: TwoColumnLine,
    /// Line 19b: Interest Income From Obligations of the United States
    pub deduction_interest_us_obligations: TwoColumnLine,
    /// Line 19c: Taxable Portion of Social Security and Railroad Retirement Benefits
    pub deduction_social_security_railroad: TwoColumnLine,
    /// Line 19d: Retirement Benefits (Bailey Settlement)
    pub deduction_retirement_bailey: TwoColumnLine,
    /// Line 19e: Bonus Asset Basis
    pub deduction_bonus_asset_basis: TwoColumnLine,
    /// Line 19f: Bonus Depreciation
    pub deduction_bonus_depreciation: TwoColumnLine,
    /// Line 19g: IRC Section 179 Expense
    pub deduction_irc_179_expense: TwoColumnLine,
}

/// D-400 Schedule PN — Part-Year Resident and Nonresident Schedule.
#[derive(Debug, Clone, Default)]
pub struct SchedulePn {
    // Header
    pub last_name: String,
    pub ssn: String,

    // Part A: Residency Status
    pub taxpayer_residency: ResidencyStatus,
    pub taxpayer_nc_residency_began: String,
    pub taxpayer_nc_residency_ended: String,
    pub spouse_residency: ResidencyStatus,
    pub spouse_nc_residency_began: String,
    pub spouse_nc_residency_ended: String,

    // Part B: Allocation of Income (Lines 1–16)
    /// Line 1
    pub wages_salaries_tips: TwoColumnLine,
    /// Line 2
    pub taxable_interest: TwoColumnLine,
    /// Line 3
    pub taxable_dividends: TwoColumnLine,
    /// Line 4
    pub taxable_refunds_credits_offsets: TwoColumnLine,
    /// Line 5
    pub alimony_received: TwoColumnLine,
    /// Line 6
    pub business_income: TwoColumnLine,
    /// Line 7
    pub capital_gain: TwoColumnLine,
    /// Line 8
    pub other_gains: TwoColumnLine,
    /// Line 9
    pub taxable_ira_distributions: TwoColumnLine,
    /// Line 10
    pub taxable_pensions_annuities: TwoColumnLine,
    /// Line 11
    pub rental_real_estate_royalties: TwoColumnLine,
    /// Line 12
    pub farm_income: TwoColumnLine,
    /// Line 13
    pub unemployment_compensation: TwoColumnLine,
    /// Line 14
    pub taxable_social_security_railroad: TwoColumnLine,
    /// Line 15
    pub other_income: TwoColumnLine,
    /// Line 16: Total Income (Add Lines 1 through 15)
    pub total_income: TwoColumnLine,

    // NC Adjustments — Additions
    /// Line 17a
    pub addition_interest_other_states: TwoColumnLine,
    /// Line 17b
    pub addition_deferred_gains_opportunity_fund: TwoColumnLine,
    /// Line 17c
    pub addition_bonus_depreciation: TwoColumnLine,
    /// Line 17d
    pub addition_irc_179_expense: TwoColumnLine,
    /// Line 17e
    pub addition_other: TwoColumnLine,
    /// Line 18: Total Additions (Add Lines 17a through 17e)
    pub total_additions: TwoColumnLine,

    // NC Adjustments — Deductions
    /// Line 19a
    pub deduction_state_local_tax_refund: TwoColumnLine,
    /// Line 19b
    pub deduction_interest_us_obligations: TwoColumnLine,
    /// Line 19c
    pub deduction_social_security_railroad: TwoColumnLine,
    /// Line 19d
    pub deduction_retirement_bailey: TwoColumnLine,
    /// Line 19e
    pub deduction_bonus_asset_basis: TwoColumnLine,
    /// Line 19f
    pub deduction_bonus_depreciation: TwoColumnLine,
    /// Line 19g
    pub deduction_irc_179_expense: TwoColumnLine,
    /// Line 19h
    pub deduction_other: TwoColumnLine,
    /// Line 20: Total Deductions (Add Lines 19a through 19h)
    pub total_deductions: TwoColumnLine,

    /// Line 21: Total Income Modified by N.C. Adjustments (Line 16 + Line 18 − Line 20)
    pub total_income_modified: TwoColumnLine,

    // Part C: Taxable Percentage
    /// Line 22: Amount From Column B, Line 21
    pub col_b_line_21: u64,
    /// Line 23: Amount From Column A, Line 21
    pub col_a_line_21: u64,
    /// Line 24: Taxable Percentage (Line 22 / Line 23) → D-400 Line 13, stored as 0–10000.
    pub taxable_percentage: u16,
}

/// Helper to sum two TwoColumnLine values.
fn add_two(a: TwoColumnLine, b: TwoColumnLine) -> TwoColumnLine {
    TwoColumnLine {
        col_a: a.col_a + b.col_a,
        col_b: a.col_b + b.col_b,
    }
}

/// Helper to subtract TwoColumnLine b from a.
fn sub_two(a: TwoColumnLine, b: TwoColumnLine) -> TwoColumnLine {
    TwoColumnLine {
        col_a: a.col_a - b.col_a,
        col_b: a.col_b - b.col_b,
    }
}

impl SchedulePn {
    pub fn is_valid(&self) -> bool {
        let income_lines = [
            self.wages_salaries_tips,
            self.taxable_interest,
            self.taxable_dividends,
            self.taxable_refunds_credits_offsets,
            self.alimony_received,
            self.business_income,
            self.capital_gain,
            self.other_gains,
            self.taxable_ira_distributions,
            self.taxable_pensions_annuities,
            self.rental_real_estate_royalties,
            self.farm_income,
            self.unemployment_compensation,
            self.taxable_social_security_railroad,
            self.other_income,
        ];
        let expected_total_income = TwoColumnLine {
            col_a: income_lines.iter().map(|l| l.col_a).sum(),
            col_b: income_lines.iter().map(|l| l.col_b).sum(),
        };

        let addition_lines = [
            self.addition_interest_other_states,
            self.addition_deferred_gains_opportunity_fund,
            self.addition_bonus_depreciation,
            self.addition_irc_179_expense,
            self.addition_other,
        ];
        let expected_total_additions = TwoColumnLine {
            col_a: addition_lines.iter().map(|l| l.col_a).sum(),
            col_b: addition_lines.iter().map(|l| l.col_b).sum(),
        };

        let deduction_lines = [
            self.deduction_state_local_tax_refund,
            self.deduction_interest_us_obligations,
            self.deduction_social_security_railroad,
            self.deduction_retirement_bailey,
            self.deduction_bonus_asset_basis,
            self.deduction_bonus_depreciation,
            self.deduction_irc_179_expense,
            self.deduction_other,
        ];
        let expected_total_deductions = TwoColumnLine {
            col_a: deduction_lines.iter().map(|l| l.col_a).sum(),
            col_b: deduction_lines.iter().map(|l| l.col_b).sum(),
        };

        let expected_modified = TwoColumnLine {
            col_a: self.total_income.col_a + self.total_additions.col_a
                - self.total_deductions.col_a,
            col_b: self.total_income.col_b + self.total_additions.col_b
                - self.total_deductions.col_b,
        };

        let all_lines: Vec<&TwoColumnLine> = income_lines
            .iter()
            .chain(addition_lines.iter())
            .chain(deduction_lines.iter())
            .chain([
                &self.total_income,
                &self.total_additions,
                &self.total_deductions,
                &self.total_income_modified,
            ])
            .collect();
        let col_b_le_col_a = all_lines.iter().all(|l| l.col_b <= l.col_a);

        self.total_income.col_a == expected_total_income.col_a
            && self.total_income.col_b == expected_total_income.col_b
            && self.total_additions.col_a == expected_total_additions.col_a
            && self.total_additions.col_b == expected_total_additions.col_b
            && self.total_deductions.col_a == expected_total_deductions.col_a
            && self.total_deductions.col_b == expected_total_deductions.col_b
            && self.total_income_modified.col_a == expected_modified.col_a
            && self.total_income_modified.col_b == expected_modified.col_b
            && self.taxable_percentage <= 10_000
            && col_b_le_col_a
    }

    /// Compute Schedule PN from inputs.
    pub fn compute(
        input: &SchedulePnInput,
        pn1_additions: TwoColumnLine,
        pn1_deductions: TwoColumnLine,
    ) -> Self {
        #![allow(clippy::field_reassign_with_default)]
        let mut pn = SchedulePn::default();

        // Header
        pn.last_name = input.last_name.clone();
        pn.ssn = input.ssn.clone();

        // Part A
        pn.taxpayer_residency = input.taxpayer_residency;
        pn.taxpayer_nc_residency_began = input.taxpayer_nc_residency_began.clone();
        pn.taxpayer_nc_residency_ended = input.taxpayer_nc_residency_ended.clone();
        pn.spouse_residency = input.spouse_residency;
        pn.spouse_nc_residency_began = input.spouse_nc_residency_began.clone();
        pn.spouse_nc_residency_ended = input.spouse_nc_residency_ended.clone();

        // Part B: Lines 1–15
        let income_lines = [
            &input.wages_salaries_tips,
            &input.taxable_interest,
            &input.taxable_dividends,
            &input.taxable_refunds_credits_offsets,
            &input.alimony_received,
            &input.business_income,
            &input.capital_gain,
            &input.other_gains,
            &input.taxable_ira_distributions,
            &input.taxable_pensions_annuities,
            &input.rental_real_estate_royalties,
            &input.farm_income,
            &input.unemployment_compensation,
            &input.taxable_social_security_railroad,
            &input.other_income,
        ];

        pn.wages_salaries_tips = input.wages_salaries_tips;
        pn.taxable_interest = input.taxable_interest;
        pn.taxable_dividends = input.taxable_dividends;
        pn.taxable_refunds_credits_offsets = input.taxable_refunds_credits_offsets;
        pn.alimony_received = input.alimony_received;
        pn.business_income = input.business_income;
        pn.capital_gain = input.capital_gain;
        pn.other_gains = input.other_gains;
        pn.taxable_ira_distributions = input.taxable_ira_distributions;
        pn.taxable_pensions_annuities = input.taxable_pensions_annuities;
        pn.rental_real_estate_royalties = input.rental_real_estate_royalties;
        pn.farm_income = input.farm_income;
        pn.unemployment_compensation = input.unemployment_compensation;
        pn.taxable_social_security_railroad = input.taxable_social_security_railroad;
        pn.other_income = input.other_income;

        // Line 16: Total Income
        pn.total_income = TwoColumnLine {
            col_a: income_lines.iter().map(|l| l.col_a).sum(),
            col_b: income_lines.iter().map(|l| l.col_b).sum(),
        };

        // Additions (Lines 17a–17e)
        pn.addition_interest_other_states = input.addition_interest_other_states;
        pn.addition_deferred_gains_opportunity_fund =
            input.addition_deferred_gains_opportunity_fund;
        pn.addition_bonus_depreciation = input.addition_bonus_depreciation;
        pn.addition_irc_179_expense = input.addition_irc_179_expense;
        pn.addition_other = pn1_additions;

        let addition_lines = [
            pn.addition_interest_other_states,
            pn.addition_deferred_gains_opportunity_fund,
            pn.addition_bonus_depreciation,
            pn.addition_irc_179_expense,
            pn.addition_other,
        ];
        // Line 18
        pn.total_additions = TwoColumnLine {
            col_a: addition_lines.iter().map(|l| l.col_a).sum(),
            col_b: addition_lines.iter().map(|l| l.col_b).sum(),
        };

        // Deductions (Lines 19a–19h)
        pn.deduction_state_local_tax_refund = input.deduction_state_local_tax_refund;
        pn.deduction_interest_us_obligations = input.deduction_interest_us_obligations;
        pn.deduction_social_security_railroad = input.deduction_social_security_railroad;
        pn.deduction_retirement_bailey = input.deduction_retirement_bailey;
        pn.deduction_bonus_asset_basis = input.deduction_bonus_asset_basis;
        pn.deduction_bonus_depreciation = input.deduction_bonus_depreciation;
        pn.deduction_irc_179_expense = input.deduction_irc_179_expense;
        pn.deduction_other = pn1_deductions;

        let deduction_lines = [
            pn.deduction_state_local_tax_refund,
            pn.deduction_interest_us_obligations,
            pn.deduction_social_security_railroad,
            pn.deduction_retirement_bailey,
            pn.deduction_bonus_asset_basis,
            pn.deduction_bonus_depreciation,
            pn.deduction_irc_179_expense,
            pn.deduction_other,
        ];
        // Line 20
        pn.total_deductions = TwoColumnLine {
            col_a: deduction_lines.iter().map(|l| l.col_a).sum(),
            col_b: deduction_lines.iter().map(|l| l.col_b).sum(),
        };

        // Line 21: Line 16 + Line 18 − Line 20
        pn.total_income_modified = sub_two(
            add_two(pn.total_income, pn.total_additions),
            pn.total_deductions,
        );

        // Part C
        pn.col_b_line_21 = pn.total_income_modified.col_b;
        pn.col_a_line_21 = pn.total_income_modified.col_a;

        // Line 24: taxable percentage (4 decimal places, stored as 0–10000)
        pn.taxable_percentage = if pn.col_a_line_21 == 0 {
            0
        } else {
            let num = pn.col_b_line_21;
            let den = pn.col_a_line_21;
            ((num * 10_000 + den / 2) / den) as u16
        };

        pn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_income_sums_all_lines() {
        let input = SchedulePnInput {
            wages_salaries_tips: TwoColumnLine {
                col_a: 50_000,
                col_b: 30_000,
            },
            taxable_interest: TwoColumnLine {
                col_a: 1_000,
                col_b: 500,
            },
            ..Default::default()
        };
        let pn = SchedulePn::compute(&input, TwoColumnLine::default(), TwoColumnLine::default());
        assert_eq!(pn.total_income.col_a, 51_000);
        assert_eq!(pn.total_income.col_b, 30_500);
    }

    #[test]
    fn taxable_percentage_computed_correctly() {
        let input = SchedulePnInput {
            wages_salaries_tips: TwoColumnLine {
                col_a: 100_000,
                col_b: 60_000,
            },
            ..Default::default()
        };
        let pn = SchedulePn::compute(&input, TwoColumnLine::default(), TwoColumnLine::default());
        assert_eq!(pn.taxable_percentage, 6000);
    }

    #[test]
    fn zero_total_income_gives_zero_percentage() {
        let pn = SchedulePn::compute(
            &SchedulePnInput::default(),
            TwoColumnLine::default(),
            TwoColumnLine::default(),
        );
        assert_eq!(pn.taxable_percentage, 0);
    }

    #[test]
    fn line_21_is_income_plus_additions_minus_deductions() {
        let input = SchedulePnInput {
            wages_salaries_tips: TwoColumnLine {
                col_a: 80_000,
                col_b: 40_000,
            },
            addition_interest_other_states: TwoColumnLine {
                col_a: 1_000,
                col_b: 500,
            },
            deduction_state_local_tax_refund: TwoColumnLine {
                col_a: 2_000,
                col_b: 1_000,
            },
            ..Default::default()
        };
        let pn = SchedulePn::compute(&input, TwoColumnLine::default(), TwoColumnLine::default());
        assert_eq!(pn.total_income_modified.col_a, 79_000);
        assert_eq!(pn.total_income_modified.col_b, 39_500);
    }

    #[test]
    fn computed_schedule_pn_is_valid() {
        let input = SchedulePnInput {
            wages_salaries_tips: TwoColumnLine {
                col_a: 100_000,
                col_b: 60_000,
            },
            taxable_interest: TwoColumnLine {
                col_a: 1_000,
                col_b: 500,
            },
            ..Default::default()
        };
        let pn = SchedulePn::compute(&input, TwoColumnLine::default(), TwoColumnLine::default());
        assert!(pn.is_valid());
    }

    #[test]
    fn inconsistent_schedule_pn_is_invalid() {
        let mut pn = SchedulePn::compute(
            &SchedulePnInput::default(),
            TwoColumnLine::default(),
            TwoColumnLine::default(),
        );
        pn.total_income.col_a = 999;
        assert!(!pn.is_valid());
    }
}
