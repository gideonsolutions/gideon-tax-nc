/// Minimal federal return data needed by state tax computations.
#[derive(Debug, Clone, Copy, Default)]
pub struct FederalReturn {
    /// Federal adjusted gross income (whole dollars).
    pub federal_agi: u64,
    /// Whether the taxpayer is eligible for the federal standard deduction.
    pub eligible_for_federal_standard_deduction: bool,
}
