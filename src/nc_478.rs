use crate::nc_rehab::FranchiseIncome;
use crate::tax_form_type::TaxFormType;

/// Other tax credit sub-types for Line 8/22.
#[derive(Debug, Clone, Copy, Default)]
pub struct OtherTaxCredits {
    // Limited to corporations
    pub cigarettes_for_export: bool,
    // Limited to corporations
    pub cigarettes_for_export_increasing_employment: bool,
    pub substantial_investment: bool,
}

/// Manual inputs for NC-478.
#[derive(Debug, Clone, Default)]
pub struct Nc478Input {
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

    // Part 1: Tax Credits Subject to 50% Limit (Lines 1–8)
    /// Line 1: N.C. Research and Development
    pub nc_research_and_development: FranchiseIncome,
    /// Line 2: Investing in Renewable Energy Property
    pub investing_renewable_energy: FranchiseIncome,
    /// Line 3: Technology Commercialization
    pub technology_commercialization: FranchiseIncome,
    /// Line 4: Renewable Fuel Facility
    pub renewable_fuel_facility: FranchiseIncome,
    /// Line 5: Constructing a Railroad Intermodal Facility
    pub constructing_railroad_intermodal: FranchiseIncome,
    /// Line 6: Investing in Real Property
    pub investing_real_property: FranchiseIncome,
    /// Line 7: Interactive Digital Media
    pub interactive_digital_media: FranchiseIncome,
    /// Line 8: Other Tax Credits Subject to 50% Limit
    pub other_tax_credits: FranchiseIncome,
    /// Line 8 sub-types
    pub other_tax_credit_types: OtherTaxCredits,

    // Part 2: Computation of 50% Limit
    /// Line 10: Amount of Franchise and Income Tax Due
    pub franchise_and_income_tax_due: FranchiseIncome,
    /// Line 11: Amount of Credits Not Subject to 50% Limit
    pub credits_not_subject_to_50_pct: FranchiseIncome,

    // Part 3: Amount of Each Credit Taken (Lines 15–22)
    /// Line 15: N.C. Research and Development taken
    pub taken_nc_research_and_development: FranchiseIncome,
    /// Line 16: Investing in Renewable Energy Property taken
    pub taken_investing_renewable_energy: FranchiseIncome,
    /// Line 17: Technology Commercialization taken
    pub taken_technology_commercialization: FranchiseIncome,
    /// Line 18: Renewable Fuel Facility taken
    pub taken_renewable_fuel_facility: FranchiseIncome,
    /// Line 19: Constructing a Railroad Intermodal Facility taken
    pub taken_constructing_railroad_intermodal: FranchiseIncome,
    /// Line 20: Investing in Real Property taken
    pub taken_investing_real_property: FranchiseIncome,
    /// Line 21: Interactive Digital Media taken
    pub taken_interactive_digital_media: FranchiseIncome,
    /// Line 22: Other Tax Credits taken
    pub taken_other_tax_credits: FranchiseIncome,
    /// Line 22 sub-types
    pub taken_other_tax_credit_types: OtherTaxCredits,
}

/// NC-478 — Summary of Tax Credits Limited to 50% of Tax.
#[derive(Debug, Clone, Default)]
pub struct Nc478 {
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

    // Part 1: Tax Credits (Lines 1–9)
    /// Line 1
    pub nc_research_and_development: FranchiseIncome,
    /// Line 2
    pub investing_renewable_energy: FranchiseIncome,
    /// Line 3
    pub technology_commercialization: FranchiseIncome,
    /// Line 4
    pub renewable_fuel_facility: FranchiseIncome,
    /// Line 5
    pub constructing_railroad_intermodal: FranchiseIncome,
    /// Line 6
    pub investing_real_property: FranchiseIncome,
    /// Line 7
    pub interactive_digital_media: FranchiseIncome,
    /// Line 8
    pub other_tax_credits: FranchiseIncome,
    pub other_tax_credit_types: OtherTaxCredits,
    /// Line 9: Total Tax Credits Subject to 50% Limit (sum of Lines 1–8)
    pub total_tax_credits_subject_to_50_pct: FranchiseIncome,

