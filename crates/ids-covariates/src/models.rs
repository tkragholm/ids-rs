use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeVaryingValue<T> {
    pub pnr: String,
    pub value: T,
    pub date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub level: String,
    pub isced_code: Option<String>,
    pub years: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Income {
    pub amount: f64,
    pub currency: String,
    pub type_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occupation {
    pub code: String,
    pub classification: String,
}

#[derive(Debug, Clone)]
pub struct PersonCovariates {
    pub pnr: String,
    pub education: Vec<TimeVaryingValue<Education>>,
    pub income: Vec<TimeVaryingValue<Income>>,
    pub occupation: Vec<TimeVaryingValue<Occupation>>,
}

#[derive(Debug, Clone)]
pub struct CovariateSummary {
    pub variable: String,
    pub mean_cases: f64,
    pub mean_controls: f64,
    pub std_diff: f64,
    pub variance_ratio: f64,
}

impl<T> TimeVaryingValue<T> {
    pub fn at_or_before_date(&self, date: NaiveDate) -> bool {
        self.date <= date
    }
}

impl Education {
    pub fn to_numeric_value(&self) -> Option<f64> {
        self.years.map(f64::from)
    }
}

impl Income {
    pub fn to_numeric_value(&self) -> f64 {
        self.amount
    }
}

impl Occupation {
    pub fn to_categorical_value(&self) -> &str {
        &self.code
    }
}
