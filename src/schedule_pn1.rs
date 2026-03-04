use crate::schedule_pn::TwoColumnLine;

/// Manual inputs for D-400 Schedule PN-1.
#[derive(Debug, Clone, Default)]
pub struct SchedulePn1Input {
    // Header
    pub last_name: String,
    pub ssn: String,

    // Part A: Other Additions (Lines 1–10)
    /// Line 1: S Corporation Shareholder Built-in Gains Tax
    pub s_corp_shareholder_built_in_gains_tax: TwoColumnLine,
    /// Line 2: Amount by Which Federal Basis Exceeds State Basis for Property Disposed
    pub federal_basis_exceeds_state_basis: TwoColumnLine,
    /// Line 3: Federal Net Operating Loss Deduction
    pub federal_net_operating_loss_deduction: TwoColumnLine,
    /// Line 4: State, Local, or Foreign Income Tax Deducted by S-Corp, Partnership, or Estate/Trust
    pub state_local_foreign_income_tax_deducted: TwoColumnLine,
    /// Line 5: Withdrawal of 529 Plan Contributions Not Used for Permissible Purpose
    pub withdrawal_529_not_permissible: TwoColumnLine,
    /// Line 6: Discharge of Qualified Principal Residence Indebtedness
    pub discharge_qualified_principal_residence: TwoColumnLine,
    /// Line 7: Qualified Education Loan Payments Paid by Employer
    pub qualified_education_loan_payments: TwoColumnLine,
    /// Line 8: Expenses Allocable to Income Exempt or Excluded From Gross Income
    pub expenses_allocable_exempt_income: TwoColumnLine,
    /// Line 9: Discharge of Certain Student Loan Debt
    pub discharge_certain_student_loan_debt: TwoColumnLine,
    /// Line 10: Taxed Pass-Through Entity Loss
    pub taxed_pass_through_entity_loss: TwoColumnLine,

    // Part B: Other Deductions (Lines 13–28)
    /// Line 13: Certain Retirement Benefits (Uniformed Services, not on PN Line 19d)
    pub certain_retirement_benefits_uniformed_services: TwoColumnLine,
    /// Line 14: Recognized IRC Section 1400Z-2 Gain
    pub recognized_irc_1400z2_gain: TwoColumnLine,
    /// Line 15: Gain From Disposition of Exempt N.C. Obligations Before July 1, 1995
    pub gain_disposition_exempt_nc_obligations: TwoColumnLine,
    /// Line 16: Exempt Income of Federally Recognized Indian Tribe Member
    pub exempt_income_indian_tribe: TwoColumnLine,
    /// Line 17: Amount by Which State Basis Exceeds Federal Basis for Property Disposed
    pub state_basis_exceeds_federal_basis: TwoColumnLine,
    /// Line 18: Ordinary Business Expense Reduced for Federal Tax Credit
    pub ordinary_business_expense_reduced: TwoColumnLine,
    /// Line 19: Personal Education Student Account Deposits
    pub personal_education_student_account_deposits: TwoColumnLine,
    /// Line 20: Certain State Emergency/Disaster Relief Payments
    pub certain_emergency_disaster_payments: TwoColumnLine,
    /// Line 21: Certain Economic Incentive Payments
    pub certain_economic_incentive_payments: TwoColumnLine,
    /// Line 22: Certain N.C. Grant Payments
    pub certain_nc_grant_payments: TwoColumnLine,
    /// Line 23: Certain Net Operating Loss Carrybacks
    pub certain_net_operating_loss_carrybacks: TwoColumnLine,
    /// Line 24: Excess Net Operating Loss Carryforward
    pub excess_net_operating_loss_carryforward: TwoColumnLine,
    /// Line 25: Excess Business Loss
    pub excess_business_loss: TwoColumnLine,
    /// Line 26: Business Interest Limitation
    pub business_interest_limitation: TwoColumnLine,
    /// Line 27a: Taxed Pass-Through Entity Income (N.C. Sourced)
    pub taxed_pass_through_entity_income_nc: TwoColumnLine,
    /// Line 27b: Taxed Pass-Through Entity Income (Non-N.C. Sourced)
    pub taxed_pass_through_entity_income_non_nc: TwoColumnLine,
    /// Line 28: N.C. Net Operating Loss
    pub nc_net_operating_loss: TwoColumnLine,
}

