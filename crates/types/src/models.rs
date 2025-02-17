use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeVaryingValue<T> {
    pub pnr: String,
    pub value: T,
    pub date: NaiveDate,
}

// Core data models
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demographics {
    pub family_size: i32,
    pub municipality: i32,
    pub family_type: String,
}
