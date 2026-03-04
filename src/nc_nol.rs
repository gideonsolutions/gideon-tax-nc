use crate::constants::{CAPITAL_LOSS_LIMIT, CAPITAL_LOSS_LIMIT_MFS};
use us_tax_brackets::FilingStatus;

/// Manual inputs for NC-NOL Part 1 (Net Operating Loss Calculation).
#[derive(Debug, Clone)]
pub struct NcNolPart1Input {
    // Header
    pub name: String,
    pub federal_tax_id: String,
    pub filing_status: FilingStatus,

    /// Line 1a: Federal Adjusted Gross Income
    pub federal_agi: u64,
    /// Line 1b: Additions to Federal Adjusted Gross Income
    pub additions_to_federal_agi: u64,
    /// Line 1d: Deductions from Federal Adjusted Gross Income
    pub deductions_from_federal_agi: u64,
    /// Line 1e: N.C. Standard Deduction Amount or N.C. Itemized Deduction Amount
    pub nc_standard_or_itemized_deduction: u64,
    /// Line 1f: Excess Business Loss Included as Other Income on 2025 Federal Return
    pub excess_business_loss_other_income: u64,
    /// Line 2: Nonbusiness Capital Losses (positive number)
    pub nonbusiness_capital_losses: u64,
    /// Line 3: Nonbusiness Capital Gains (without IRC section 1202 exclusion)
    pub nonbusiness_capital_gains: u64,
    /// Line 6: Nonbusiness Deductions (positive number)
    pub nonbusiness_deductions: u64,
    /// Line 7: Nonbusiness Income Other than Capital Gains
    pub nonbusiness_income_other_than_capital_gains: u64,
    /// Line 11: Business Capital Losses Before Limitations (positive number)
    pub business_capital_losses_before_limitations: u64,
    /// Line 12: Business Capital Gains (without IRC section 1202 exclusion)
    pub business_capital_gains: u64,
    /// Line 16a: Net Short-Term Capital Gain (Loss)
    pub net_short_term_capital_gain_loss: u64,
    /// Line 16b: Net Long-Term Capital Gain (Loss)
    pub net_long_term_capital_gain_loss: u64,
    /// Line 17: IRC Section 1202 Exclusion
    pub irc_1202_exclusion: u64,
    /// Line 23: N.C. NOL Deduction for Losses from Prior Years (positive number)
    pub nc_nol_deduction_prior_years: u64,
    /// Whether Lines 16–21 should be skipped (no loss on Line 16c and no 1202 exclusion)
    pub skip_lines_16_through_21: bool,
}

/// A single row in the federal NOL carryover table (Part 2A).
#[derive(Debug, Clone, Copy, Default)]
pub struct FederalNolCarryoverRow {
    /// Column A: Federal NOL Incurred and not Absorbed by January 1, 2022
    pub col_a: u64,
    /// Column B: Federal NOL Carryover Deducted as N.C. NOL in Tax Year 2025
    pub col_b: u64,
}

/// A single row in the N.C. NOL carryover table (Part 2B).
#[derive(Debug, Clone, Copy, Default)]
pub struct NcNolCarryoverRow {
    /// Column A: N.C. NOL Carryover
    pub col_a: u64,
    /// Column B: N.C. NOL Carryover Deducted as N.C. NOL in Tax Year 2025
    pub col_b: u64,
}

/// Manual inputs for NC-NOL Part 2.
#[derive(Debug, Clone, Default)]
pub struct NcNolPart2Input {
    /// Part 2A: Federal NOL Carryover (Lines 1–12, years 2010–2021)
    pub federal_nol_carryover: [FederalNolCarryoverRow; 12],
    /// Part 2B: N.C. NOL Carryover (Lines 14–16, years 2022–2024)
    pub nc_nol_carryover: [NcNolCarryoverRow; 3],
}

/// NC-NOL Part 1 — Net Operating Loss Calculation.
#[derive(Debug, Clone, Default)]
pub struct NcNolPart1 {
    // Header
    pub name: String,
    pub federal_tax_id: String,

