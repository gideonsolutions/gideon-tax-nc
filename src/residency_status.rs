/// Residency status for a taxpayer or spouse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResidencyStatus {
    #[default]
    FullYearResident,
    Nonresident,
    PartYearResident,
}
