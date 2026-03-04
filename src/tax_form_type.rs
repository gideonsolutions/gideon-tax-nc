/// Tax form type filed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaxFormType {
    #[default]
    Individual,
    EstateTrust,
    CCorp,
    SCorp,
    Insurance,
    Partnership,
}
