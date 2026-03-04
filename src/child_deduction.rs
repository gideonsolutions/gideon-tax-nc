use us_tax_brackets::FilingStatus;

/// Returns the child deduction amount per qualifying child based on filing status
/// and federal adjusted gross income, per the NC D-400 Child Deduction Table.
pub fn child_deduction_per_child(filing_status: FilingStatus, federal_agi: u64) -> u64 {
    match filing_status {
        FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => {
            match federal_agi {
                0..=40_000 => 3_000,
                40_001..=60_000 => 2_500,
                60_001..=80_000 => 2_000,
                80_001..=100_000 => 1_500,
                100_001..=120_000 => 1_000,
                120_001..=140_000 => 500,
                _ => 0,
            }
        }
        FilingStatus::HeadOfHousehold => match federal_agi {
            0..=30_000 => 3_000,
            30_001..=45_000 => 2_500,
            45_001..=60_000 => 2_000,
            60_001..=75_000 => 1_500,
            75_001..=90_000 => 1_000,
            90_001..=105_000 => 500,
            _ => 0,
        },
        FilingStatus::Single | FilingStatus::MarriedFilingSeparately => match federal_agi {
            0..=20_000 => 3_000,
            20_001..=30_000 => 2_500,
            30_001..=40_000 => 2_000,
            40_001..=50_000 => 1_500,
            50_001..=60_000 => 1_000,
            60_001..=70_000 => 500,
            _ => 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // MFJ / QSS brackets
    #[test]
    fn mfj_under_40k() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingJointly, 40_000), 3_000);
    }

    #[test]
    fn mfj_40k_to_60k() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingJointly, 50_000), 2_500);
    }

    #[test]
    fn mfj_60k_to_80k() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingJointly, 70_000), 2_000);
    }

    #[test]
    fn mfj_80k_to_100k() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingJointly, 90_000), 1_500);
    }

    #[test]
    fn mfj_100k_to_120k() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingJointly, 110_000), 1_000);
    }

    #[test]
    fn mfj_120k_to_140k() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingJointly, 130_000), 500);
    }

    #[test]
    fn mfj_over_140k() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingJointly, 200_000), 0);
    }

    #[test]
    fn qss_same_as_mfj() {
        assert_eq!(
            child_deduction_per_child(FilingStatus::QualifyingSurvivingSpouse, 50_000),
            2_500
        );
    }

    // HOH brackets
    #[test]
    fn hoh_under_30k() {
        assert_eq!(child_deduction_per_child(FilingStatus::HeadOfHousehold, 30_000), 3_000);
    }

    #[test]
    fn hoh_30k_to_45k() {
        assert_eq!(child_deduction_per_child(FilingStatus::HeadOfHousehold, 35_000), 2_500);
    }

    #[test]
    fn hoh_45k_to_60k() {
        assert_eq!(child_deduction_per_child(FilingStatus::HeadOfHousehold, 50_000), 2_000);
    }

    #[test]
    fn hoh_60k_to_75k() {
        assert_eq!(child_deduction_per_child(FilingStatus::HeadOfHousehold, 70_000), 1_500);
    }

    #[test]
    fn hoh_75k_to_90k() {
        assert_eq!(child_deduction_per_child(FilingStatus::HeadOfHousehold, 80_000), 1_000);
    }

    #[test]
    fn hoh_90k_to_105k() {
        assert_eq!(child_deduction_per_child(FilingStatus::HeadOfHousehold, 100_000), 500);
    }

    #[test]
    fn hoh_over_105k() {
        assert_eq!(child_deduction_per_child(FilingStatus::HeadOfHousehold, 150_000), 0);
    }

    // Single / MFS brackets
    #[test]
    fn single_under_20k() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 20_000), 3_000);
    }

    #[test]
    fn single_20k_to_30k() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 25_000), 2_500);
    }

    #[test]
    fn single_30k_to_40k() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 35_000), 2_000);
    }

    #[test]
    fn single_40k_to_50k() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 45_000), 1_500);
    }

    #[test]
    fn single_50k_to_60k() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 55_000), 1_000);
    }

    #[test]
    fn single_60k_to_70k() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 65_000), 500);
    }

    #[test]
    fn single_over_70k() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 100_000), 0);
    }

    #[test]
    fn mfs_same_as_single() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingSeparately, 25_000), 2_500);
    }

    // Boundary tests
    #[test]
    fn mfj_exact_boundary_40001() {
        assert_eq!(child_deduction_per_child(FilingStatus::MarriedFilingJointly, 40_001), 2_500);
    }

    #[test]
    fn hoh_exact_boundary_30001() {
        assert_eq!(child_deduction_per_child(FilingStatus::HeadOfHousehold, 30_001), 2_500);
    }

    #[test]
    fn single_exact_boundary_20001() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 20_001), 2_500);
    }

    #[test]
    fn zero_agi() {
        assert_eq!(child_deduction_per_child(FilingStatus::Single, 0), 3_000);
    }
}
