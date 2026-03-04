/// Manual inputs for D-400 Schedule S (NC Adjustments).
#[derive(Debug, Clone, Default)]
pub struct ScheduleSInput {
    // Header
    pub name: String,
    pub ssn: String,

    // Part A: Additions to Federal AGI (Lines 1–15)
    /// Line 1: Interest income from obligations of states other than NC.
    pub interest_income_from_obligations_of_other_states: u64,
    /// Line 2: Deferred gains reinvested in a Qualified Opportunity Fund.
    pub deferred_gains_reinvested_opportunity_fund: u64,
    /// Line 3: Bonus depreciation added back.
    pub bonus_depreciation: u64,
    /// Line 4: IRC Section 179 expense deduction added back.
    pub irc_section_179_expense: u64,
    /// Line 5: S-corporation shareholder's share of built-in gains tax.
    pub s_corp_shareholder_built_in_gains_tax: u64,
    /// Line 6: Basis difference on disposed asset (federal exceeds state).
    pub federal_basis_exceeds_state_basis_disposed: u64,
    /// Line 7: Federal net operating loss deduction added back.
    pub federal_net_operating_loss_deduction: u64,
    /// Line 8: State/local/foreign income tax deducted by S-corp, partnership, or estate.
    pub state_local_foreign_income_tax_deducted_scorp_partnership_estate: u64,
    /// Line 9: Withdrawal from NC 529 plan that was not a permissible withdrawal.
    pub withdrawal_529_not_permissible: u64,
    /// Line 10: Discharge of qualified principal residence indebtedness excluded from federal.
    pub discharge_qualified_principal_residence_indebtedness: u64,
    /// Line 11: Qualified education loan payments made by employer excluded from federal.
    pub qualified_education_loan_payments_by_employer: u64,
    /// Line 12: Expenses allocable to income exempt from NC tax.
    pub expenses_allocable_exempt_income: u64,
    /// Line 13: Discharge of certain student loan debt excluded from federal income.
    pub discharge_certain_student_loan_debt: u64,
    /// Line 14: NC taxed pass-through entity loss adjustment.
    pub taxed_pass_through_entity_loss: u64,

    // Part B: Deductions from Federal AGI (Lines 17–40)
    /// Line 17: State or local income tax refund reported on federal return.
    pub state_or_local_income_tax_refund: u64,
    /// Line 18: Interest income from U.S. obligations.
    pub interest_income_us_obligations: u64,
    /// Line 19: Taxable portion of Social Security / Railroad Retirement benefits.
    pub taxable_social_security_railroad_retirement: u64,
    /// Line 20: Retirement benefits received under the Bailey settlement.
    pub retirement_benefits_bailey_settlement: u64,
    /// Line 21: Certain retirement benefits received by uniformed services members.
    pub certain_retirement_benefits_uniformed_services: u64,
    /// Line 22: Bonus asset basis adjustment.
    pub bonus_asset_basis: u64,
    /// Line 23a: Bonus depreciation (2020).
    pub bonus_depreciation_2020: u64,
    /// Line 23b: Bonus depreciation (2021).
    pub bonus_depreciation_2021: u64,
    /// Line 23c: Bonus depreciation (2022).
    pub bonus_depreciation_2022: u64,
    /// Line 23d: Bonus depreciation (2023).
    pub bonus_depreciation_2023: u64,
    /// Line 23e: Bonus depreciation (2024).
    pub bonus_depreciation_2024: u64,
    /// Line 24a: IRC Section 179 expense (2020).
    pub irc_179_expense_2020: u64,
    /// Line 24b: IRC Section 179 expense (2021).
    pub irc_179_expense_2021: u64,
    /// Line 24c: IRC Section 179 expense (2022).
    pub irc_179_expense_2022: u64,
    /// Line 24d: IRC Section 179 expense (2023).
    pub irc_179_expense_2023: u64,
    /// Line 24e: IRC Section 179 expense (2024).
    pub irc_179_expense_2024: u64,
    /// Line 25: Recognized IRC 1400Z-2 gain invested in Qualified Opportunity Fund.
    pub recognized_irc_1400z2_gain: u64,
    /// Line 26: Gain on disposition of NC obligations exempt before July 1, 1995.
    pub gain_disposition_exempt_nc_obligations_before_july_1995: u64,
    /// Line 27: Exempt income of a federally recognized Indian tribe member.
    pub exempt_income_federally_recognized_indian_tribe: u64,
    /// Line 28: Basis difference on disposed asset (state exceeds federal).
    pub state_basis_exceeds_federal_basis_disposed: u64,
    /// Line 29: Ordinary business expense reduced for federal credit.
    pub ordinary_business_expense_reduced_for_federal_credit: u64,
    /// Line 30: NC 529/ABLE personal education/student account deposits.
    pub personal_education_student_account_deposits: u64,
    /// Line 31: Certain state emergency/disaster relief payments.
    pub certain_state_emergency_disaster_relief_payments: u64,
    /// Line 32: Certain economic incentive payments.
    pub certain_economic_incentive_payments: u64,
    /// Line 33: Certain NC grant payments.
    pub certain_nc_grant_payments: u64,
    /// Line 34: Certain net operating loss carrybacks.
    pub certain_net_operating_loss_carrybacks: u64,
    /// Line 35: Excess net operating loss carryforward.
    pub excess_net_operating_loss_carryforward: u64,
    /// Line 36: Excess business loss.
    pub excess_business_loss: u64,
    /// Line 37: Business interest limitation adjustment.
    pub business_interest_limitation: u64,
    /// Line 38a: Taxed pass-through entity income (NC-sourced).
    pub taxed_pass_through_entity_income_nc_sourced: u64,
    /// Line 38b: Taxed pass-through entity income (non-NC-sourced).
    pub taxed_pass_through_entity_income_non_nc_sourced: u64,
}

