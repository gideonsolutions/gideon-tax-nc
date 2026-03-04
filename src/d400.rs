use us_tax_brackets::FilingStatus;

use crate::child_deduction::child_deduction_per_child;
use crate::constants::NC_TAX_RATE_BPS;
use crate::counties::County;
use crate::d400tc::{D400Tc, D400TcInput};
use crate::nc_478::{Nc478, Nc478Input};
use crate::nc_nol::{NcNolPart1, NcNolPart1Input, NcNolPart2, NcNolPart2Input};
use crate::nc_rehab::{NcRehab, NcRehabInput};
use crate::schedule_a::{ScheduleA, ScheduleAInput, nc_standard_deduction};
use crate::schedule_pn::{SchedulePn, SchedulePnInput, TwoColumnLine};
use crate::schedule_pn1::{SchedulePn1, SchedulePn1Input};
use crate::schedule_s::{ScheduleS, ScheduleSInput};
use crate::federal::FederalReturn;

/// Complete set of NC-specific inputs needed to compute a D-400 return.
#[derive(Debug, Clone)]
pub struct NcReturnInput {
    /// Filing status (reuse federal).
    pub filing_status: FilingStatus,
    /// Federal return data.
    pub federal_return: FederalReturn,

    // Header / Filing info
    /// Out of Country: taxpayer (or spouse if MFJ) was out of the country on April 15, 2026.
    pub out_of_country: bool,

    // Deceased Taxpayer Information
    /// Return is filed and signed by Executor, Administrator, or Court-Appointed Personal Representative.
    pub deceased_taxpayer: bool,
    /// Date of death of taxpayer (MM-DD-YY).
    pub taxpayer_date_of_death: String,
    /// Date of death of spouse (MM-DD-YY).
    pub spouse_date_of_death: String,

    // Mailing address
    pub mailing_address: String,
    pub apartment_number: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub county: Option<County>,

    pub residency_status_taxpayer: bool,
    pub residency_status_spouse: bool,
    pub is_veteran: bool,
    pub is_spouse_veteran: bool,
    pub federal_extension: bool,
    pub amended_return: bool,

    /// Qualifying Widow(er): year spouse died.
    pub qualifying_widow_year_spouse_died: String,
    /// MFS / MFJ: spouse's full name.
    pub spouse_name: String,
    /// MFS / MFJ: spouse's Social Security Number.
    pub spouse_ssn: String,

    // Schedule S inputs
    pub schedule_s_input: ScheduleSInput,

    // Schedule A choice
    pub use_nc_itemized_deductions: bool,
    /// Whether the MFS spouse itemizes (affects standard deduction).
    pub mfs_spouse_itemizes: bool,
    pub schedule_a_input: ScheduleAInput,

    // D-400 specific
    pub child_deduction_qualifying_children: u32,

    // Optional sub-form inputs
    pub nc_nol_part1_input: Option<NcNolPart1Input>,
    pub nc_nol_part2_input: Option<NcNolPart2Input>,
    pub schedule_pn_input: Option<SchedulePnInput>,
    pub schedule_pn1_input: Option<SchedulePn1Input>,
    pub nc_rehab_input: Option<NcRehabInput>,
    pub nc_478_input: Option<Nc478Input>,

    // D-400TC inputs
    pub d400tc_input: D400TcInput,

    // Payment info
    pub nc_tax_withheld_taxpayer: u64,
    pub nc_tax_withheld_spouse: u64,
    pub estimated_tax_payments: u64,
    pub paid_with_extension: u64,
    pub partnership_payment: u64,
    pub s_corporation_payment: u64,
    pub consumer_use_tax: u64,
    pub no_consumer_use_tax: bool,

    // Amended return fields
    pub additional_payments_amended: u64,
    pub previous_refunds_amended: u64,

    // Penalties / interest
    pub penalties: u64,
    pub interest: u64,
    pub interest_underpayment_estimated: u64,

    // Voluntary contributions
    pub applied_to_estimated_tax: u64,
    pub contribution_wildlife_fund: u64,
    pub contribution_education_fund: u64,
    pub contribution_cancer_program: u64,
}

