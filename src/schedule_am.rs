use crate::counties::County;

/// Reason(s) for amending the return.
#[derive(Debug, Clone, Default)]
pub struct AmendedReasons {
    pub federal_audit_change: bool,
    pub additional_income: bool,
    pub adjustments_to_schedule_a: bool,
    pub adjustments_to_schedule_s: bool,
    pub expenses_paid_with_forgiven_ppp_loan: bool,
    pub adjustments_to_schedule_pn_or_pn1: bool,
    pub tax_credits: bool,
    pub filing_status: bool,
    pub change_in_ssn_or_itin: bool,
    /// SSN or ITIN on original return (if change_in_ssn_or_itin is true).
    pub original_ssn_or_itin: String,
    pub military_spouse_residency_election: bool,
    pub original_return_previously_audited: bool,
    pub net_operating_loss: bool,
    pub injured_innocent_spouse: bool,
    pub tax_treaties: bool,
    pub unemployment_compensation_2020: bool,
    pub other: bool,
}

/// D-400 Schedule AM — North Carolina Amended Schedule.
///
/// This is a cover sheet for amended returns. It has no computed lines —
/// all fields are informational / input only.
#[derive(Debug, Clone, Default)]
pub struct ScheduleAm {
    // Header
    pub tax_year: u16,
    pub tax_year_beginning: String,
    pub tax_year_ending: String,
    pub ssn: String,
    pub spouse_ssn: String,
    pub first_name: String,
    pub middle_initial: String,
    pub last_name: String,
    pub spouse_first_name: String,
    pub spouse_middle_initial: String,
    pub spouse_last_name: String,
    pub mailing_address: String,
    pub address_is_change: bool,
    pub apartment_number: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub county: Option<County>,

    // Reasons
    pub reasons: AmendedReasons,

    // Explanation of changes (free-form text)
    pub explanation_of_changes: String,
}
