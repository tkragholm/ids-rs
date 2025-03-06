use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// A covariate represents a variable that can be used for matching or analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Covariate {
    pub type_: CovariateType,
    pub value: CovariateValue,
    pub metadata: HashMap<String, String>,
}

/// Types of covariates available in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CovariateType {
    Education,
    Income,
    Occupation,
    Demographics,
}

/// The actual value of a covariate, with type-specific fields
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
        wage_income: Option<f64>,       // LOENMV_13
        employment_status: Option<i32>, // BESKST13
    },
    Occupation {
        code: String,
        classification: String,
        socio: Option<i32>,     // SOCIO
        socio02: Option<i32>,   // SOCIO02
        pre_socio: Option<i32>, // PRE_SOCIO
    },
    Demographics {
        family_size: i32,
        municipality: i32,
        family_type: String,
        civil_status: Option<String>, // CIVST
        gender: Option<String>,       // KOEN
        citizenship: Option<String>,  // STATSB
        age: Option<i32>,             // ALDER
        children_count: Option<i32>,  // ANTBOERNF/ANTBOERNH
    },
}

/// Container for additional demographic information to avoid clippy warnings
/// about too many arguments
#[derive(Debug, Clone, Default)]
pub struct DemographicExtras {
    pub civil_status: Option<String>,
    pub gender: Option<String>,
    pub citizenship: Option<String>,
    pub age: Option<i32>,
    pub children_count: Option<i32>,
}