/// D-400 Schedule S — Additions and Deductions to Federal AGI.
#[derive(Debug, Clone, Default)]
pub struct ScheduleS {
    // Header
    pub name: String,
    pub ssn: String,

    // Part A: Additions to Federal AGI
    /// Line 1
    pub interest_income_from_obligations_of_other_states: u64,
    /// Line 2
    pub deferred_gains_reinvested_opportunity_fund: u64,
    /// Line 3
    pub bonus_depreciation: u64,
    /// Line 4
    pub irc_section_179_expense: u64,
    /// Line 5
    pub s_corp_shareholder_built_in_gains_tax: u64,
    /// Line 6
    pub federal_basis_exceeds_state_basis_disposed: u64,
    /// Line 7
    pub federal_net_operating_loss_deduction: u64,
    /// Line 8
    pub state_local_foreign_income_tax_deducted_scorp_partnership_estate: u64,
    /// Line 9
    pub withdrawal_529_not_permissible: u64,
    /// Line 10
    pub discharge_qualified_principal_residence_indebtedness: u64,
    /// Line 11
    pub qualified_education_loan_payments_by_employer: u64,
    /// Line 12
    pub expenses_allocable_exempt_income: u64,
    /// Line 13
    pub discharge_certain_student_loan_debt: u64,
    /// Line 14
    pub taxed_pass_through_entity_loss: u64,
    /// Line 15: Reserved for future use.
    pub reserved_for_future_use_addition: (),
    /// Line 16: Total additions (sum of lines 1–14) → D-400 Line 7
    pub total_additions: u64,