/// D-400 Schedule PN-1 — Other Additions and Other Deductions.
#[derive(Debug, Clone, Default)]
pub struct SchedulePn1 {
    // Header
    pub last_name: String,
    pub ssn: String,

    // Part A: Other Additions
    /// Line 1
    pub s_corp_shareholder_built_in_gains_tax: TwoColumnLine,
    /// Line 2
    pub federal_basis_exceeds_state_basis: TwoColumnLine,
    /// Line 3
    pub federal_net_operating_loss_deduction: TwoColumnLine,
    /// Line 4
    pub state_local_foreign_income_tax_deducted: TwoColumnLine,
    /// Line 5
    pub withdrawal_529_not_permissible: TwoColumnLine,
    /// Line 6
    pub discharge_qualified_principal_residence: TwoColumnLine,
    /// Line 7
    pub qualified_education_loan_payments: TwoColumnLine,
    /// Line 8
    pub expenses_allocable_exempt_income: TwoColumnLine,
    /// Line 9
    pub discharge_certain_student_loan_debt: TwoColumnLine,
    /// Line 10
    pub taxed_pass_through_entity_loss: TwoColumnLine,
    /// Line 11: Reserved for Future Use
    pub reserved_for_future_use_addition: (),
    /// Line 12: Total Other Additions (Add Lines 1 through 11) → PN Line 17e
    pub total_other_additions: TwoColumnLine,

    // Part B: Other Deductions
    /// Line 13
    pub certain_retirement_benefits_uniformed_services: TwoColumnLine,
    /// Line 14
    pub recognized_irc_1400z2_gain: TwoColumnLine,
    /// Line 15
    pub gain_disposition_exempt_nc_obligations: TwoColumnLine,
    /// Line 16
    pub exempt_income_indian_tribe: TwoColumnLine,
    /// Line 17
    pub state_basis_exceeds_federal_basis: TwoColumnLine,
    /// Line 18
    pub ordinary_business_expense_reduced: TwoColumnLine,
    /// Line 19
    pub personal_education_student_account_deposits: TwoColumnLine,
    /// Line 20
    pub certain_emergency_disaster_payments: TwoColumnLine,
    /// Line 21
    pub certain_economic_incentive_payments: TwoColumnLine,
    /// Line 22
    pub certain_nc_grant_payments: TwoColumnLine,
    /// Line 23
    pub certain_net_operating_loss_carrybacks: TwoColumnLine,
    /// Line 24
    pub excess_net_operating_loss_carryforward: TwoColumnLine,
    /// Line 25
    pub excess_business_loss: TwoColumnLine,
    /// Line 26
    pub business_interest_limitation: TwoColumnLine,
    /// Line 27a
    pub taxed_pass_through_entity_income_nc: TwoColumnLine,
    /// Line 27b
    pub taxed_pass_through_entity_income_non_nc: TwoColumnLine,
    /// Line 28
    pub nc_net_operating_loss: TwoColumnLine,
    /// Line 29: Reserved for Future Use
    pub reserved_for_future_use_deduction: (),
    /// Line 30: Total Other Deductions (Add Lines 13 through 29) → PN Line 19h
    pub total_other_deductions: TwoColumnLine,
}