    /// Line 1a
    pub federal_agi: u64,
    /// Line 1b
    pub additions_to_federal_agi: u64,
    /// Line 1c: Add Lines 1a and 1b
    pub line_1c: u64,
    /// Line 1d
    pub deductions_from_federal_agi: u64,
    /// Line 1e
    pub nc_standard_or_itemized_deduction: u64,
    /// Line 1f
    pub excess_business_loss_other_income: u64,
    /// Line 1g: Add Lines 1d through 1f
    pub line_1g: u64,
    /// Line 1: Subtract Line 1g from Line 1c
    pub line_1: u64,
    /// Line 2
    pub nonbusiness_capital_losses: u64,
    /// Line 3
    pub nonbusiness_capital_gains: u64,
    /// Line 4: If Line 2 > Line 3, Line 2 − Line 3; else zero
    pub line_4: u64,
    /// Line 5: If Line 3 > Line 2, Line 3 − Line 2; else zero
    pub line_5: u64,
    /// Line 6
    pub nonbusiness_deductions: u64,
    /// Line 7
    pub nonbusiness_income_other_than_capital_gains: u64,
    /// Line 8: Add Line 5 and Line 7
    pub line_8: u64,
    /// Line 9: If Line 6 > Line 8, Line 6 − Line 8; else zero
    pub line_9: u64,
    /// Line 10: If Line 8 > Line 6, Line 8 − Line 6; else zero (cannot exceed Line 5)
    pub line_10: u64,
    /// Line 11
    pub business_capital_losses_before_limitations: u64,
    /// Line 12
    pub business_capital_gains: u64,
    /// Line 13: Add Line 10 and Line 12
    pub line_13: u64,
    /// Line 14: Subtract Line 13 from Line 11 (zero floor)
    pub line_14: u64,
    /// Line 15: Add Line 4 and Line 14
    pub line_15: u64,
    /// Line 16a
    pub net_short_term_capital_gain_loss: u64,
    /// Line 16b
    pub net_long_term_capital_gain_loss: u64,
    /// Line 16c: Add Lines 16a and 16b
    pub line_16c: u64,
    /// Line 16: Amount of Line 16c (positive number)
    pub line_16: u64,
    /// Line 17: IRC Section 1202 Exclusion
    pub irc_1202_exclusion: u64,
    /// Line 18: Subtract Line 17 from Line 16 (zero floor)
    pub line_18: u64,
    /// Line 19: Smaller of Line 16 or $3,000 (or $1,500 if MFS)
    pub line_19: u64,
    /// Line 20: If Line 18 > Line 19, Line 18 − Line 19; else zero
    pub line_20: u64,
    /// Line 21: If Line 19 > Line 18, Line 19 − Line 18; else zero
    pub line_21: u64,
    /// Line 22: Subtract Line 20 from Line 15 (zero floor)
    pub line_22: u64,
    /// Line 23: N.C. NOL Deduction for Losses from Prior Years (positive number)
    pub nc_nol_deduction_prior_years: u64,
    /// Line 24: N.C. NOL (Add Lines 1, 9, 17, 21, 22, and 23; if < 0, that's the NOL)
    pub nc_nol: u64,
}

/// NC-NOL Part 2 — Carryover Deductions.
#[derive(Debug, Clone, Default)]
pub struct NcNolPart2 {
    // Part 2A: Federal NOL Carryover (Lines 1–12)
    pub federal_nol_carryover: [FederalNolCarryoverRow; 12],
    /// Column C is computed: col_a − col_b for each row
    pub federal_nol_carryover_remaining: [u64; 12],
    /// Line 13: Sum of Column B (Federal NOL Carryover Deduction)
    pub federal_nol_carryover_deduction: u64,

    // Part 2B: N.C. NOL Carryover (Lines 14–16)
    pub nc_nol_carryover: [NcNolCarryoverRow; 3],
    /// Column C: col_a − col_b for each row
    pub nc_nol_carryover_remaining: [u64; 3],
    /// Line 17: Sum of Column B (N.C. NOL Carryover Deduction)
    pub nc_nol_carryover_deduction: u64,

    // Part 2C
    /// Line 18: Federal NOL Carryover Deduction Amount (from Part 2A Line 13)
    pub line_18: u64,
    /// Line 19: N.C. NOL Carryover Deduction Amount (from Part 2B Line 17)
    pub line_19: u64,
    /// Line 20: Total N.C. NOL Deduction (Line 18 + Line 19) → Schedule S Line 39
    pub total_nc_nol_deduction: u64,
}

