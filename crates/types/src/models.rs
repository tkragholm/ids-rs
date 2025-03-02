use chrono::NaiveDate;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

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
    
    pub fn get_wage_income(&self) -> Option<f64> {
        match &self.value {
            CovariateValue::Income { wage_income, .. } => *wage_income,
            _ => None,
        }
    }
    
    pub fn get_employment_status(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Income { employment_status, .. } => *employment_status,
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
    
    pub fn get_socio(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { socio, .. } => *socio,
            _ => None,
        }
    }
    
    pub fn get_socio02(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { socio02, .. } => *socio02,
            _ => None,
        }
    }
    
    pub fn get_pre_socio(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { pre_socio, .. } => *pre_socio,
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
    
    pub fn get_civil_status(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Demographics { civil_status, .. } => civil_status.clone(),
            _ => None,
        }
    }
    
    pub fn get_gender(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Demographics { gender, .. } => gender.clone(),
            _ => None,
        }
    }
    
    pub fn get_citizenship(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Demographics { citizenship, .. } => citizenship.clone(),
            _ => None,
        }
    }
    
    pub fn get_age(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { age, .. } => *age,
            _ => None,
        }
    }
    
    pub fn get_children_count(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { children_count, .. } => *children_count,
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
                wage_income: None,
                employment_status: None,
            },
            metadata: HashMap::new(),
        }
    }
    
    // Extended version with all income fields
    pub fn income_extended(
        amount: f64, 
        currency: String, 
        type_code: String,
        wage_income: Option<f64>,
        employment_status: Option<i32>,
    ) -> Self {
        Self {
            type_: CovariateType::Income,
            value: CovariateValue::Income {
                amount,
                currency,
                type_code,
                wage_income,
                employment_status,
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
                socio: None,
                socio02: None,
                pre_socio: None,
            },
            metadata: HashMap::new(),
        }
    }
    
    // Extended version with all occupation fields
    pub fn occupation_extended(
        code: String, 
        classification: String,
        socio: Option<i32>,
        socio02: Option<i32>,
        pre_socio: Option<i32>,
    ) -> Self {
        Self {
            type_: CovariateType::Occupation,
            value: CovariateValue::Occupation {
                code,
                classification,
                socio,
                socio02,
                pre_socio,
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
                civil_status: None,
                gender: None,
                citizenship: None,
                age: None,
                children_count: None,
            },
            metadata: HashMap::new(),
        }
    }
    
    /// Extended version with all demographic fields
    /// 
    /// # Arguments
    /// * `family_size` - The size of the family
    /// * `municipality` - The municipality code
    /// * `family_type` - The family type code
    /// * `demo_extras` - Additional demographic information (civil status, gender, citizenship, age, children count)
    pub fn demographics_with_extras(
        family_size: i32, 
        municipality: i32, 
        family_type: String,
        demo_extras: DemographicExtras,
    ) -> Self {
        Self {
            type_: CovariateType::Demographics,
            value: CovariateValue::Demographics {
                family_size,
                municipality,
                family_type,
                civil_status: demo_extras.civil_status,
                gender: demo_extras.gender,
                citizenship: demo_extras.citizenship,
                age: demo_extras.age,
                children_count: demo_extras.children_count,
            },
            metadata: HashMap::new(),
        }
    }
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
