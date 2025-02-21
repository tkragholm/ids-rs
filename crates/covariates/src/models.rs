pub use types::models::{Covariate, CovariateType, CovariateValue, TimeVaryingValue};

// pub trait EducationExt {
//     fn to_numeric_value(&self) -> Option<f64>;
// }

// impl EducationExt for Education {
//     fn to_numeric_value(&self) -> Option<f64> {
//         self.years.map(f64::from)
//     }
// }

#[derive(Debug, Clone)]
pub struct CovariateSummary {
    pub variable: String,
    pub mean_cases: f64,
    pub mean_controls: f64,
    pub std_diff: f64,
    pub variance_ratio: f64,
}
