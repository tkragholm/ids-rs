use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeVaryingValue<T> {
    pub pnr: String,
    pub value: T,
    pub date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Covariate {
    pub type_: CovariateType,
    pub value: CovariateValue,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CovariateValue {
    Education {
        level: String,
        isced_code: Option<String>,
        years: Option<f32>,
    },
    Income {
        amount: f64,
        currency: String,
        type_code: String,
    },
    Occupation {
        code: String,
        classification: String,
    },
    Demographics {
        family_size: i32,
        municipality: i32,
        family_type: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CovariateType {
    Education,
    Income,
    Occupation,
    Demographics,
}

impl Covariate {
    pub fn get_type(&self) -> CovariateType {
        self.type_
    }

    pub fn education(level: String, isced_code: Option<String>, years: Option<f32>) -> Self {
        Self {
            type_: CovariateType::Education,
            value: CovariateValue::Education {
                level,
                isced_code,
                years,
            },
            metadata: HashMap::new(),
        }
    }

    pub fn income(amount: f64, currency: String, type_code: String) -> Self {
        Self {
            type_: CovariateType::Income,
            value: CovariateValue::Income {
                amount,
                currency,
                type_code,
            },
            metadata: HashMap::new(),
        }
    }

    pub fn occupation(code: String, classification: String) -> Self {
        Self {
            type_: CovariateType::Occupation,
            value: CovariateValue::Occupation {
                code,
                classification,
            },
            metadata: HashMap::new(),
        }
    }

    pub fn demographics(family_size: i32, municipality: i32, family_type: String) -> Self {
        Self {
            type_: CovariateType::Demographics,
            value: CovariateValue::Demographics {
                family_size,
                municipality,
                family_type,
            },
            metadata: HashMap::new(),
        }
    }
}
