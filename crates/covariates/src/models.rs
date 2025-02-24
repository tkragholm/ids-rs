use chrono::NaiveDate;
pub use types::models::{Covariate, CovariateType, CovariateValue, TimeVaryingValue};

#[derive(Debug, Clone)]
pub struct CovariateSummary {
    pub variable: String,
    pub mean_cases: f64,
    pub mean_controls: f64,
    pub std_diff: f64,
    pub variance_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct MatchedPairDetail {
    pub case_pnr: String,
    pub control_pnrs: String,
    pub treatment_date: NaiveDate,
    pub variable: String,
    pub case_value: f64,
    pub control_value: f64,
    pub std_diff: f64,
}