    // Part B: Deductions from Federal AGI
    /// Line 17
    pub state_or_local_income_tax_refund: u64,
    /// Line 18
    pub interest_income_us_obligations: u64,
    /// Line 19
    pub taxable_social_security_railroad_retirement: u64,
    /// Line 20
    pub retirement_benefits_bailey_settlement: u64,
    /// Line 21
    pub certain_retirement_benefits_uniformed_services: u64,
    /// Line 22
    pub bonus_asset_basis: u64,
    /// Line 23a
    pub bonus_depreciation_2020: u64,
    /// Line 23b
    pub bonus_depreciation_2021: u64,
    /// Line 23c
    pub bonus_depreciation_2022: u64,
    /// Line 23d
    pub bonus_depreciation_2023: u64,
    /// Line 23e
    pub bonus_depreciation_2024: u64,
    /// Line 23f: Total bonus depreciation deductions (sum of 23a–23e)
    pub bonus_depreciation_total: u64,
    /// Line 24a
    pub irc_179_expense_2020: u64,
    /// Line 24b
    pub irc_179_expense_2021: u64,
    /// Line 24c
    pub irc_179_expense_2022: u64,
    /// Line 24d
    pub irc_179_expense_2023: u64,
    /// Line 24e
    pub irc_179_expense_2024: u64,
    /// Line 24f: Total IRC 179 expense deductions (sum of 24a–24e)
    pub irc_179_expense_total: u64,
    /// Line 25
    pub recognized_irc_1400z2_gain: u64,
    /// Line 26
    pub gain_disposition_exempt_nc_obligations_before_july_1995: u64,
    /// Line 27
    pub exempt_income_federally_recognized_indian_tribe: u64,
    /// Line 28
    pub state_basis_exceeds_federal_basis_disposed: u64,
    /// Line 29
    pub ordinary_business_expense_reduced_for_federal_credit: u64,
    /// Line 30
    pub personal_education_student_account_deposits: u64,
    /// Line 31
    pub certain_state_emergency_disaster_relief_payments: u64,
    /// Line 32
    pub certain_economic_incentive_payments: u64,
    /// Line 33
    pub certain_nc_grant_payments: u64,
    /// Line 34
    pub certain_net_operating_loss_carrybacks: u64,
    /// Line 35
    pub excess_net_operating_loss_carryforward: u64,
    /// Line 36
    pub excess_business_loss: u64,
    /// Line 37
    pub business_interest_limitation: u64,
    /// Line 38a
    pub taxed_pass_through_entity_income_nc_sourced: u64,
    /// Line 38b
    pub taxed_pass_through_entity_income_non_nc_sourced: u64,
    /// Line 38c: Total taxed pass-through entity income (38a + 38b)
    pub taxed_pass_through_entity_income_total: u64,
    /// Line 39
    pub nc_net_operating_loss: u64,
    /// Line 40: Reserved for future use.
    pub reserved_for_future_use_deduction: (),
    /// Line 41: Total deductions (sum of lines 17–22, 23f, 24f, 25–37, 38c, 39) → D-400 Line 9
    pub total_deductions: u64,
}

impl ScheduleS {
    pub fn is_valid(&self) -> bool {
        let expected_additions: u64 = [
            self.interest_income_from_obligations_of_other_states,
            self.deferred_gains_reinvested_opportunity_fund,
            self.bonus_depreciation,
            self.irc_section_179_expense,
            self.s_corp_shareholder_built_in_gains_tax,
            self.federal_basis_exceeds_state_basis_disposed,
            self.federal_net_operating_loss_deduction,
            self.state_local_foreign_income_tax_deducted_scorp_partnership_estate,
            self.withdrawal_529_not_permissible,
            self.discharge_qualified_principal_residence_indebtedness,
            self.qualified_education_loan_payments_by_employer,
            self.expenses_allocable_exempt_income,
            self.discharge_certain_student_loan_debt,
            self.taxed_pass_through_entity_loss,
        ]
        .into_iter()
        .sum();

        let expected_bonus_depreciation_total: u64 = [
            self.bonus_depreciation_2020,
            self.bonus_depreciation_2021,
            self.bonus_depreciation_2022,
            self.bonus_depreciation_2023,
            self.bonus_depreciation_2024,
        ]
        .into_iter()
        .sum();

        let expected_irc_179_expense_total: u64 = [
            self.irc_179_expense_2020,
            self.irc_179_expense_2021,
            self.irc_179_expense_2022,
            self.irc_179_expense_2023,
            self.irc_179_expense_2024,
        ]
        .into_iter()
        .sum();

        let expected_pass_through_total =
            self.taxed_pass_through_entity_income_nc_sourced
                + self.taxed_pass_through_entity_income_non_nc_sourced;

        let expected_deductions: u64 = [
            self.state_or_local_income_tax_refund,
            self.interest_income_us_obligations,
            self.taxable_social_security_railroad_retirement,
            self.retirement_benefits_bailey_settlement,
            self.certain_retirement_benefits_uniformed_services,
            self.bonus_asset_basis,
            self.bonus_depreciation_total,
            self.irc_179_expense_total,
            self.recognized_irc_1400z2_gain,
            self.gain_disposition_exempt_nc_obligations_before_july_1995,
            self.exempt_income_federally_recognized_indian_tribe,
            self.state_basis_exceeds_federal_basis_disposed,
            self.ordinary_business_expense_reduced_for_federal_credit,
            self.personal_education_student_account_deposits,
            self.certain_state_emergency_disaster_relief_payments,
            self.certain_economic_incentive_payments,
            self.certain_nc_grant_payments,
            self.certain_net_operating_loss_carrybacks,
            self.excess_net_operating_loss_carryforward,
            self.excess_business_loss,
            self.business_interest_limitation,
            self.taxed_pass_through_entity_income_total,
            self.nc_net_operating_loss,
        ]
        .into_iter()
        .sum();

        self.total_additions == expected_additions
            && self.bonus_depreciation_total == expected_bonus_depreciation_total
            && self.irc_179_expense_total == expected_irc_179_expense_total
            && self.taxed_pass_through_entity_income_total == expected_pass_through_total
            && self.total_deductions == expected_deductions
    }