impl NcNolPart1 {
    pub fn is_valid(&self) -> bool {
        self.line_1c == self.federal_agi + self.additions_to_federal_agi
            && self.line_1g
                == self.deductions_from_federal_agi
                    + self.nc_standard_or_itemized_deduction
                    + self.excess_business_loss_other_income
            && self.line_1 == self.line_1c.saturating_sub(self.line_1g)
            && self.line_4
                == self
                    .nonbusiness_capital_losses
                    .saturating_sub(self.nonbusiness_capital_gains)
            && self.line_5
                == self
                    .nonbusiness_capital_gains
                    .saturating_sub(self.nonbusiness_capital_losses)
            && self.line_8 == self.line_5 + self.nonbusiness_income_other_than_capital_gains
            && self.line_9 == self.nonbusiness_deductions.saturating_sub(self.line_8)
            && self.line_13 == self.line_10 + self.business_capital_gains
            && self.line_14
                == self
                    .business_capital_losses_before_limitations
                    .saturating_sub(self.line_13)
            && self.line_15 == self.line_4 + self.line_14
            && self.line_16c
                == self.net_short_term_capital_gain_loss + self.net_long_term_capital_gain_loss
            && self.nc_nol
                == self.line_1
                    + self.line_9
                    + self.irc_1202_exclusion
                    + self.line_21
                    + self.line_22
                    + self.nc_nol_deduction_prior_years
    }

    /// Compute Part 1 of the NC-NOL worksheet.
    pub fn compute(input: &NcNolPart1Input) -> Self {
        let mut n = NcNolPart1::default();

        n.name = input.name.clone();
        n.federal_tax_id = input.federal_tax_id.clone();

        // Lines 1a–1g, Line 1
        n.federal_agi = input.federal_agi;
        n.additions_to_federal_agi = input.additions_to_federal_agi;
        n.line_1c = n.federal_agi + n.additions_to_federal_agi;

        n.deductions_from_federal_agi = input.deductions_from_federal_agi;
        n.nc_standard_or_itemized_deduction = input.nc_standard_or_itemized_deduction;
        n.excess_business_loss_other_income = input.excess_business_loss_other_income;
        n.line_1g = n.deductions_from_federal_agi
            + n.nc_standard_or_itemized_deduction
            + n.excess_business_loss_other_income;

        n.line_1 = n.line_1c.saturating_sub(n.line_1g);

        // Lines 2–5
        n.nonbusiness_capital_losses = input.nonbusiness_capital_losses;
        n.nonbusiness_capital_gains = input.nonbusiness_capital_gains;
        n.line_4 = n
            .nonbusiness_capital_losses
            .saturating_sub(n.nonbusiness_capital_gains);
        n.line_5 = n
            .nonbusiness_capital_gains
            .saturating_sub(n.nonbusiness_capital_losses);

        // Lines 6–10
        n.nonbusiness_deductions = input.nonbusiness_deductions;
        n.nonbusiness_income_other_than_capital_gains =
            input.nonbusiness_income_other_than_capital_gains;
        n.line_8 = n.line_5 + n.nonbusiness_income_other_than_capital_gains;
        n.line_9 = n.nonbusiness_deductions.saturating_sub(n.line_8);
        n.line_10 = if n.line_8 > n.nonbusiness_deductions {
            (n.line_8 - n.nonbusiness_deductions).min(n.line_5)
        } else {
            0
        };

        // Lines 11–15
        n.business_capital_losses_before_limitations =
            input.business_capital_losses_before_limitations;
        n.business_capital_gains = input.business_capital_gains;
        n.line_13 = n.line_10 + n.business_capital_gains;
        n.line_14 = n
            .business_capital_losses_before_limitations
            .saturating_sub(n.line_13);
        n.line_15 = n.line_4 + n.line_14;

        // Lines 16–21
        n.net_short_term_capital_gain_loss = input.net_short_term_capital_gain_loss;
        n.net_long_term_capital_gain_loss = input.net_long_term_capital_gain_loss;
        n.line_16c = n.net_short_term_capital_gain_loss + n.net_long_term_capital_gain_loss;

        if input.skip_lines_16_through_21 {
            // Skip lines 16–21; Line 22 = Line 15
            n.line_22 = n.line_15;
        } else {
            // Line 16: absolute value of line 16c
            n.line_16 = n.line_16c;
            n.irc_1202_exclusion = input.irc_1202_exclusion;
            n.line_18 = n.line_16.saturating_sub(n.irc_1202_exclusion);
            let cap = if input.filing_status == FilingStatus::MarriedFilingSeparately {
                CAPITAL_LOSS_LIMIT_MFS
            } else {
                CAPITAL_LOSS_LIMIT
            };
            n.line_19 = n.line_16.min(cap);
            n.line_20 = n.line_18.saturating_sub(n.line_19);
            n.line_21 = n.line_19.saturating_sub(n.line_18);
            n.line_22 = n.line_15.saturating_sub(n.line_20);
        }

        // Line 23
        n.nc_nol_deduction_prior_years = input.nc_nol_deduction_prior_years;

        // Line 24: Add Lines 1, 9, 17, 21, 22, and 23
        n.nc_nol = n.line_1
            + n.line_9
            + n.irc_1202_exclusion
            + n.line_21
            + n.line_22
            + n.nc_nol_deduction_prior_years;

        n
    }
}