impl Default for NcReturnInput {
    fn default() -> Self {
        Self {
            filing_status: FilingStatus::Single,
            federal_return: FederalReturn::default(),
            out_of_country: false,
            deceased_taxpayer: false,
            taxpayer_date_of_death: String::new(),
            spouse_date_of_death: String::new(),
            mailing_address: String::new(),
            apartment_number: String::new(),
            city: String::new(),
            state: String::new(),
            zip_code: String::new(),
            country: String::new(),
            county: None,
            residency_status_taxpayer: false,
            residency_status_spouse: false,
            is_veteran: false,
            is_spouse_veteran: false,
            federal_extension: false,
            amended_return: false,
            qualifying_widow_year_spouse_died: String::new(),
            spouse_name: String::new(),
            spouse_ssn: String::new(),
            schedule_s_input: ScheduleSInput::default(),
            use_nc_itemized_deductions: false,
            mfs_spouse_itemizes: false,
            schedule_a_input: ScheduleAInput::default(),
            child_deduction_qualifying_children: 0,
            nc_nol_part1_input: None,
            nc_nol_part2_input: None,
            schedule_pn_input: None,
            schedule_pn1_input: None,
            nc_rehab_input: None,
            nc_478_input: None,
            d400tc_input: D400TcInput::default(),
            nc_tax_withheld_taxpayer: 0,
            nc_tax_withheld_spouse: 0,
            estimated_tax_payments: 0,
            paid_with_extension: 0,
            partnership_payment: 0,
            s_corporation_payment: 0,
            consumer_use_tax: 0,
            no_consumer_use_tax: false,
            additional_payments_amended: 0,
            previous_refunds_amended: 0,
            penalties: 0,
            interest: 0,
            interest_underpayment_estimated: 0,
            applied_to_estimated_tax: 0,
            contribution_wildlife_fund: 0,
            contribution_education_fund: 0,
            contribution_cancer_program: 0,
        }
    }
}

/// D-400 — NC Individual Income Tax Return.
#[derive(Debug, Clone)]
pub struct D400 {
    // Header / Filing info
    pub filing_status: FilingStatus,
    pub out_of_country: bool,

    // Deceased Taxpayer Information
    pub deceased_taxpayer: bool,
    pub taxpayer_date_of_death: String,
    pub spouse_date_of_death: String,

    // Mailing address
    pub mailing_address: String,
    pub apartment_number: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub county: Option<County>,

    pub residency_status_taxpayer: bool,
    pub residency_status_spouse: bool,
    pub is_veteran: bool,
    pub is_spouse_veteran: bool,
    pub federal_extension: bool,
    pub amended_return: bool,

    pub qualifying_widow_year_spouse_died: String,
    pub spouse_name: String,
    pub spouse_ssn: String,

    // Income lines (Lines 6–15)
    /// Line 6: Federal AGI.
    pub federal_agi: u64,
    /// Line 7: Additions to federal AGI (Schedule S Part A Line 16).
    pub additions_to_federal_agi: u64,
    /// Line 8: Line 6 + Line 7.
    pub add_lines_6_and_7: u64,
    /// Line 9: Deductions from federal AGI (Schedule S Part B Line 41).
    pub deductions_from_federal_agi: u64,
    /// Line 10a: Number of qualifying children.
    pub child_deduction_qualifying_children: u32,
    /// Line 10b: Child deduction amount ($3,000 × qualifying children).
    pub child_deduction_amount: u64,
    /// Line 11: NC standard or itemized deduction.
    pub nc_standard_or_itemized_deduction: u64,
    /// True = standard deduction, false = itemized.
    pub line_11_is_standard: bool,
    /// Line 12a: Line 9 + Line 10b + Line 11.
    pub add_lines_9_10b_11: u64,
    /// Line 12b: Line 8 − Line 12a (zero floor).
    pub subtract_line_12a_from_line_8: u64,
    /// Line 13: Part-year / nonresident taxable percentage (0 for full-year, stored as 0–10000).
    pub part_year_nonresident_taxable_pct: u16,
    /// Line 14: NC taxable income.
    pub nc_taxable_income: u64,
    /// Line 15: NC income tax (Line 14 × 4.25%, zero floor).
    pub nc_income_tax: u64,

    // Tax / Credits / Payments (Lines 16–34)
    /// Line 16: Tax credits (D-400TC Part 3 Line 20).
    pub tax_credits: u64,
    /// Line 17: Line 15 − Line 16.
    pub subtract_line_16_from_15: u64,
    /// Line 18: Consumer use tax.
    pub consumer_use_tax: u64,
    /// No consumer use tax checkbox.
    pub line_18_no_consumer_use_tax: bool,
    /// Line 19: Line 17 + Line 18.
    pub add_lines_17_and_18: u64,

    /// Line 20a: NC tax withheld (taxpayer).
    pub nc_tax_withheld_taxpayer: u64,
    /// Line 20b: NC tax withheld (spouse).
    pub nc_tax_withheld_spouse: u64,
    /// Line 21a: Estimated tax payments.
    pub estimated_tax_payments: u64,
    /// Line 21b: Paid with extension.
    pub paid_with_extension: u64,
    /// Line 21c: Partnership payment.
    pub partnership_payment: u64,
    /// Line 21d: S-corporation payment.
    pub s_corporation_payment: u64,
    /// Line 22: Additional payments (amended returns).
    pub additional_payments_amended: u64,
    /// Line 23: Sum of lines 20a through 22.
    pub add_lines_20a_through_22: u64,
    /// Line 24: Previous refunds (amended returns).
    pub previous_refunds_amended: u64,
    /// Line 25: Line 23 − Line 24.
    pub subtract_line_24_from_23: u64,

