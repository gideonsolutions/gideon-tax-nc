use crate::constants::{
    MAX_CREDIT_INCOME_PRODUCING, MAX_EXPENDITURES_INCOME_PRODUCING,
    MAX_EXPENSES_NONINCOME_PRODUCING, TIER_THRESHOLD,
};
use crate::tax_form_type::TaxFormType;

/// Type of historic structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HistoricStructureType {
    #[default]
    IncomeProducing,
    NonincomeProducing,
}

/// Two-column value for Franchise and Income.
#[derive(Debug, Clone, Copy, Default)]
pub struct FranchiseIncome {
    pub franchise: u64,
    pub income: u64,
}

/// Manual inputs for NC-Rehab.
#[derive(Debug, Clone, Default)]
pub struct NcRehabInput {
    // Header
    pub first_name: String,
    pub middle_initial: String,
    pub last_name: String,
    pub ssn: String,
    pub entity_legal_name: String,
    pub fein: String,
    pub tax_year_beginning: String,
    pub tax_year_ending: String,
    pub amended_return: bool,
    pub tax_form_type: TaxFormType,

    // Part 1: Qualifying Information
    pub federal_income_tax_credit_section_47: bool,
    pub historic_structure_type: HistoricStructureType,
    pub address_and_county: String,

    // Part 2: Income-Producing (Lines 1, 8, 10, 12)
    /// Line 1: Total qualified rehabilitation expenditures
    pub total_qualified_rehabilitation_expenditures: u64,
    /// Line 8: Amount of Line 3 in a development tier 1 or tier 2 area
    pub amount_in_tier_1_or_2: u64,
    /// Line 10: Amount of Line 3 on an eligible targeted investment site
    pub amount_on_targeted_investment_site: u64,
    /// Line 12: Amount of Line 3 used for educational purpose
    pub amount_for_educational_purpose: u64,

    // Part 3: Nonincome-Producing (Line 17)
    /// Line 17: Total rehabilitation expenses per discrete property parcel
    pub total_rehabilitation_expenses_nonincome: u64,

    // Part 4: Carryforwards
    /// Line 22: Carryforwards (income-producing) — Franchise and Income
    pub carryforwards_income_producing: FranchiseIncome,
    /// Line 25: Carryforwards (nonincome-producing) — Franchise and Income
    pub carryforwards_nonincome_producing: FranchiseIncome,
}

/// NC-Rehab — Historic Rehabilitation Tax Credits.
#[derive(Debug, Clone, Default)]
pub struct NcRehab {
    // Header
    pub first_name: String,
    pub middle_initial: String,
    pub last_name: String,
    pub ssn: String,
    pub entity_legal_name: String,
    pub fein: String,
    pub tax_year_beginning: String,
    pub tax_year_ending: String,
    pub amended_return: bool,
    pub tax_form_type: TaxFormType,

    // Part 1
    pub federal_income_tax_credit_section_47: bool,
    pub historic_structure_type: HistoricStructureType,
    pub address_and_county: String,

    // Part 2: Income-Producing Historic Structure
    /// Line 1: Total qualified rehabilitation expenditures
    pub line_1_total_expenditures: u64,
    /// Line 2: Maximum expenditures ($20,000,000)
    pub line_2_max_expenditures: u64,
    /// Line 3: Lesser of Line 1 or Line 2
    pub line_3_qualifying_amount: u64,
    /// Line 4: Amount of Line 3 up to $10 million
    pub line_4_up_to_10m: u64,
    /// Line 5: Line 4 × 15%
    pub line_5_fifteen_pct: u64,
    /// Line 6: Line 3 − Line 4
    pub line_6_remainder: u64,
    /// Line 7: Line 6 × 10%
    pub line_7_ten_pct: u64,
    /// Line 8: Amount in development tier 1 or 2 area
    pub line_8_tier_1_2_amount: u64,
    /// Line 9: Development tier bonus (Line 8 × 5%)
    pub line_9_tier_bonus: u64,
    /// Line 10: Amount on targeted investment site
    pub line_10_targeted_investment: u64,
    /// Line 11: Targeted investment bonus (Line 10 × 5%)
    pub line_11_targeted_bonus: u64,
    /// Line 12: Amount for educational purpose
    pub line_12_educational_amount: u64,
    /// Line 13: Education bonus (Line 12 × 5%)
    pub line_13_education_bonus: u64,
    /// Line 14: Add Lines 5, 7, 9, 11, and 13
    pub line_14_total_credit: u64,
    /// Line 15: Maximum credit ($4,500,000)
    pub line_15_max_credit: u64,
    /// Line 16: Credit — lesser of Line 14 or Line 15
    pub line_16_income_producing_credit: u64,