impl SchedulePn1 {
    pub fn is_valid(&self) -> bool {
        let addition_lines = [
            self.s_corp_shareholder_built_in_gains_tax,
            self.federal_basis_exceeds_state_basis,
            self.federal_net_operating_loss_deduction,
            self.state_local_foreign_income_tax_deducted,
            self.withdrawal_529_not_permissible,
            self.discharge_qualified_principal_residence,
            self.qualified_education_loan_payments,
            self.expenses_allocable_exempt_income,
            self.discharge_certain_student_loan_debt,
            self.taxed_pass_through_entity_loss,
        ];
        let expected_additions = TwoColumnLine {
            col_a: addition_lines.iter().map(|l| l.col_a).sum(),
            col_b: addition_lines.iter().map(|l| l.col_b).sum(),
        };

        let deduction_lines = [
            self.certain_retirement_benefits_uniformed_services,
            self.recognized_irc_1400z2_gain,
            self.gain_disposition_exempt_nc_obligations,
            self.exempt_income_indian_tribe,
            self.state_basis_exceeds_federal_basis,
            self.ordinary_business_expense_reduced,
            self.personal_education_student_account_deposits,
            self.certain_emergency_disaster_payments,
            self.certain_economic_incentive_payments,
            self.certain_nc_grant_payments,
            self.certain_net_operating_loss_carrybacks,
            self.excess_net_operating_loss_carryforward,
            self.excess_business_loss,
            self.business_interest_limitation,
            self.taxed_pass_through_entity_income_nc,
            self.taxed_pass_through_entity_income_non_nc,
            self.nc_net_operating_loss,
        ];
        let expected_deductions = TwoColumnLine {
            col_a: deduction_lines.iter().map(|l| l.col_a).sum(),
            col_b: deduction_lines.iter().map(|l| l.col_b).sum(),
        };

        let all_lines: Vec<&TwoColumnLine> = addition_lines
            .iter()
            .chain(deduction_lines.iter())
            .chain(std::iter::once(&self.total_other_additions))
            .chain(std::iter::once(&self.total_other_deductions))
            .collect();
        let col_b_le_col_a = all_lines.iter().all(|l| l.col_b <= l.col_a);

        self.total_other_additions.col_a == expected_additions.col_a
            && self.total_other_additions.col_b == expected_additions.col_b
            && self.total_other_deductions.col_a == expected_deductions.col_a
            && self.total_other_deductions.col_b == expected_deductions.col_b
            && col_b_le_col_a
    }