    /// Line 26a: Tax due (if Line 25 < Line 19).
    pub tax_due: u64,
    /// Line 26b: Penalties.
    pub penalties: u64,
    /// Line 26c: Interest.
    pub interest: u64,
    /// Line 26d: Penalties + interest.
    pub penalties_plus_interest: u64,
    /// Line 26e: Interest on underpayment of estimated tax.
    pub interest_underpayment_estimated: u64,
    /// Line 27: Amount due (26a + 26d + 26e).
    pub amount_due: u64,

    /// Line 28: Overpayment (if Line 25 > Line 19).
    pub overpayment: u64,
    /// Line 29: Applied to estimated tax.
    pub applied_to_estimated_tax: u64,
    /// Line 30: Contribution to NC Wildlife Fund.
    pub contribution_wildlife_fund: u64,
    /// Line 31: Contribution to NC Education Fund.
    pub contribution_education_fund: u64,
    /// Line 32: Contribution to NC Cancer Program.
    pub contribution_cancer_program: u64,
    /// Line 33: Sum of lines 29–32.
    pub add_lines_29_through_32: u64,
    /// Line 34: Amount to be refunded (Line 28 − Line 33, zero floor).
    pub amount_to_be_refunded: u64,

    // Computed sub-forms
    pub schedule_s: Option<ScheduleS>,
    pub schedule_a: Option<ScheduleA>,
    pub schedule_pn: Option<SchedulePn>,
    pub schedule_pn1: Option<SchedulePn1>,
    pub d400tc: Option<D400Tc>,
    pub nc_nol_part1: Option<NcNolPart1>,
    pub nc_nol_part2: Option<NcNolPart2>,
    pub nc_rehab: Option<NcRehab>,
    pub nc_478: Option<Nc478>,
}

impl Default for D400 {
    fn default() -> Self {
        Self {
            filing_status: FilingStatus::Single,
            out_of_country: false,
            deceased_taxpayer: false,
            taxpayer_date_of_death: String::new(),
            spouse_date_of_death: String::new(),
            mailing_address: String::new(),
            apartment_number: String::new(),
            city: String::new(),
            state: String::new(),
            zip_code: String::new(),
            country: String::new(),
            county: None,
            residency_status_taxpayer: false,
            residency_status_spouse: false,
            is_veteran: false,
            is_spouse_veteran: false,
            federal_extension: false,
            amended_return: false,
            qualifying_widow_year_spouse_died: String::new(),
            spouse_name: String::new(),
            spouse_ssn: String::new(),
            federal_agi: 0,
            additions_to_federal_agi: 0,
            add_lines_6_and_7: 0,
            deductions_from_federal_agi: 0,
            child_deduction_qualifying_children: 0,
            child_deduction_amount: 0,
            nc_standard_or_itemized_deduction: 0,
            line_11_is_standard: false,
            add_lines_9_10b_11: 0,
            subtract_line_12a_from_line_8: 0,
            part_year_nonresident_taxable_pct: 0,
            nc_taxable_income: 0,
            nc_income_tax: 0,
            tax_credits: 0,
            subtract_line_16_from_15: 0,
            consumer_use_tax: 0,
            line_18_no_consumer_use_tax: false,
            add_lines_17_and_18: 0,
            nc_tax_withheld_taxpayer: 0,
            nc_tax_withheld_spouse: 0,
            estimated_tax_payments: 0,
            paid_with_extension: 0,
            partnership_payment: 0,
            s_corporation_payment: 0,
            additional_payments_amended: 0,
            add_lines_20a_through_22: 0,
            previous_refunds_amended: 0,
            subtract_line_24_from_23: 0,
            tax_due: 0,
            penalties: 0,
            interest: 0,
            penalties_plus_interest: 0,
            interest_underpayment_estimated: 0,
            amount_due: 0,
            overpayment: 0,
            applied_to_estimated_tax: 0,
            contribution_wildlife_fund: 0,
            contribution_education_fund: 0,
            contribution_cancer_program: 0,
            add_lines_29_through_32: 0,
            amount_to_be_refunded: 0,
            schedule_s: None,
            schedule_a: None,
            schedule_pn: None,
            schedule_pn1: None,
            d400tc: None,
            nc_nol_part1: None,
            nc_nol_part2: None,
            nc_rehab: None,
            nc_478: None,
        }
    }
}

