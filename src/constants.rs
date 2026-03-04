// ── D-400 ────────────────────────────────────────────────────────────────────

/// NC income tax rate for 2025: 4.25% expressed as basis points (÷ 10_000).
pub(crate) const NC_TAX_RATE_BPS: u64 = 425;

// ── Schedule A ───────────────────────────────────────────────────────────────

/// Home mortgage + property tax cap (Schedule A Line 4).
pub(crate) const MORTGAGE_PROPERTY_TAX_LIMIT: u64 = 20_000;

/// Medical/dental expense threshold: 7.5% of federal AGI (basis points).
pub(crate) const MEDICAL_THRESHOLD_BPS: u64 = 750;

/// NC standard deduction amounts for tax year 2025.
pub(crate) const STANDARD_DEDUCTION_SINGLE: u64 = 12_750;
pub(crate) const STANDARD_DEDUCTION_HOH: u64 = 19_125;
pub(crate) const STANDARD_DEDUCTION_MFJ: u64 = 25_500;
pub(crate) const STANDARD_DEDUCTION_QSS: u64 = 25_500;
pub(crate) const STANDARD_DEDUCTION_MFS: u64 = 12_750;

// ── NC-NOL ───────────────────────────────────────────────────────────────────

/// Capital loss limitation for most filing statuses (NC-NOL Line 19).
pub(crate) const CAPITAL_LOSS_LIMIT: u64 = 3_000;

/// Capital loss limitation for Married Filing Separately (NC-NOL Line 19).
pub(crate) const CAPITAL_LOSS_LIMIT_MFS: u64 = 1_500;

// ── NC-Rehab ─────────────────────────────────────────────────────────────────

/// Maximum qualified rehabilitation expenditures for income-producing (Part 2, Line 2).
pub(crate) const MAX_EXPENDITURES_INCOME_PRODUCING: u64 = 20_000_000;

/// Tier threshold for 15% rate (Part 2, Line 4).
pub(crate) const TIER_THRESHOLD: u64 = 10_000_000;

/// Maximum credit for income-producing historic structure (Part 2, Line 15).
pub(crate) const MAX_CREDIT_INCOME_PRODUCING: u64 = 4_500_000;

/// Maximum expenses for nonincome-producing historic structure (Part 3, Line 18).
pub(crate) const MAX_EXPENSES_NONINCOME_PRODUCING: u64 = 150_000;
