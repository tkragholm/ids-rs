use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
pub use types::models::{Covariate, CovariateType, CovariateValue, TimeVaryingValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovariateSummary {
    pub variable: String,
    pub mean_cases: f64,
    pub mean_controls: f64,
    pub std_diff: f64,
    pub variance_ratio: f64,
}

impl CovariateSummary {
    #[must_use] pub fn new(
        variable: String,
        mean_cases: f64,
        mean_controls: f64,
        std_diff: f64,
        variance_ratio: f64,
    ) -> Self {
        Self {
            variable,
            mean_cases,
            mean_controls,
            std_diff,
            variance_ratio,
        }
    }

    #[must_use] pub fn is_balanced(&self, threshold: f64) -> bool {
        self.std_diff.abs() <= threshold
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedPairDetail {
    pub case_pnr: String,
    pub control_pnrs: Vec<String>, // Changed from String to Vec<String>
    pub treatment_date: NaiveDate,
    pub variable: String,
    pub case_value: f64,
    pub control_value: f64,
    pub std_diff: f64,
}

impl MatchedPairDetail {
    #[must_use] pub fn new(
        case_pnr: String,
        control_pnrs: Vec<String>,
        treatment_date: NaiveDate,
        variable: String,
        case_value: f64,
        control_value: f64,
        std_diff: f64,
    ) -> Self {
        Self {
            case_pnr,
            control_pnrs,
            treatment_date,
            variable,
            case_value,
            control_value,
            std_diff,
        }
    }

    #[must_use] pub fn calculate_std_diff(case_value: f64, control_value: f64) -> f64 {
        let pooled_var = (case_value.powi(2) + control_value.powi(2)) / 2.0;
        if pooled_var == 0.0 {
            0.0
        } else {
            (case_value - control_value) / pooled_var.sqrt()
        }
    }
}