impl D400 {
    pub fn is_valid(&self) -> bool {
        // Arithmetic checks
        let line_8_ok = self.add_lines_6_and_7
            == self.federal_agi + self.additions_to_federal_agi;

        let per_child = child_deduction_per_child(self.filing_status, self.federal_agi);
        let child_deduction_ok = self.child_deduction_amount
            == per_child * self.child_deduction_qualifying_children as u64;

        let line_12a_ok = self.add_lines_9_10b_11
            == self.deductions_from_federal_agi
                + self.child_deduction_amount
                + self.nc_standard_or_itemized_deduction;

        let line_12b_ok = self.subtract_line_12a_from_line_8
            == self.add_lines_6_and_7.saturating_sub(self.add_lines_9_10b_11);

        let nc_tax_ok = self.nc_income_tax
            == (self.nc_taxable_income * NC_TAX_RATE_BPS + 5_000) / 10_000;

        let line_17_ok = self.subtract_line_16_from_15
            == self.nc_income_tax.saturating_sub(self.tax_credits);

        let line_19_ok = self.add_lines_17_and_18
            == self.subtract_line_16_from_15 + self.consumer_use_tax;

        let line_23_ok = self.add_lines_20a_through_22
            == [
                self.nc_tax_withheld_taxpayer,
                self.nc_tax_withheld_spouse,
                self.estimated_tax_payments,
                self.paid_with_extension,
                self.partnership_payment,
                self.s_corporation_payment,
                self.additional_payments_amended,
            ]
            .into_iter()
            .sum::<u64>();

        let line_25_ok = self.subtract_line_24_from_23
            == self
                .add_lines_20a_through_22
                .saturating_sub(self.previous_refunds_amended);

        let penalties_ok = self.penalties_plus_interest == self.penalties + self.interest;

        let amount_due_ok = self.amount_due
            == self.tax_due + self.penalties_plus_interest + self.interest_underpayment_estimated;

        let line_33_ok = self.add_lines_29_through_32
            == self.applied_to_estimated_tax
                + self.contribution_wildlife_fund
                + self.contribution_education_fund
                + self.contribution_cancer_program;

        let refund_ok = self.amount_to_be_refunded
            == self
                .overpayment
                .saturating_sub(self.add_lines_29_through_32);

        // Tax due and overpayment are mutually exclusive
        let mutual_exclusive_ok = self.tax_due == 0 || self.overpayment == 0;

        // Credits cannot exceed income tax
        let credits_ok = self.tax_credits <= self.nc_income_tax;

        // Part-year percentage in range
        let pct_ok = self.part_year_nonresident_taxable_pct <= 10_000;

        // Business rules: spouse fields
        let spouse_ok = match self.filing_status {
            FilingStatus::MarriedFilingJointly | FilingStatus::MarriedFilingSeparately => true,
            _ => self.spouse_name.is_empty() && self.spouse_ssn.is_empty(),
        };

        // Standard / itemized consistency
        let standard_itemized_ok = if self.line_11_is_standard {
            self.schedule_a.is_none()
        } else {
            true
        } && if self.schedule_a.is_some() {
            !self.line_11_is_standard
        } else {
            true
        };

        // Sub-form consistency
        let schedule_s_ok = self.schedule_s.as_ref().is_none_or(|s| {
            s.is_valid()
                && s.total_additions == self.additions_to_federal_agi
                && s.total_deductions == self.deductions_from_federal_agi
        });

        let schedule_a_ok = self.schedule_a.as_ref().is_none_or(|a| a.is_valid());
        let schedule_pn_ok = self.schedule_pn.as_ref().is_none_or(|pn| pn.is_valid());
        let schedule_pn1_ok = self
            .schedule_pn1
            .as_ref()
            .is_none_or(|pn1| pn1.is_valid());
        let d400tc_ok = self.d400tc.as_ref().is_none_or(|tc| tc.is_valid());
        let nc_nol_part1_ok = self
            .nc_nol_part1
            .as_ref()
            .is_none_or(|p1| p1.is_valid());
        let nc_nol_part2_ok = self
            .nc_nol_part2
            .as_ref()
            .is_none_or(|p2| p2.is_valid());
        let nc_rehab_ok = self.nc_rehab.as_ref().is_none_or(|r| r.is_valid());
        let nc_478_ok = self.nc_478.as_ref().is_none_or(|nc| nc.is_valid());

        line_8_ok
            && child_deduction_ok
            && line_12a_ok
            && line_12b_ok
            && nc_tax_ok
            && line_17_ok
            && line_19_ok
            && line_23_ok
            && line_25_ok
            && penalties_ok
            && amount_due_ok
            && line_33_ok
            && refund_ok
            && mutual_exclusive_ok
            && credits_ok
            && pct_ok
            && spouse_ok
            && standard_itemized_ok
            && schedule_s_ok
            && schedule_a_ok
            && schedule_pn_ok
            && schedule_pn1_ok
            && d400tc_ok
            && nc_nol_part1_ok
            && nc_nol_part2_ok
            && nc_rehab_ok
            && nc_478_ok
    }