    // Part 2: Computation of 50% Limit (Lines 10–14)
    /// Line 10: Amount of Franchise and Income Tax Due
    pub franchise_and_income_tax_due: FranchiseIncome,
    /// Line 11: Credits Not Subject to 50% Limit
    pub credits_not_subject_to_50_pct: FranchiseIncome,
    /// Line 12: Line 10 − Line 11 (zero floor)
    pub line_12_net: FranchiseIncome,
    /// Line 13: Line 12 × 50%
    pub line_13_fifty_pct: FranchiseIncome,
    /// Line 14: Lesser of Line 9 or Line 13
    pub line_14_credit_allowed: FranchiseIncome,

    // Part 3: Amount of Each Credit Taken (Lines 15–23)
    /// Line 15
    pub taken_nc_research_and_development: FranchiseIncome,
    /// Line 16
    pub taken_investing_renewable_energy: FranchiseIncome,
    /// Line 17
    pub taken_technology_commercialization: FranchiseIncome,
    /// Line 18
    pub taken_renewable_fuel_facility: FranchiseIncome,
    /// Line 19
    pub taken_constructing_railroad_intermodal: FranchiseIncome,
    /// Line 20
    pub taken_investing_real_property: FranchiseIncome,
    /// Line 21
    pub taken_interactive_digital_media: FranchiseIncome,
    /// Line 22
    pub taken_other_tax_credits: FranchiseIncome,
    pub taken_other_tax_credit_types: OtherTaxCredits,
    /// Line 23: Total Tax Credits Subject to 50% Limit (sum of Lines 15–22, cannot exceed Line 14)
    pub total_credits_taken: FranchiseIncome,
}

/// Helper to sum FranchiseIncome values.
fn sum_fi(items: &[FranchiseIncome]) -> FranchiseIncome {
    FranchiseIncome {
        franchise: items.iter().map(|fi| fi.franchise).sum(),
        income: items.iter().map(|fi| fi.income).sum(),
    }
}

impl Nc478 {
    pub fn is_valid(&self) -> bool {
        let credit_lines = [
            self.nc_research_and_development,
            self.investing_renewable_energy,
            self.technology_commercialization,
            self.renewable_fuel_facility,
            self.constructing_railroad_intermodal,
            self.investing_real_property,
            self.interactive_digital_media,
            self.other_tax_credits,
        ];
        let expected_total = sum_fi(&credit_lines);

        let expected_line_12 = FranchiseIncome {
            franchise: self
                .franchise_and_income_tax_due
                .franchise
                .saturating_sub(self.credits_not_subject_to_50_pct.franchise),
            income: self
                .franchise_and_income_tax_due
                .income
                .saturating_sub(self.credits_not_subject_to_50_pct.income),
        };

        let expected_line_13 = FranchiseIncome {
            franchise: self.line_12_net.franchise / 2,
            income: self.line_12_net.income / 2,
        };

        let expected_line_14 = FranchiseIncome {
            franchise: self
                .total_tax_credits_subject_to_50_pct
                .franchise
                .min(self.line_13_fifty_pct.franchise),
            income: self
                .total_tax_credits_subject_to_50_pct
                .income
                .min(self.line_13_fifty_pct.income),
        };

        let taken_lines = [
            self.taken_nc_research_and_development,
            self.taken_investing_renewable_energy,
            self.taken_technology_commercialization,
            self.taken_renewable_fuel_facility,
            self.taken_constructing_railroad_intermodal,
            self.taken_investing_real_property,
            self.taken_interactive_digital_media,
            self.taken_other_tax_credits,
        ];
        let raw_taken = sum_fi(&taken_lines);
        let expected_taken = FranchiseIncome {
            franchise: raw_taken
                .franchise
                .min(self.line_14_credit_allowed.franchise),
            income: raw_taken.income.min(self.line_14_credit_allowed.income),
        };

        self.total_tax_credits_subject_to_50_pct.franchise == expected_total.franchise
            && self.total_tax_credits_subject_to_50_pct.income == expected_total.income
            && self.line_12_net.franchise == expected_line_12.franchise
            && self.line_12_net.income == expected_line_12.income
            && self.line_13_fifty_pct.franchise == expected_line_13.franchise
            && self.line_13_fifty_pct.income == expected_line_13.income
            && self.line_14_credit_allowed.franchise == expected_line_14.franchise
            && self.line_14_credit_allowed.income == expected_line_14.income
            && self.total_credits_taken.franchise == expected_taken.franchise
            && self.total_credits_taken.income == expected_taken.income
            && self.total_credits_taken.franchise <= self.line_14_credit_allowed.franchise
            && self.total_credits_taken.income <= self.line_14_credit_allowed.income
    }