    // Part 3: Nonincome-Producing Historic Structure
    /// Line 17: Total rehabilitation expenses per discrete property parcel
    pub line_17_total_expenses: u64,
    /// Line 18: Maximum expenses ($150,000)
    pub line_18_max_expenses: u64,
    /// Line 19: Lesser of Line 17 or Line 18
    pub line_19_qualifying_amount: u64,
    /// Line 20: Credit (Line 19 × 15%)
    pub line_20_nonincome_producing_credit: u64,

    // Part 4: Computation of Amount To Be Taken
    /// Line 21: Income-producing credit for 2025 (from Part 2 Line 16)
    pub line_21_income_producing: FranchiseIncome,
    /// Line 22: Carryforwards (income-producing)
    pub line_22_carryforwards_income: FranchiseIncome,
    /// Line 23: Total (Lines 21 + 22)
    pub line_23_total_income_producing: FranchiseIncome,
    /// Line 24: Nonincome-producing credit for 2025 (from Part 3 Line 20)
    pub line_24_nonincome_producing: FranchiseIncome,
    /// Line 25: Carryforwards (nonincome-producing)
    pub line_25_carryforwards_nonincome: FranchiseIncome,
    /// Line 26: Total (Lines 24 + 25)
    pub line_26_total_nonincome_producing: FranchiseIncome,
}

impl NcRehab {
    pub fn is_valid(&self) -> bool {
        self.line_3_qualifying_amount
            == self
                .line_1_total_expenditures
                .min(self.line_2_max_expenditures)
            && self.line_4_up_to_10m == self.line_3_qualifying_amount.min(TIER_THRESHOLD)
            && self.line_5_fifteen_pct == (self.line_4_up_to_10m * 15 + 50) / 100
            && self.line_6_remainder == self.line_3_qualifying_amount - self.line_4_up_to_10m
            && self.line_7_ten_pct == (self.line_6_remainder * 10 + 50) / 100
            && self.line_9_tier_bonus == (self.line_8_tier_1_2_amount * 5 + 50) / 100
            && self.line_11_targeted_bonus == (self.line_10_targeted_investment * 5 + 50) / 100
            && self.line_13_education_bonus == (self.line_12_educational_amount * 5 + 50) / 100
            && self.line_14_total_credit
                == self.line_5_fifteen_pct
                    + self.line_7_ten_pct
                    + self.line_9_tier_bonus
                    + self.line_11_targeted_bonus
                    + self.line_13_education_bonus
            && self.line_16_income_producing_credit
                == self.line_14_total_credit.min(self.line_15_max_credit)
            && self.line_19_qualifying_amount
                == self.line_17_total_expenses.min(self.line_18_max_expenses)
            && self.line_20_nonincome_producing_credit
                == (self.line_19_qualifying_amount * 15 + 50) / 100
            && self.line_23_total_income_producing.franchise
                == self.line_21_income_producing.franchise
                    + self.line_22_carryforwards_income.franchise
            && self.line_23_total_income_producing.income
                == self.line_21_income_producing.income + self.line_22_carryforwards_income.income
            && self.line_26_total_nonincome_producing.franchise
                == self.line_24_nonincome_producing.franchise
                    + self.line_25_carryforwards_nonincome.franchise
            && self.line_26_total_nonincome_producing.income
                == self.line_24_nonincome_producing.income
                    + self.line_25_carryforwards_nonincome.income
    }

