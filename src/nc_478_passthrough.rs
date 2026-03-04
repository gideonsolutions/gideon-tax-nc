/// Code indicating type of pass-through entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PassthroughCode {
    /// P: Pass-through Entity
    #[default]
    PassthroughEntity,
    /// S: Successor Business
    SuccessorBusiness,
}

/// A single row in Part 1 (pass-through entity information).
#[derive(Debug, Clone, Default)]
pub struct PassthroughEntityRow {
    pub code: PassthroughCode,
    pub legal_name: String,
    pub fein: String,
}

/// NC-478 Pass-through — Pass-through Schedule for NC-478 Series.
///
/// This is an informational form with no computed lines.
#[derive(Debug, Clone, Default)]
pub struct Nc478Passthrough {
    // Header
    pub first_name: String,
    pub middle_initial: String,
    pub last_name: String,
    pub ssn: String,
    pub spouse_first_name: String,
    pub spouse_middle_initial: String,
    pub spouse_last_name: String,
    pub spouse_ssn: String,
    pub entity_legal_name: String,
    pub fein: String,

    // Part 1: Information for Pass-through Credit
    pub entities: Vec<PassthroughEntityRow>,

    // Part 2: Article 3J Tax Credit Election
    pub article_3j_tax_credit_election: u64,
}