    /// Compute the complete NC D-400 return from inputs.
    #[allow(clippy::field_reassign_with_default)]
    pub fn compute(input: &NcReturnInput) -> Self {
        let mut d = D400::default();

        // Header
        d.filing_status = input.filing_status;
        d.out_of_country = input.out_of_country;
        d.deceased_taxpayer = input.deceased_taxpayer;
        d.taxpayer_date_of_death = input.taxpayer_date_of_death.clone();
        d.spouse_date_of_death = input.spouse_date_of_death.clone();
        d.mailing_address = input.mailing_address.clone();
        d.apartment_number = input.apartment_number.clone();
        d.city = input.city.clone();
        d.state = input.state.clone();
        d.zip_code = input.zip_code.clone();
        d.country = input.country.clone();
        d.county = input.county;
        d.residency_status_taxpayer = input.residency_status_taxpayer;
        d.residency_status_spouse = input.residency_status_spouse;
        d.is_veteran = input.is_veteran;
        d.is_spouse_veteran = input.is_spouse_veteran;
        d.federal_extension = input.federal_extension;
        d.amended_return = input.amended_return;
        d.qualifying_widow_year_spouse_died = input.qualifying_widow_year_spouse_died.clone();
        d.spouse_name = input.spouse_name.clone();
        d.spouse_ssn = input.spouse_ssn.clone();

        // Step 1: NC-NOL Part 1 (worksheet, if provided)
        let nc_nol_part1 = input.nc_nol_part1_input.as_ref().map(NcNolPart1::compute);

        // Step 2: NC-NOL Part 2 → deduction for Schedule S Line 39
        let nc_nol_part2 = input.nc_nol_part2_input.as_ref().map(NcNolPart2::compute);
        let nc_nol_deduction = nc_nol_part2
            .as_ref()
            .map_or(0, |p2| p2.total_nc_nol_deduction);

        // Step 3: Compute Schedule S (with NC-NOL wired in)
        let schedule_s = ScheduleS::compute(&input.schedule_s_input, nc_nol_deduction);

        // Line 6: Federal AGI
        d.federal_agi = input.federal_return.federal_agi;

        // Step 4: Schedule PN-1 → Schedule PN
        let schedule_pn1 = input.schedule_pn1_input.as_ref().map(SchedulePn1::compute);
        let pn1_additions = schedule_pn1
            .as_ref()
            .map_or(TwoColumnLine::default(), |pn1| pn1.total_other_additions);
        let pn1_deductions = schedule_pn1
            .as_ref()
            .map_or(TwoColumnLine::default(), |pn1| pn1.total_other_deductions);

        let schedule_pn = input
            .schedule_pn_input
            .as_ref()
            .map(|pn_input| SchedulePn::compute(pn_input, pn1_additions, pn1_deductions));

        if let Some(ref pn) = schedule_pn {
            d.part_year_nonresident_taxable_pct = pn.taxable_percentage;
        }

        // Step 5: Compute Schedule A (or use standard deduction)
        d.line_11_is_standard = !input.use_nc_itemized_deductions;
        let schedule_a = if input.use_nc_itemized_deductions {
            let sa = ScheduleA::compute(&input.schedule_a_input, d.federal_agi);
            d.nc_standard_or_itemized_deduction = sa.total_nc_itemized_deductions;
            Some(sa)
        } else {
            d.nc_standard_or_itemized_deduction = nc_standard_deduction(
                input.filing_status,
                input.mfs_spouse_itemizes,
                input.federal_return.eligible_for_federal_standard_deduction,
            );
            None
        };

        // Step 6: Compute D-400 Lines 6–15
        // Line 7
        d.additions_to_federal_agi = schedule_s.total_additions;
        // Line 8
        d.add_lines_6_and_7 = d.federal_agi + d.additions_to_federal_agi;
        // Line 9
        d.deductions_from_federal_agi = schedule_s.total_deductions;
        // Line 10a/10b
        d.child_deduction_qualifying_children = input.child_deduction_qualifying_children;
        let per_child = child_deduction_per_child(d.filing_status, d.federal_agi);
        d.child_deduction_amount = per_child * (input.child_deduction_qualifying_children as u64);
        // Line 12a
        d.add_lines_9_10b_11 = d.deductions_from_federal_agi
            + d.child_deduction_amount
            + d.nc_standard_or_itemized_deduction;
        // Line 12b (zero floor)
        d.subtract_line_12a_from_line_8 = d.add_lines_6_and_7.saturating_sub(d.add_lines_9_10b_11);
        // Line 14: NC taxable income
        if d.part_year_nonresident_taxable_pct > 0 {
            d.nc_taxable_income = (d.subtract_line_12a_from_line_8
                * d.part_year_nonresident_taxable_pct as u64
                + 5_000)
                / 10_000;
        } else {
            d.nc_taxable_income = d.subtract_line_12a_from_line_8;
        }
        // Line 15: NC income tax = line 14 × 4.25%, rounded to nearest dollar
        d.nc_income_tax = (d.nc_taxable_income * NC_TAX_RATE_BPS + 5_000) / 10_000;

        // Step 7: NC-Rehab (if provided)
        let nc_rehab = input.nc_rehab_input.as_ref().map(NcRehab::compute);

        // Step 8: NC-478 (if provided)
        let nc_478 = input.nc_478_input.as_ref().map(Nc478::compute);

        // Step 9: Compute D-400TC
        let d400tc = D400Tc::compute(
            &input.d400tc_input,
            d.nc_income_tax,
            nc_rehab.as_ref(),
            nc_478.as_ref(),
        );

        // Step 10: D-400 Lines 16–34
        // Line 16
        d.tax_credits = d400tc.total_tax_credits;
        // Line 17
        d.subtract_line_16_from_15 = d.nc_income_tax.saturating_sub(d.tax_credits);
        // Line 18
        d.consumer_use_tax = input.consumer_use_tax;
        d.line_18_no_consumer_use_tax = input.no_consumer_use_tax;
        // Line 19
        d.add_lines_17_and_18 = d.subtract_line_16_from_15 + d.consumer_use_tax;

        // Lines 20–22: Payments
        d.nc_tax_withheld_taxpayer = input.nc_tax_withheld_taxpayer;
        d.nc_tax_withheld_spouse = input.nc_tax_withheld_spouse;
        d.estimated_tax_payments = input.estimated_tax_payments;
        d.paid_with_extension = input.paid_with_extension;
        d.partnership_payment = input.partnership_payment;
        d.s_corporation_payment = input.s_corporation_payment;
        d.additional_payments_amended = input.additional_payments_amended;

        // Line 23
        d.add_lines_20a_through_22 = [
            d.nc_tax_withheld_taxpayer,
            d.nc_tax_withheld_spouse,
            d.estimated_tax_payments,
            d.paid_with_extension,
            d.partnership_payment,
            d.s_corporation_payment,
            d.additional_payments_amended,
        ]
        .into_iter()
        .sum();

        // Line 24
        d.previous_refunds_amended = input.previous_refunds_amended;
        // Line 25
        d.subtract_line_24_from_23 = d
            .add_lines_20a_through_22
            .saturating_sub(d.previous_refunds_amended);

        // Lines 26–27: Tax due path
        if d.subtract_line_24_from_23 < d.add_lines_17_and_18 {
            d.tax_due = d.add_lines_17_and_18 - d.subtract_line_24_from_23;
        }
        d.penalties = input.penalties;
        d.interest = input.interest;
        d.penalties_plus_interest = d.penalties + d.interest;
        d.interest_underpayment_estimated = input.interest_underpayment_estimated;
        d.amount_due = d.tax_due + d.penalties_plus_interest + d.interest_underpayment_estimated;

        // Lines 28–34: Overpayment / refund path
        if d.subtract_line_24_from_23 > d.add_lines_17_and_18 {
            d.overpayment = d.subtract_line_24_from_23 - d.add_lines_17_and_18;
        }
        d.applied_to_estimated_tax = input.applied_to_estimated_tax;
        d.contribution_wildlife_fund = input.contribution_wildlife_fund;
        d.contribution_education_fund = input.contribution_education_fund;
        d.contribution_cancer_program = input.contribution_cancer_program;
        // Line 33
        d.add_lines_29_through_32 = d.applied_to_estimated_tax
            + d.contribution_wildlife_fund
            + d.contribution_education_fund
            + d.contribution_cancer_program;
        // Line 34 (zero floor)
        d.amount_to_be_refunded = d.overpayment.saturating_sub(d.add_lines_29_through_32);

        // Store computed sub-forms
        d.schedule_s = Some(schedule_s);
        d.schedule_a = schedule_a;
        d.schedule_pn = schedule_pn;
        d.schedule_pn1 = schedule_pn1;
        d.d400tc = Some(d400tc);
        d.nc_nol_part1 = nc_nol_part1;
        d.nc_nol_part2 = nc_nol_part2;
        d.nc_rehab = nc_rehab;
        d.nc_478 = nc_478;

        d
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    /// Helper: build a basic NcReturnInput with a given AGI and filing status.
    fn basic_input(agi: u64, filing_status: FilingStatus) -> NcReturnInput {
        NcReturnInput {
            filing_status,
            federal_return: FederalReturn {
                federal_agi: agi,
                eligible_for_federal_standard_deduction: true,
            },
            ..Default::default()
        }
    }

    #[test]
    fn simple_single_filer_standard_deduction() {
        let input = basic_input(75_000, FilingStatus::Single);
        let d = D400::compute(&input);

        assert_eq!(d.federal_agi, 75_000);
        assert_eq!(d.additions_to_federal_agi, 0);
        assert_eq!(d.deductions_from_federal_agi, 0);
        assert_eq!(d.nc_standard_or_itemized_deduction, 12_750);
        assert!(d.line_11_is_standard);
        // Taxable: 75000 - 0 - 0 - 12750 = 62250
        assert_eq!(d.nc_taxable_income, 62_250);
        // Tax: 62250 * 425 / 10000 = 2645.625 → rounded = 2646
        assert_eq!(d.nc_income_tax, 2_646);
    }

    #[test]
    fn mfj_with_children() {
        let mut input = basic_input(120_000, FilingStatus::MarriedFilingJointly);
        input.child_deduction_qualifying_children = 2;
        let d = D400::compute(&input);

        assert_eq!(d.nc_standard_or_itemized_deduction, 25_500);
        // AGI $120,000 MFJ → $1,000/child × 2 = $2,000
        assert_eq!(d.child_deduction_amount, 2_000);
        // Taxable: 120000 - 0 - 2000 - 25500 = 92500
        assert_eq!(d.nc_taxable_income, 92_500);
        // Tax: 92500 * 425 / 10000 = 3931.25 → rounded = 3931
        assert_eq!(d.nc_income_tax, 3_931);
    }

    #[test]
    fn withholding_causes_overpayment() {
        let mut input = basic_input(50_000, FilingStatus::Single);
        input.nc_tax_withheld_taxpayer = 5_000;
        let d = D400::compute(&input);

        // Taxable: 50000 - 12750 = 37250
        // Tax: 37250 * 425 / 10000 = 1583.125 → rounded = 1583
        assert_eq!(d.nc_income_tax, 1_583);
        // Total payments: 5000
        // Overpayment: 5000 - 1583 = 3417
        assert_eq!(d.overpayment, 3_417);
        assert_eq!(d.tax_due, 0);
    }

    #[test]
    fn underpayment_causes_tax_due() {
        let input = basic_input(100_000, FilingStatus::Single);
        let d = D400::compute(&input);

        // Taxable: 100000 - 12750 = 87250
        // Tax: 87250 * 425 / 10000 = 3708.125 → rounded = 3708
        assert_eq!(d.nc_income_tax, 3_708);
        assert_eq!(d.tax_due, 3_708);
        assert_eq!(d.overpayment, 0);
    }

    #[test]
    fn schedule_s_additions_and_deductions_flow_through() {
        let mut input = basic_input(80_000, FilingStatus::Single);
        input
            .schedule_s_input
            .interest_income_from_obligations_of_other_states = 1_000;
        input.schedule_s_input.interest_income_us_obligations = 500;
        let d = D400::compute(&input);

        assert_eq!(d.additions_to_federal_agi, 1_000);
        assert_eq!(d.deductions_from_federal_agi, 500);
        // Line 8: 80000 + 1000 = 81000
        assert_eq!(d.add_lines_6_and_7, 81_000);
        // Line 12a: 500 + 0 + 12750 = 13250
        assert_eq!(d.add_lines_9_10b_11, 13_250);
        // Taxable: 81000 - 13250 = 67750
        assert_eq!(d.nc_taxable_income, 67_750);
    }

    #[test]
    fn itemized_deduction_path() {
        let mut input = basic_input(100_000, FilingStatus::Single);
        input.use_nc_itemized_deductions = true;
        input.schedule_a_input.home_mortgage_interest = 10_000;
        input.schedule_a_input.real_estate_property_taxes = 5_000;
        input.schedule_a_input.charitable_contributions = 3_000;
        let d = D400::compute(&input);

        assert!(!d.line_11_is_standard);
        // Schedule A: min(15000, 20000) + 3000 + 0 + 0 + 0 = 18000
        assert_eq!(d.nc_standard_or_itemized_deduction, 18_000);
        // Taxable: 100000 - 18000 = 82000
        assert_eq!(d.nc_taxable_income, 82_000);
    }

    #[test]
    fn tax_credits_capped_at_tax() {
        let mut input = basic_input(50_000, FilingStatus::Single);
        input.d400tc_input.tax_credits_carried_from_previous_years = 999_999;
        let d = D400::compute(&input);

        // Credits cannot exceed NC income tax
        assert_eq!(d.tax_credits, d.nc_income_tax);
        assert_eq!(d.subtract_line_16_from_15, 0);
    }

    #[test]
    fn refund_reduced_by_contributions() {
        let mut input = basic_input(50_000, FilingStatus::Single);
        input.nc_tax_withheld_taxpayer = 5_000;
        input.contribution_wildlife_fund = 100;
        input.contribution_education_fund = 50;
        let d = D400::compute(&input);

        let expected_refund = d.overpayment - 150;
        assert_eq!(d.amount_to_be_refunded, expected_refund);
    }

    #[test]
    fn zero_income_results_in_zero_tax() {
        let input = basic_input(0, FilingStatus::Single);
        let d = D400::compute(&input);
        assert_eq!(d.nc_taxable_income, 0);
        assert_eq!(d.nc_income_tax, 0);
    }

    #[test]
    fn deductions_exceeding_income_give_zero_taxable() {
        let input = basic_input(10_000, FilingStatus::MarriedFilingJointly);
        // Standard deduction alone (25500) exceeds income
        let d = D400::compute(&input);
        assert_eq!(d.nc_taxable_income, 0);
        assert_eq!(d.nc_income_tax, 0);
    }

    #[test]
    fn nc_nol_wires_into_schedule_s_and_d400() {
        use crate::nc_nol::{NcNolCarryoverRow, NcNolPart2Input};

        let mut input = basic_input(80_000, FilingStatus::Single);
        // Provide NC-NOL Part 2 with a $5,000 deduction
        input.nc_nol_part2_input = Some(NcNolPart2Input {
            nc_nol_carryover: [
                NcNolCarryoverRow {
                    col_a: 5_000,
                    col_b: 5_000,
                },
                NcNolCarryoverRow::default(),
                NcNolCarryoverRow::default(),
            ],
            ..Default::default()
        });
        let d = D400::compute(&input);

        // NC-NOL deduction of 5000 should flow into Schedule S Line 39 → D-400 Line 9
        let schedule_s = d.schedule_s.as_ref().unwrap();
        assert_eq!(schedule_s.nc_net_operating_loss, 5_000);
        assert_eq!(d.deductions_from_federal_agi, 5_000);
        // Taxable: 80000 - 5000 - 0 - 12750 = 62250
        assert_eq!(d.nc_taxable_income, 62_250);
    }

    #[test]
    fn nc_rehab_wires_into_d400tc() {
        use crate::nc_rehab::{FranchiseIncome, NcRehabInput};

        let mut input = basic_input(100_000, FilingStatus::Single);
        input.nc_rehab_input = Some(NcRehabInput {
            total_qualified_rehabilitation_expenditures: 1_000_000,
            carryforwards_income_producing: FranchiseIncome {
                franchise: 0,
                income: 10_000,
            },
            total_rehabilitation_expenses_nonincome: 50_000,
            carryforwards_nonincome_producing: FranchiseIncome {
                franchise: 0,
                income: 5_000,
            },
            ..Default::default()
        });
        let d = D400::compute(&input);

        let tc = d.d400tc.as_ref().unwrap();
        let rehab = d.nc_rehab.as_ref().unwrap();
        // Line 8 = income-producing total including carryforwards
        assert_eq!(
            tc.income_producing_historic_structure_3d,
            rehab.line_23_total_income_producing.income
        );
        // Line 9 = nonincome-producing total including carryforwards
        assert_eq!(
            tc.nonincome_producing_historic_structure_3d,
            rehab.line_26_total_nonincome_producing.income
        );
    }

    #[test]
    fn nc_478_wires_into_d400tc_line_19() {
        use crate::nc_478::Nc478Input;
        use crate::nc_rehab::FranchiseIncome;

        let mut input = basic_input(100_000, FilingStatus::Single);
        input.nc_478_input = Some(Nc478Input {
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
        });
        let d = D400::compute(&input);

        let tc = d.d400tc.as_ref().unwrap();
        let nc478 = d.nc_478.as_ref().unwrap();
        // D-400TC Line 19 should equal NC-478 total_credits_taken.income
        assert_eq!(
            tc.business_incentive_energy_tax_credits,
            nc478.total_credits_taken.income
        );
        assert!(tc.business_incentive_energy_tax_credits > 0);
    }

    #[test]
    fn computed_d400_is_valid() {
        let input = basic_input(75_000, FilingStatus::Single);
        let d = D400::compute(&input);
        assert!(d.is_valid());
    }

    #[test]
    fn computed_d400_mfj_with_children_is_valid() {
        let mut input = basic_input(120_000, FilingStatus::MarriedFilingJointly);
        input.child_deduction_qualifying_children = 2;
        input.spouse_name = "Jane Doe".to_string();
        input.spouse_ssn = "123-45-6789".to_string();
        input.nc_tax_withheld_taxpayer = 5_000;
        let d = D400::compute(&input);
        assert!(d.is_valid());
    }

    #[test]
    fn computed_d400_itemized_is_valid() {
        let mut input = basic_input(100_000, FilingStatus::Single);
        input.use_nc_itemized_deductions = true;
        input.schedule_a_input.home_mortgage_interest = 10_000;
        input.schedule_a_input.real_estate_property_taxes = 5_000;
        input.schedule_a_input.charitable_contributions = 3_000;
        let d = D400::compute(&input);
        assert!(d.is_valid());
    }

    #[test]
    fn inconsistent_d400_is_invalid() {
        let mut d = D400::compute(&basic_input(50_000, FilingStatus::Single));
        d.add_lines_6_and_7 = 999;
        assert!(!d.is_valid());
    }

    #[test]
    fn d400_spouse_fields_invalid_for_single() {
        let mut d = D400::compute(&basic_input(50_000, FilingStatus::Single));
        d.spouse_name = "Unexpected".to_string();
        assert!(!d.is_valid());
    }

    #[test]
    fn schedule_pn_wires_taxable_percentage() {
        use crate::schedule_pn::{SchedulePnInput, TwoColumnLine};

        let mut input = basic_input(100_000, FilingStatus::Single);
        input.schedule_pn_input = Some(SchedulePnInput {
            wages_salaries_tips: TwoColumnLine {
                col_a: 100_000,
                col_b: 60_000,
            },
            ..Default::default()
        });
        let d = D400::compute(&input);

        assert_eq!(d.part_year_nonresident_taxable_pct, 6000); // 60%
        // Taxable income should be 60% of line 12b
        // Line 12b = 100000 - 12750 = 87250
        // Line 14 = (87250 * 6000 + 5000) / 10000 = 52350
        assert_eq!(d.nc_taxable_income, 52_350);
    }
}