    pub fn compute(input: &ScheduleSInput, nc_nol_deduction: u64) -> Self {
        let mut s = ScheduleS::default();

        // Header
        s.name = input.name.clone();
        s.ssn = input.ssn.clone();

        // Part A: Additions
        s.interest_income_from_obligations_of_other_states =
            input.interest_income_from_obligations_of_other_states;
        s.deferred_gains_reinvested_opportunity_fund =
            input.deferred_gains_reinvested_opportunity_fund;
        s.bonus_depreciation = input.bonus_depreciation;
        s.irc_section_179_expense = input.irc_section_179_expense;
        s.s_corp_shareholder_built_in_gains_tax = input.s_corp_shareholder_built_in_gains_tax;
        s.federal_basis_exceeds_state_basis_disposed =
            input.federal_basis_exceeds_state_basis_disposed;
        s.federal_net_operating_loss_deduction = input.federal_net_operating_loss_deduction;
        s.state_local_foreign_income_tax_deducted_scorp_partnership_estate =
            input.state_local_foreign_income_tax_deducted_scorp_partnership_estate;
        s.withdrawal_529_not_permissible = input.withdrawal_529_not_permissible;
        s.discharge_qualified_principal_residence_indebtedness =
            input.discharge_qualified_principal_residence_indebtedness;
        s.qualified_education_loan_payments_by_employer =
            input.qualified_education_loan_payments_by_employer;
        s.expenses_allocable_exempt_income = input.expenses_allocable_exempt_income;
        s.discharge_certain_student_loan_debt = input.discharge_certain_student_loan_debt;
        s.taxed_pass_through_entity_loss = input.taxed_pass_through_entity_loss;

        // Line 16: sum of lines 1–14
        s.total_additions = [
            s.interest_income_from_obligations_of_other_states,
            s.deferred_gains_reinvested_opportunity_fund,
            s.bonus_depreciation,
            s.irc_section_179_expense,
            s.s_corp_shareholder_built_in_gains_tax,
            s.federal_basis_exceeds_state_basis_disposed,
            s.federal_net_operating_loss_deduction,
            s.state_local_foreign_income_tax_deducted_scorp_partnership_estate,
            s.withdrawal_529_not_permissible,
            s.discharge_qualified_principal_residence_indebtedness,
            s.qualified_education_loan_payments_by_employer,
            s.expenses_allocable_exempt_income,
            s.discharge_certain_student_loan_debt,
            s.taxed_pass_through_entity_loss,
        ]
        .into_iter()
        .sum();

        // Part B: Deductions
        s.state_or_local_income_tax_refund = input.state_or_local_income_tax_refund;
        s.interest_income_us_obligations = input.interest_income_us_obligations;
        s.taxable_social_security_railroad_retirement =
            input.taxable_social_security_railroad_retirement;
        s.retirement_benefits_bailey_settlement = input.retirement_benefits_bailey_settlement;
        s.certain_retirement_benefits_uniformed_services =
            input.certain_retirement_benefits_uniformed_services;
        s.bonus_asset_basis = input.bonus_asset_basis;

        // Lines 23a–23e
        s.bonus_depreciation_2020 = input.bonus_depreciation_2020;
        s.bonus_depreciation_2021 = input.bonus_depreciation_2021;
        s.bonus_depreciation_2022 = input.bonus_depreciation_2022;
        s.bonus_depreciation_2023 = input.bonus_depreciation_2023;
        s.bonus_depreciation_2024 = input.bonus_depreciation_2024;
        // Line 23f
        s.bonus_depreciation_total = [
            s.bonus_depreciation_2020,
            s.bonus_depreciation_2021,
            s.bonus_depreciation_2022,
            s.bonus_depreciation_2023,
            s.bonus_depreciation_2024,
        ]
        .into_iter()
        .sum();

        // Lines 24a–24e
        s.irc_179_expense_2020 = input.irc_179_expense_2020;
        s.irc_179_expense_2021 = input.irc_179_expense_2021;
        s.irc_179_expense_2022 = input.irc_179_expense_2022;
        s.irc_179_expense_2023 = input.irc_179_expense_2023;
        s.irc_179_expense_2024 = input.irc_179_expense_2024;
        // Line 24f
        s.irc_179_expense_total = [
            s.irc_179_expense_2020,
            s.irc_179_expense_2021,
            s.irc_179_expense_2022,
            s.irc_179_expense_2023,
            s.irc_179_expense_2024,
        ]
        .into_iter()
        .sum();

        s.recognized_irc_1400z2_gain = input.recognized_irc_1400z2_gain;
        s.gain_disposition_exempt_nc_obligations_before_july_1995 =
            input.gain_disposition_exempt_nc_obligations_before_july_1995;
        s.exempt_income_federally_recognized_indian_tribe =
            input.exempt_income_federally_recognized_indian_tribe;
        s.state_basis_exceeds_federal_basis_disposed =
            input.state_basis_exceeds_federal_basis_disposed;
        s.ordinary_business_expense_reduced_for_federal_credit =
            input.ordinary_business_expense_reduced_for_federal_credit;
        s.personal_education_student_account_deposits =
            input.personal_education_student_account_deposits;
        s.certain_state_emergency_disaster_relief_payments =
            input.certain_state_emergency_disaster_relief_payments;
        s.certain_economic_incentive_payments = input.certain_economic_incentive_payments;
        s.certain_nc_grant_payments = input.certain_nc_grant_payments;
        s.certain_net_operating_loss_carrybacks = input.certain_net_operating_loss_carrybacks;
        s.excess_net_operating_loss_carryforward = input.excess_net_operating_loss_carryforward;
        s.excess_business_loss = input.excess_business_loss;
        s.business_interest_limitation = input.business_interest_limitation;

        // Lines 38a–38c
        s.taxed_pass_through_entity_income_nc_sourced =
            input.taxed_pass_through_entity_income_nc_sourced;
        s.taxed_pass_through_entity_income_non_nc_sourced =
            input.taxed_pass_through_entity_income_non_nc_sourced;
        s.taxed_pass_through_entity_income_total = s.taxed_pass_through_entity_income_nc_sourced
            + s.taxed_pass_through_entity_income_non_nc_sourced;

        s.nc_net_operating_loss = nc_nol_deduction;

        // Line 41: sum of lines 17–22, 23f, 24f, 25–37, 38c, 39
        s.total_deductions = [
            s.state_or_local_income_tax_refund,
            s.interest_income_us_obligations,
            s.taxable_social_security_railroad_retirement,
            s.retirement_benefits_bailey_settlement,
            s.certain_retirement_benefits_uniformed_services,
            s.bonus_asset_basis,
            s.bonus_depreciation_total,
            s.irc_179_expense_total,
            s.recognized_irc_1400z2_gain,
            s.gain_disposition_exempt_nc_obligations_before_july_1995,
            s.exempt_income_federally_recognized_indian_tribe,
            s.state_basis_exceeds_federal_basis_disposed,
            s.ordinary_business_expense_reduced_for_federal_credit,
            s.personal_education_student_account_deposits,
            s.certain_state_emergency_disaster_relief_payments,
            s.certain_economic_incentive_payments,
            s.certain_nc_grant_payments,
            s.certain_net_operating_loss_carrybacks,
            s.excess_net_operating_loss_carryforward,
            s.excess_business_loss,
            s.business_interest_limitation,
            s.taxed_pass_through_entity_income_total,
            s.nc_net_operating_loss,
        ]
        .into_iter()
        .sum();

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_additions_is_sum_of_lines_1_through_15() {
        let input = ScheduleSInput {
            interest_income_from_obligations_of_other_states: 100,
            deferred_gains_reinvested_opportunity_fund: 200,
            bonus_depreciation: 300,
            irc_section_179_expense: 400,
            s_corp_shareholder_built_in_gains_tax: 500,
            federal_basis_exceeds_state_basis_disposed: 50,
            federal_net_operating_loss_deduction: 75,
            state_local_foreign_income_tax_deducted_scorp_partnership_estate: 25,
            withdrawal_529_not_permissible: 10,
            discharge_qualified_principal_residence_indebtedness: 15,
            qualified_education_loan_payments_by_employer: 20,
            expenses_allocable_exempt_income: 30,
            discharge_certain_student_loan_debt: 40,
            taxed_pass_through_entity_loss: 60,
            ..Default::default()
        };
        let s = ScheduleS::compute(&input, 0);
        // 100+200+300+400+500+50+75+25+10+15+20+30+40+60 = 1825
        assert_eq!(s.total_additions, 1825);
    }

    #[test]
    fn total_deductions_is_sum_of_part_b() {
        let input = ScheduleSInput {
            state_or_local_income_tax_refund: 500,
            interest_income_us_obligations: 1_000,
            taxable_social_security_railroad_retirement: 2_000,
            bonus_depreciation_2020: 100,
            bonus_depreciation_2021: 200,
            irc_179_expense_2023: 300,
            taxed_pass_through_entity_income_nc_sourced: 400,
            taxed_pass_through_entity_income_non_nc_sourced: 150,
            ..Default::default()
        };
        let s = ScheduleS::compute(&input, 0);
        assert_eq!(s.bonus_depreciation_total, 300);
        assert_eq!(s.irc_179_expense_total, 300);
        assert_eq!(s.taxed_pass_through_entity_income_total, 550);
        // 500 + 1000 + 2000 + 300 + 300 + 550 = 4650
        assert_eq!(s.total_deductions, 4_650);
    }

    #[test]
    fn default_schedule_s_is_all_zeros() {
        let s = ScheduleS::compute(&ScheduleSInput::default(), 0);
        assert_eq!(s.total_additions, 0);
        assert_eq!(s.total_deductions, 0);
    }

    #[test]
    fn computed_schedule_s_is_valid() {
        let input = ScheduleSInput {
            interest_income_from_obligations_of_other_states: 100,
            state_or_local_income_tax_refund: 500,
            bonus_depreciation_2020: 200,
            taxed_pass_through_entity_income_nc_sourced: 300,
            taxed_pass_through_entity_income_non_nc_sourced: 150,
            ..Default::default()
        };
        let s = ScheduleS::compute(&input, 1_000);
        assert!(s.is_valid());
    }

    #[test]
    fn inconsistent_schedule_s_is_invalid() {
        let mut s = ScheduleS::compute(&ScheduleSInput::default(), 0);
        s.total_additions = 999;
        assert!(!s.is_valid());
    }
}
