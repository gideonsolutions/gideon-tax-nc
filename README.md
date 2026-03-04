# gideon-tax-nc

North Carolina individual income tax form computation for tax year 2025.

Written in Rust. All dollar amounts are whole-dollar `u64` values. Percentages are stored as basis points (0-10,000).

## Supported Forms

### D-400 — NC Individual Income Tax Return
The main return form. `D400::compute()` orchestrates all sub-forms and produces a complete return including:
- Federal AGI through NC taxable income (Lines 6-14)
- NC income tax at 4.25% (Line 15)
- Tax credits, consumer use tax, payments, penalties, and interest (Lines 16-27)
- Overpayment, voluntary contributions, and refund (Lines 28-34)
- Full-year, part-year, and nonresident filing

### D-400TC — Tax Credits
- Credit for income tax paid to another state (Part 1)
- Historic rehabilitation structure credits (Part 2)
- Business incentive and energy tax credits from NC-478 (Part 3)
- Total tax credits capped at NC income tax

### Schedule S — Additions and Deductions
- 14 addition lines (Lines 1-14) including bonus depreciation, IRC 179, federal NOL addback
- 23 deduction lines (Lines 17-39) including Social Security, Bailey settlement, NC NOL
- Bonus depreciation and IRC 179 five-year breakouts (Lines 23a-23e, 24a-24e)
- Pass-through entity income (NC-sourced and non-NC-sourced)

### Schedule A — NC Itemized Deductions
- Home mortgage interest and real estate property taxes (capped at $20,000)
- Charitable contributions
- Medical and dental expenses (7.5% AGI threshold)
- Claim-of-right repayment

### Schedule PN — Part-Year Resident and Nonresident
- Two-column income allocation (total vs. NC-attributable) across 15 income lines
- NC adjustment additions (Lines 17a-17e) and deductions (Lines 19a-19h)
- Taxable percentage calculation (Line 24)

### Schedule PN-1 — Other Additions and Other Deductions
- 10 other addition lines and 16 other deduction lines (two-column)
- Flows into Schedule PN Lines 17e and 19h

### Schedule AM — Amended Return
- Cover sheet for amended returns (informational only, no computed lines)

### NC-NOL — Net Operating Loss
- **Part 1**: NOL calculation worksheet with nonbusiness/business capital gain/loss handling, IRC 1202 exclusion, and capital loss limitations ($3,000 / $1,500 MFS)
- **Part 2**: Federal NOL carryover (12 years, 2010-2021) and NC NOL carryover (3 years, 2022-2024) with deduction computation

### NC-Rehab — Historic Rehabilitation Tax Credits
- Income-producing structure credits: tiered 15%/10% rates, development tier and targeted investment bonuses (5% each), capped at $4,500,000
- Nonincome-producing structure credits: 15% of expenses capped at $150,000
- Franchise/income split with carryforward support

### NC-478 — Business Incentive and Energy Tax Credits
- 8 credit categories (R&D, renewable energy, technology commercialization, etc.)
- 50% tax limit computation with franchise/income dual-column tracking
- Credit-taken totals capped at allowed amount

### NC-478 Passthrough
- Pass-through entity information schedule (informational only, no computed lines)

## Other Features

### Child Deduction Table
AGI-dependent and filing-status-dependent deduction per qualifying child:

| Filing Status | Max AGI for $3,000 | Phase-out to $0 |
|---|---|---|
| MFJ / QSS | $40,000 | $140,000 |
| HOH | $30,000 | $105,000 |
| Single / MFS | $20,000 | $70,000 |

### Validation
Every computed form struct has an `is_valid()` method that verifies arithmetic consistency and business rules. Forms produced by `compute()` always return `true`; manually constructed forms can be checked for correctness.

### NC Counties
All 100 North Carolina counties with display formatting.

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
