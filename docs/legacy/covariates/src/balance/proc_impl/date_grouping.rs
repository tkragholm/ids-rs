use super::super::BalanceChecker;
use chrono::NaiveDate;
use types::models::CovariateType;

/// Parameters for processing a chunk of data with date grouping
pub struct DateGroupingParams<'a, F, V> {
    pub chunk: &'a [(String, NaiveDate)],
    pub covariate_type: CovariateType,
    pub checker: &'a BalanceChecker,
    pub extractor: &'a F,
    pub values: &'a mut Vec<V>,
    pub missing: &'a mut usize,
    pub cache_hits: &'a mut usize,
    pub cache_misses: &'a mut usize,
}
