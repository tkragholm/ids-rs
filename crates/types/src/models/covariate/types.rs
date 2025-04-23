use hashbrown::HashMap;

#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

/// A covariate represents a variable that can be used for matching or analysis
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Covariate {
    pub type_: CovariateType,
    pub value: CovariateValue,
    pub metadata: HashMap<String, String>,
}

/// Types of covariates available in the system
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CovariateType {
    /// Educational background information
    Education,
    /// Income and wealth information
    Income,
    /// Occupational data
    Occupation,
    /// Demographic information (age, gender, etc.)
    Demographics,
}

/// The actual value of a covariate, with type-specific fields
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum CovariateValue {
    /// Education-related information
    Education {
        /// Education level description
        level: String,
        /// International Standard Classification of Education code
        isced_code: Option<String>,
        /// Years of education
        years: Option<f32>,
    },
    /// Income-related information
    Income {
        /// Income amount
        amount: f64,
        /// Currency code
        currency: String,
        /// Type of income
        type_code: String,
        /// Wage income (LOENMV_13)
        wage_income: Option<f64>,
        /// Employment status code (BESKST13)
        employment_status: Option<i32>,
    },
    /// Occupation-related information
    Occupation {
        /// Occupation code
        code: String,
        /// Classification system used
        classification: String,
        /// Socioeconomic status (SOCIO)
        socio: Option<i32>,
        /// Alternative socioeconomic status (SOCIO02)
        socio02: Option<i32>,
        /// Preliminary socioeconomic status (PRE_SOCIO)
        pre_socio: Option<i32>,
    },
    /// Demographic information
    Demographics {
        /// Number of people in the family
        family_size: i32,
        /// Municipality code
        municipality: i32,
        /// Family type description
        family_type: String,
        /// Civil status (CIVST)
        civil_status: Option<String>,
        /// Gender (KOEN)
        gender: Option<String>,
        /// Citizenship (STATSB)
        citizenship: Option<String>,
        /// Age in years (ALDER)
        age: Option<i32>,
        /// Number of children (ANTBOERNF/ANTBOERNH)
        children_count: Option<i32>,
    },
}

/// Container for additional demographic information to avoid clippy warnings
/// about too many arguments
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct DemographicExtras {
    /// Civil status (CIVST)
    pub civil_status: Option<String>,
    /// Gender (KOEN)
    pub gender: Option<String>,
    /// Citizenship (STATSB)
    pub citizenship: Option<String>,
    /// Age in years (ALDER)
    pub age: Option<i32>,
    /// Number of children (ANTBOERNF/ANTBOERNH)
    pub children_count: Option<i32>,
}
