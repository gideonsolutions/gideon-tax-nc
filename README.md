# gideon-tax-nc

North Carolina individual income tax form computation for tax year 2025.

Written in Rust. All dollar amounts are whole-dollar `u64` values. Percentages are stored as basis points (0-10,000).

## Supported Forms

- **D-400** — NC Individual Income Tax Return
- **D-400TC** — Tax Credits
- **Schedule S** — Additions and Deductions to Federal AGI
- **Schedule A** — NC Itemized Deductions
- **Schedule PN** — Part-Year Resident and Nonresident Schedule
- **Schedule PN-1** — Other Additions and Other Deductions
- **Schedule AM** — Amended Return
- **NC-NOL** — Net Operating Loss (Parts 1 and 2)
- **NC-Rehab** — Historic Rehabilitation Tax Credits
- **NC-478** — Business Incentive and Energy Tax Credits
- **NC-478 Passthrough** — Pass-Through Entity Schedule

## Usage

```rust
use gideon_tax_nc::{D400, NcReturnInput, FederalReturn, FilingStatus};

let input = NcReturnInput {
    filing_status: FilingStatus::Single,
    federal_return: FederalReturn {
        federal_agi: 75_000,
        eligible_for_federal_standard_deduction: true,
    },
    ..Default::default()
};

let d400 = D400::compute(&input);
assert!(d400.is_valid());

println!("NC taxable income: ${}", d400.nc_taxable_income);
println!("NC income tax:     ${}", d400.nc_income_tax);
```

## License

Licensed under the [Christian Open Source License (COSL) v1.0](LICENSE.md).