    /// Compute NC-Rehab from inputs.
    pub fn compute(input: &NcRehabInput) -> Self {
        let mut r = NcRehab::default();

        // Header
        r.first_name = input.first_name.clone();
        r.middle_initial = input.middle_initial.clone();
        r.last_name = input.last_name.clone();
        r.ssn = input.ssn.clone();
        r.entity_legal_name = input.entity_legal_name.clone();
        r.fein = input.fein.clone();
        r.tax_year_beginning = input.tax_year_beginning.clone();
        r.tax_year_ending = input.tax_year_ending.clone();
        r.amended_return = input.amended_return;
        r.tax_form_type = input.tax_form_type;

        // Part 1
        r.federal_income_tax_credit_section_47 = input.federal_income_tax_credit_section_47;
        r.historic_structure_type = input.historic_structure_type;
        r.address_and_county = input.address_and_county.clone();

        // Part 2: Income-Producing
        r.line_1_total_expenditures = input.total_qualified_rehabilitation_expenditures;
        r.line_2_max_expenditures = MAX_EXPENDITURES_INCOME_PRODUCING;
        r.line_3_qualifying_amount = r.line_1_total_expenditures.min(r.line_2_max_expenditures);
        r.line_4_up_to_10m = r.line_3_qualifying_amount.min(TIER_THRESHOLD);
        // Line 5: 15% of Line 4
        r.line_5_fifteen_pct = (r.line_4_up_to_10m * 15 + 50) / 100;
        // Line 6: Line 3 − Line 4, always non-negative by construction
        r.line_6_remainder = r.line_3_qualifying_amount - r.line_4_up_to_10m;
        // Line 7: 10% of Line 6
        r.line_7_ten_pct = (r.line_6_remainder * 10 + 50) / 100;

        // Lines 8–13: bonus credits (5% each)
        r.line_8_tier_1_2_amount = input.amount_in_tier_1_or_2;
        r.line_9_tier_bonus = (r.line_8_tier_1_2_amount * 5 + 50) / 100;
        r.line_10_targeted_investment = input.amount_on_targeted_investment_site;
        r.line_11_targeted_bonus = (r.line_10_targeted_investment * 5 + 50) / 100;
        r.line_12_educational_amount = input.amount_for_educational_purpose;
        r.line_13_education_bonus = (r.line_12_educational_amount * 5 + 50) / 100;

        // Line 14: Sum of Lines 5, 7, 9, 11, 13
        r.line_14_total_credit = [
            r.line_5_fifteen_pct,
            r.line_7_ten_pct,
            r.line_9_tier_bonus,
            r.line_11_targeted_bonus,
            r.line_13_education_bonus,
        ]
        .into_iter()
        .sum();

        // Line 15/16
        r.line_15_max_credit = MAX_CREDIT_INCOME_PRODUCING;
        r.line_16_income_producing_credit = r.line_14_total_credit.min(r.line_15_max_credit);

        // Part 3: Nonincome-Producing
        r.line_17_total_expenses = input.total_rehabilitation_expenses_nonincome;
        r.line_18_max_expenses = MAX_EXPENSES_NONINCOME_PRODUCING;
        r.line_19_qualifying_amount = r.line_17_total_expenses.min(r.line_18_max_expenses);
        // Line 20: 15% of Line 19
        r.line_20_nonincome_producing_credit = (r.line_19_qualifying_amount * 15 + 50) / 100;

        // Part 4: Franchise/Income split
        // Line 21
        r.line_21_income_producing = FranchiseIncome {
            franchise: r.line_16_income_producing_credit,
            income: r.line_16_income_producing_credit,
        };
        // Line 22
        r.line_22_carryforwards_income = input.carryforwards_income_producing;
        // Line 23
        r.line_23_total_income_producing = FranchiseIncome {
            franchise: r.line_21_income_producing.franchise
                + r.line_22_carryforwards_income.franchise,
            income: r.line_21_income_producing.income + r.line_22_carryforwards_income.income,
        };

        // Line 24
        r.line_24_nonincome_producing = FranchiseIncome {
            franchise: r.line_20_nonincome_producing_credit,
            income: r.line_20_nonincome_producing_credit,
        };
        // Line 25
        r.line_25_carryforwards_nonincome = input.carryforwards_nonincome_producing;
        // Line 26
        r.line_26_total_nonincome_producing = FranchiseIncome {
            franchise: r.line_24_nonincome_producing.franchise
                + r.line_25_carryforwards_nonincome.franchise,
            income: r.line_24_nonincome_producing.income + r.line_25_carryforwards_nonincome.income,
        };

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn income_producing_under_10m() {
        let input = NcRehabInput {
            total_qualified_rehabilitation_expenditures: 5_000_000,
            ..Default::default()
        };
        let r = NcRehab::compute(&input);
        assert_eq!(r.line_5_fifteen_pct, 750_000);
        assert_eq!(r.line_6_remainder, 0);
        assert_eq!(r.line_14_total_credit, 750_000);
        assert_eq!(r.line_16_income_producing_credit, 750_000);
    }

    #[test]
    fn income_producing_over_10m() {
        let input = NcRehabInput {
            total_qualified_rehabilitation_expenditures: 15_000_000,
            ..Default::default()
        };
        let r = NcRehab::compute(&input);
        assert_eq!(r.line_5_fifteen_pct, 1_500_000);
        assert_eq!(r.line_7_ten_pct, 500_000);
        assert_eq!(r.line_14_total_credit, 2_000_000);
    }

    #[test]
    fn income_producing_capped_at_max() {
        let input = NcRehabInput {
            total_qualified_rehabilitation_expenditures: 20_000_000,
            amount_in_tier_1_or_2: 20_000_000,
            amount_on_targeted_investment_site: 20_000_000,
            amount_for_educational_purpose: 20_000_000,
            ..Default::default()
        };
        let r = NcRehab::compute(&input);
        assert_eq!(r.line_16_income_producing_credit, 4_500_000);
    }

    #[test]
    fn nonincome_producing_basic() {
        let input = NcRehabInput {
            total_rehabilitation_expenses_nonincome: 100_000,
            ..Default::default()
        };
        let r = NcRehab::compute(&input);
        assert_eq!(r.line_20_nonincome_producing_credit, 15_000);
    }

    #[test]
    fn nonincome_producing_capped_at_150k() {
        let input = NcRehabInput {
            total_rehabilitation_expenses_nonincome: 200_000,
            ..Default::default()
        };
        let r = NcRehab::compute(&input);
        assert_eq!(r.line_20_nonincome_producing_credit, 22_500);
    }

    #[test]
    fn computed_nc_rehab_is_valid() {
        let input = NcRehabInput {
            total_qualified_rehabilitation_expenditures: 15_000_000,
            amount_in_tier_1_or_2: 5_000_000,
            amount_on_targeted_investment_site: 3_000_000,
            amount_for_educational_purpose: 2_000_000,
            total_rehabilitation_expenses_nonincome: 100_000,
            carryforwards_income_producing: FranchiseIncome {
                franchise: 50_000,
                income: 25_000,
            },
            ..Default::default()
        };
        let r = NcRehab::compute(&input);
        assert!(r.is_valid());
    }

    #[test]
    fn inconsistent_nc_rehab_is_invalid() {
        let mut r = NcRehab::compute(&NcRehabInput::default());
        r.line_3_qualifying_amount = 999;
        assert!(!r.is_valid());
    }

    #[test]
    fn part4_totals_include_carryforwards() {
        let input = NcRehabInput {
            total_qualified_rehabilitation_expenditures: 5_000_000,
            carryforwards_income_producing: FranchiseIncome {
                franchise: 100_000,
                income: 50_000,
            },
            total_rehabilitation_expenses_nonincome: 100_000,
            carryforwards_nonincome_producing: FranchiseIncome {
                franchise: 10_000,
                income: 5_000,
            },
            ..Default::default()
        };
        let r = NcRehab::compute(&input);
        assert_eq!(
            r.line_23_total_income_producing.franchise,
            750_000 + 100_000
        );
        assert_eq!(r.line_23_total_income_producing.income, 750_000 + 50_000);
        assert_eq!(
            r.line_26_total_nonincome_producing.franchise,
            15_000 + 10_000
        );
    }
}