    /// Compute Schedule PN-1 from inputs.
    #[allow(clippy::field_reassign_with_default)]
    pub fn compute(input: &SchedulePn1Input) -> Self {
        let mut pn1 = SchedulePn1::default();

        // Header
        pn1.last_name = input.last_name.clone();
        pn1.ssn = input.ssn.clone();

        // Part A: Additions (Lines 1–10)
        pn1.s_corp_shareholder_built_in_gains_tax = input.s_corp_shareholder_built_in_gains_tax;
        pn1.federal_basis_exceeds_state_basis = input.federal_basis_exceeds_state_basis;
        pn1.federal_net_operating_loss_deduction = input.federal_net_operating_loss_deduction;
        pn1.state_local_foreign_income_tax_deducted = input.state_local_foreign_income_tax_deducted;
        pn1.withdrawal_529_not_permissible = input.withdrawal_529_not_permissible;
        pn1.discharge_qualified_principal_residence = input.discharge_qualified_principal_residence;
        pn1.qualified_education_loan_payments = input.qualified_education_loan_payments;
        pn1.expenses_allocable_exempt_income = input.expenses_allocable_exempt_income;
        pn1.discharge_certain_student_loan_debt = input.discharge_certain_student_loan_debt;
        pn1.taxed_pass_through_entity_loss = input.taxed_pass_through_entity_loss;

        // Line 12: sum of lines 1–10 (line 11 is reserved ())
        let addition_lines = [
            pn1.s_corp_shareholder_built_in_gains_tax,
            pn1.federal_basis_exceeds_state_basis,
            pn1.federal_net_operating_loss_deduction,
            pn1.state_local_foreign_income_tax_deducted,
            pn1.withdrawal_529_not_permissible,
            pn1.discharge_qualified_principal_residence,
            pn1.qualified_education_loan_payments,
            pn1.expenses_allocable_exempt_income,
            pn1.discharge_certain_student_loan_debt,
            pn1.taxed_pass_through_entity_loss,
        ];
        pn1.total_other_additions = TwoColumnLine {
            col_a: addition_lines.iter().map(|l| l.col_a).sum(),
            col_b: addition_lines.iter().map(|l| l.col_b).sum(),
        };

        // Part B: Deductions (Lines 13–28)
        pn1.certain_retirement_benefits_uniformed_services =
            input.certain_retirement_benefits_uniformed_services;
        pn1.recognized_irc_1400z2_gain = input.recognized_irc_1400z2_gain;
        pn1.gain_disposition_exempt_nc_obligations = input.gain_disposition_exempt_nc_obligations;
        pn1.exempt_income_indian_tribe = input.exempt_income_indian_tribe;
        pn1.state_basis_exceeds_federal_basis = input.state_basis_exceeds_federal_basis;
        pn1.ordinary_business_expense_reduced = input.ordinary_business_expense_reduced;
        pn1.personal_education_student_account_deposits =
            input.personal_education_student_account_deposits;
        pn1.certain_emergency_disaster_payments = input.certain_emergency_disaster_payments;
        pn1.certain_economic_incentive_payments = input.certain_economic_incentive_payments;
        pn1.certain_nc_grant_payments = input.certain_nc_grant_payments;
        pn1.certain_net_operating_loss_carrybacks = input.certain_net_operating_loss_carrybacks;
        pn1.excess_net_operating_loss_carryforward = input.excess_net_operating_loss_carryforward;
        pn1.excess_business_loss = input.excess_business_loss;
        pn1.business_interest_limitation = input.business_interest_limitation;
        pn1.taxed_pass_through_entity_income_nc = input.taxed_pass_through_entity_income_nc;
        pn1.taxed_pass_through_entity_income_non_nc = input.taxed_pass_through_entity_income_non_nc;
        pn1.nc_net_operating_loss = input.nc_net_operating_loss;

        // Line 30: sum of lines 13–28 (line 29 is reserved ())
        let deduction_lines = [
            pn1.certain_retirement_benefits_uniformed_services,
            pn1.recognized_irc_1400z2_gain,
            pn1.gain_disposition_exempt_nc_obligations,
            pn1.exempt_income_indian_tribe,
            pn1.state_basis_exceeds_federal_basis,
            pn1.ordinary_business_expense_reduced,
            pn1.personal_education_student_account_deposits,
            pn1.certain_emergency_disaster_payments,
            pn1.certain_economic_incentive_payments,
            pn1.certain_nc_grant_payments,
            pn1.certain_net_operating_loss_carrybacks,
            pn1.excess_net_operating_loss_carryforward,
            pn1.excess_business_loss,
            pn1.business_interest_limitation,
            pn1.taxed_pass_through_entity_income_nc,
            pn1.taxed_pass_through_entity_income_non_nc,
            pn1.nc_net_operating_loss,
        ];
        pn1.total_other_deductions = TwoColumnLine {
            col_a: deduction_lines.iter().map(|l| l.col_a).sum(),
            col_b: deduction_lines.iter().map(|l| l.col_b).sum(),
        };

        pn1
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn total_additions_sums_lines_1_through_10() {
        let input = SchedulePn1Input {
            s_corp_shareholder_built_in_gains_tax: TwoColumnLine {
                col_a: 100,
                col_b: 50,
            },
            federal_net_operating_loss_deduction: TwoColumnLine {
                col_a: 200,
                col_b: 100,
            },
            ..Default::default()
        };
        let pn1 = SchedulePn1::compute(&input);
        assert_eq!(pn1.total_other_additions.col_a, 300);
        assert_eq!(pn1.total_other_additions.col_b, 150);
    }

    #[test]
    fn total_deductions_sums_lines_13_through_28() {
        let input = SchedulePn1Input {
            recognized_irc_1400z2_gain: TwoColumnLine {
                col_a: 500,
                col_b: 250,
            },
            nc_net_operating_loss: TwoColumnLine {
                col_a: 300,
                col_b: 150,
            },
            ..Default::default()
        };
        let pn1 = SchedulePn1::compute(&input);
        assert_eq!(pn1.total_other_deductions.col_a, 800);
        assert_eq!(pn1.total_other_deductions.col_b, 400);
    }

    #[test]
    fn default_is_all_zeros() {
        let pn1 = SchedulePn1::compute(&SchedulePn1Input::default());
        assert_eq!(pn1.total_other_additions.col_a, 0);
        assert_eq!(pn1.total_other_deductions.col_b, 0);
    }

    #[test]
    fn computed_schedule_pn1_is_valid() {
        let input = SchedulePn1Input {
            s_corp_shareholder_built_in_gains_tax: TwoColumnLine {
                col_a: 100,
                col_b: 50,
            },
            recognized_irc_1400z2_gain: TwoColumnLine {
                col_a: 500,
                col_b: 250,
            },
            ..Default::default()
        };
        let pn1 = SchedulePn1::compute(&input);
        assert!(pn1.is_valid());
    }

    #[test]
    fn inconsistent_schedule_pn1_is_invalid() {
        let mut pn1 = SchedulePn1::compute(&SchedulePn1Input::default());
        pn1.total_other_additions.col_a = 999;
        assert!(!pn1.is_valid());
    }
}