    /// Compute NC-478 from inputs.
    pub fn compute(input: &Nc478Input) -> Self {
        let mut nc = Nc478::default();

        // Header
        nc.first_name = input.first_name.clone();
        nc.middle_initial = input.middle_initial.clone();
        nc.last_name = input.last_name.clone();
        nc.ssn = input.ssn.clone();
        nc.entity_legal_name = input.entity_legal_name.clone();
        nc.fein = input.fein.clone();
        nc.tax_year_beginning = input.tax_year_beginning.clone();
        nc.tax_year_ending = input.tax_year_ending.clone();
        nc.amended_return = input.amended_return;
        nc.tax_form_type = input.tax_form_type;

        // Part 1
        nc.nc_research_and_development = input.nc_research_and_development;
        nc.investing_renewable_energy = input.investing_renewable_energy;
        nc.technology_commercialization = input.technology_commercialization;
        nc.renewable_fuel_facility = input.renewable_fuel_facility;
        nc.constructing_railroad_intermodal = input.constructing_railroad_intermodal;
        nc.investing_real_property = input.investing_real_property;
        nc.interactive_digital_media = input.interactive_digital_media;
        nc.other_tax_credits = input.other_tax_credits;
        nc.other_tax_credit_types = input.other_tax_credit_types;

        // Line 9
        nc.total_tax_credits_subject_to_50_pct = sum_fi(&[
            nc.nc_research_and_development,
            nc.investing_renewable_energy,
            nc.technology_commercialization,
            nc.renewable_fuel_facility,
            nc.constructing_railroad_intermodal,
            nc.investing_real_property,
            nc.interactive_digital_media,
            nc.other_tax_credits,
        ]);

        // Part 2
        nc.franchise_and_income_tax_due = input.franchise_and_income_tax_due;
        nc.credits_not_subject_to_50_pct = input.credits_not_subject_to_50_pct;

        // Line 12: Line 10 − Line 11 (zero floor)
        nc.line_12_net = FranchiseIncome {
            franchise: nc
                .franchise_and_income_tax_due
                .franchise
                .saturating_sub(nc.credits_not_subject_to_50_pct.franchise),
            income: nc
                .franchise_and_income_tax_due
                .income
                .saturating_sub(nc.credits_not_subject_to_50_pct.income),
        };

        // Line 13: Line 12 × 50%
        nc.line_13_fifty_pct = FranchiseIncome {
            franchise: nc.line_12_net.franchise / 2,
            income: nc.line_12_net.income / 2,
        };

        // Line 14: min(Line 9, Line 13)
        nc.line_14_credit_allowed = FranchiseIncome {
            franchise: nc
                .total_tax_credits_subject_to_50_pct
                .franchise
                .min(nc.line_13_fifty_pct.franchise),
            income: nc
                .total_tax_credits_subject_to_50_pct
                .income
                .min(nc.line_13_fifty_pct.income),
        };

        // Part 3
        nc.taken_nc_research_and_development = input.taken_nc_research_and_development;
        nc.taken_investing_renewable_energy = input.taken_investing_renewable_energy;
        nc.taken_technology_commercialization = input.taken_technology_commercialization;
        nc.taken_renewable_fuel_facility = input.taken_renewable_fuel_facility;
        nc.taken_constructing_railroad_intermodal = input.taken_constructing_railroad_intermodal;
        nc.taken_investing_real_property = input.taken_investing_real_property;
        nc.taken_interactive_digital_media = input.taken_interactive_digital_media;
        nc.taken_other_tax_credits = input.taken_other_tax_credits;
        nc.taken_other_tax_credit_types = input.taken_other_tax_credit_types;

        // Line 23: sum of Lines 15–22, cannot exceed Line 14
        let raw_total = sum_fi(&[
            nc.taken_nc_research_and_development,
            nc.taken_investing_renewable_energy,
            nc.taken_technology_commercialization,
            nc.taken_renewable_fuel_facility,
            nc.taken_constructing_railroad_intermodal,
            nc.taken_investing_real_property,
            nc.taken_interactive_digital_media,
            nc.taken_other_tax_credits,
        ]);
        nc.total_credits_taken = FranchiseIncome {
            franchise: raw_total.franchise.min(nc.line_14_credit_allowed.franchise),
            income: raw_total.income.min(nc.line_14_credit_allowed.income),
        };

        nc
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn total_credits_subject_to_50_pct() {
        let input = Nc478Input {
            nc_research_and_development: FranchiseIncome {
                franchise: 1_000,
                income: 2_000,
            },
            investing_renewable_energy: FranchiseIncome {
                franchise: 500,
                income: 300,
            },
            ..Default::default()
        };
        let nc = Nc478::compute(&input);
        assert_eq!(nc.total_tax_credits_subject_to_50_pct.franchise, 1_500);
        assert_eq!(nc.total_tax_credits_subject_to_50_pct.income, 2_300);
    }

    #[test]
    fn fifty_pct_limit_computation() {
        let input = Nc478Input {
            nc_research_and_development: FranchiseIncome {
                franchise: 10_000,
                income: 10_000,
            },
            franchise_and_income_tax_due: FranchiseIncome {
                franchise: 8_000,
                income: 6_000,
            },
            credits_not_subject_to_50_pct: FranchiseIncome {
                franchise: 2_000,
                income: 1_000,
            },
            ..Default::default()
        };
        let nc = Nc478::compute(&input);
        // Line 12: 8000-2000=6000, 6000-1000=5000
        // Line 13: 3000, 2500
        // Line 14: min(10000, 3000)=3000, min(10000, 2500)=2500
        assert_eq!(nc.line_14_credit_allowed.franchise, 3_000);
        assert_eq!(nc.line_14_credit_allowed.income, 2_500);
    }

    #[test]
    fn computed_nc_478_is_valid() {
        let input = Nc478Input {
            nc_research_and_development: FranchiseIncome {
                franchise: 1_000,
                income: 2_000,
            },
            franchise_and_income_tax_due: FranchiseIncome {
                franchise: 10_000,
                income: 10_000,
            },
            taken_nc_research_and_development: FranchiseIncome {
                franchise: 1_000,
                income: 2_000,
            },
            ..Default::default()
        };
        let nc = Nc478::compute(&input);
        assert!(nc.is_valid());
    }

    #[test]
    fn inconsistent_nc_478_is_invalid() {
        let mut nc = Nc478::compute(&Nc478Input::default());
        nc.total_tax_credits_subject_to_50_pct.franchise = 999;
        assert!(!nc.is_valid());
    }

    #[test]
    fn credits_taken_cannot_exceed_line_14() {
        let input = Nc478Input {
            nc_research_and_development: FranchiseIncome {
                franchise: 5_000,
                income: 5_000,
            },
            franchise_and_income_tax_due: FranchiseIncome {
                franchise: 4_000,
                income: 4_000,
            },
            taken_nc_research_and_development: FranchiseIncome {
                franchise: 5_000,
                income: 5_000,
            },
            ..Default::default()
        };
        let nc = Nc478::compute(&input);
        // Line 14: min(5000, 2000) = 2000 each
        assert_eq!(nc.total_credits_taken.franchise, 2_000);
        assert_eq!(nc.total_credits_taken.income, 2_000);
    }
}