impl NcNolPart2 {
    pub fn is_valid(&self) -> bool {
        let federal_remaining_ok = (0..12).all(|i| {
            self.federal_nol_carryover_remaining[i]
                == self.federal_nol_carryover[i]
                    .col_a
                    .saturating_sub(self.federal_nol_carryover[i].col_b)
        });

        let federal_deduction_ok = self.federal_nol_carryover_deduction
            == self.federal_nol_carryover.iter().map(|r| r.col_b).sum::<u64>();

        let nc_remaining_ok = (0..3).all(|i| {
            self.nc_nol_carryover_remaining[i]
                == self.nc_nol_carryover[i]
                    .col_a
                    .saturating_sub(self.nc_nol_carryover[i].col_b)
        });

        let nc_deduction_ok = self.nc_nol_carryover_deduction
            == self.nc_nol_carryover.iter().map(|r| r.col_b).sum::<u64>();

        federal_remaining_ok
            && federal_deduction_ok
            && nc_remaining_ok
            && nc_deduction_ok
            && self.line_18 == self.federal_nol_carryover_deduction
            && self.line_19 == self.nc_nol_carryover_deduction
            && self.total_nc_nol_deduction == self.line_18 + self.line_19
    }

    /// Compute Part 2 of the NC-NOL worksheet.
    #[allow(clippy::field_reassign_with_default)]
    pub fn compute(input: &NcNolPart2Input) -> Self {
        let mut p2 = NcNolPart2::default();

        // Part 2A: Federal NOL Carryover
        p2.federal_nol_carryover = input.federal_nol_carryover;
        for i in 0..12 {
            p2.federal_nol_carryover_remaining[i] =
                input.federal_nol_carryover[i].col_a - input.federal_nol_carryover[i].col_b;
        }
        // Line 13: Sum of Column B
        p2.federal_nol_carryover_deduction =
            input.federal_nol_carryover.iter().map(|r| r.col_b).sum();

        // Part 2B: N.C. NOL Carryover
        p2.nc_nol_carryover = input.nc_nol_carryover;
        for i in 0..3 {
            p2.nc_nol_carryover_remaining[i] =
                input.nc_nol_carryover[i].col_a - input.nc_nol_carryover[i].col_b;
        }
        // Line 17: Sum of Column B
        p2.nc_nol_carryover_deduction = input.nc_nol_carryover.iter().map(|r| r.col_b).sum();

        // Part 2C
        p2.line_18 = p2.federal_nol_carryover_deduction;
        p2.line_19 = p2.nc_nol_carryover_deduction;
        p2.total_nc_nol_deduction = p2.line_18 + p2.line_19;

        p2
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn empty_input(filing_status: FilingStatus) -> NcNolPart1Input {
        NcNolPart1Input {
            name: String::new(),
            federal_tax_id: String::new(),
            filing_status,
            federal_agi: 0,
            additions_to_federal_agi: 0,
            deductions_from_federal_agi: 0,
            nc_standard_or_itemized_deduction: 0,
            excess_business_loss_other_income: 0,
            nonbusiness_capital_losses: 0,
            nonbusiness_capital_gains: 0,
            nonbusiness_deductions: 0,
            nonbusiness_income_other_than_capital_gains: 0,
            business_capital_losses_before_limitations: 0,
            business_capital_gains: 0,
            net_short_term_capital_gain_loss: 0,
            net_long_term_capital_gain_loss: 0,
            irc_1202_exclusion: 0,
            nc_nol_deduction_prior_years: 0,
            skip_lines_16_through_21: false,
        }
    }

    #[test]
    fn part2_total_deduction_sums_both_parts() {
        let mut input = NcNolPart2Input::default();
        input.federal_nol_carryover[0] = FederalNolCarryoverRow {
            col_a: 10_000,
            col_b: 5_000,
        };
        input.nc_nol_carryover[0] = NcNolCarryoverRow {
            col_a: 3_000,
            col_b: 2_000,
        };
        let p2 = NcNolPart2::compute(&input);
        assert_eq!(p2.federal_nol_carryover_deduction, 5_000);
        assert_eq!(p2.nc_nol_carryover_deduction, 2_000);
        assert_eq!(p2.total_nc_nol_deduction, 7_000);
        assert_eq!(p2.federal_nol_carryover_remaining[0], 5_000);
        assert_eq!(p2.nc_nol_carryover_remaining[0], 1_000);
    }

    #[test]
    fn part1_basic_nol_calculation() {
        let mut input = empty_input(FilingStatus::Single);
        input.additions_to_federal_agi = 1_000;
        input.deductions_from_federal_agi = 2_000;
        input.nc_standard_or_itemized_deduction = 12_750;
        input.skip_lines_16_through_21 = true;
        let p1 = NcNolPart1::compute(&input);
        assert_eq!(p1.line_1c, 1_000);
        assert_eq!(p1.line_1g, 14_750);
        assert_eq!(p1.line_1, 0);
    }

    #[test]
    fn part1_lines_4_5_correct() {
        let mut input = empty_input(FilingStatus::Single);
        input.nonbusiness_capital_losses = 5_000;
        input.nonbusiness_capital_gains = 3_000;
        input.skip_lines_16_through_21 = true;
        let p1 = NcNolPart1::compute(&input);
        assert_eq!(p1.line_4, 2_000);
        assert_eq!(p1.line_5, 0);
    }

    #[test]
    fn default_part1_all_zeros() {
        let p1 = NcNolPart1::compute(&empty_input(FilingStatus::Single));
        assert_eq!(p1.line_1, 0);
        assert_eq!(p1.line_15, 0);
        assert_eq!(p1.nc_nol, 0);
    }

    #[test]
    fn line_19_uses_3000_for_single() {
        let mut input = empty_input(FilingStatus::Single);
        input.net_short_term_capital_gain_loss = 5_000;
        let p1 = NcNolPart1::compute(&input);
        assert_eq!(p1.line_19, 3_000);
    }

    #[test]
    fn line_19_uses_1500_for_mfs() {
        let mut input = empty_input(FilingStatus::MarriedFilingSeparately);
        input.net_short_term_capital_gain_loss = 5_000;
        let p1 = NcNolPart1::compute(&input);
        assert_eq!(p1.line_19, 1_500);
    }

    #[test]
    fn line_19_capped_at_line_16() {
        let mut input = empty_input(FilingStatus::Single);
        input.net_short_term_capital_gain_loss = 2_000;
        let p1 = NcNolPart1::compute(&input);
        assert_eq!(p1.line_19, 2_000);
    }

    #[test]
    fn computed_nc_nol_part1_is_valid() {
        let mut input = empty_input(FilingStatus::Single);
        input.federal_agi = 50_000;
        input.additions_to_federal_agi = 1_000;
        input.deductions_from_federal_agi = 2_000;
        input.nc_standard_or_itemized_deduction = 12_750;
        input.nonbusiness_capital_losses = 5_000;
        input.nonbusiness_capital_gains = 3_000;
        input.net_short_term_capital_gain_loss = 4_000;
        let p1 = NcNolPart1::compute(&input);
        assert!(p1.is_valid());
    }

    #[test]
    fn inconsistent_nc_nol_part1_is_invalid() {
        let mut p1 = NcNolPart1::compute(&empty_input(FilingStatus::Single));
        p1.line_1c = 999;
        assert!(!p1.is_valid());
    }

    #[test]
    fn computed_nc_nol_part2_is_valid() {
        let mut input = NcNolPart2Input::default();
        input.federal_nol_carryover[0] = FederalNolCarryoverRow {
            col_a: 10_000,
            col_b: 5_000,
        };
        input.nc_nol_carryover[0] = NcNolCarryoverRow {
            col_a: 3_000,
            col_b: 2_000,
        };
        let p2 = NcNolPart2::compute(&input);
        assert!(p2.is_valid());
    }

    #[test]
    fn inconsistent_nc_nol_part2_is_invalid() {
        let mut p2 = NcNolPart2::compute(&NcNolPart2Input::default());
        p2.total_nc_nol_deduction = 999;
        assert!(!p2.is_valid());
    }
}
