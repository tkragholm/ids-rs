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

    // Education accessors
    pub fn get_education_level(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Education { level, .. } => Some(level.clone()),
            _ => None,
        }
    }

    pub fn get_isced_code(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Education { isced_code, .. } => isced_code.clone(),
            _ => None,
        }
    }

    pub fn get_education_years(&self) -> Option<f32> {
        match &self.value {
            CovariateValue::Education { years, .. } => *years,
            _ => None,
        }
    }

    // Income accessors
    pub fn get_income_amount(&self) -> Option<f64> {
        match &self.value {
            CovariateValue::Income { amount, .. } => Some(*amount),
            _ => None,
        }
    }

    pub fn get_currency(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Income { currency, .. } => Some(currency.clone()),
            _ => None,
        }
    }

    pub fn get_income_type_code(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Income { type_code, .. } => Some(type_code.clone()),
            _ => None,
        }
    }

    // Occupation accessors
    pub fn get_occupation_code(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Occupation { code, .. } => Some(code.clone()),
            _ => None,
        }
    }

    pub fn get_classification(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Occupation { classification, .. } => Some(classification.clone()),
            _ => None,
        }
    }

    // Demographics accessors
    pub fn get_family_size(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { family_size, .. } => Some(*family_size),
            _ => None,
        }
    }

    pub fn get_municipality(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { municipality, .. } => Some(*municipality),
            _ => None,
        }
    }

    pub fn get_family_type(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Demographics { family_type, .. } => Some(family_type.clone()),
            _ => None,
        }
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
