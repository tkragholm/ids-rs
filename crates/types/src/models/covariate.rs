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

impl Covariate {
    /// Get the type of this covariate
    #[must_use]
    pub const fn get_type(&self) -> CovariateType {
        self.type_
    }

    /// Create a new education covariate using the builder pattern
    #[must_use]
    pub fn education(level: impl Into<String>) -> EducationBuilder {
        EducationBuilder {
            level: level.into(),
            isced_code: None,
            years: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a new income covariate using the builder pattern
    #[must_use]
    pub fn income(amount: f64, currency: impl Into<String>, type_code: impl Into<String>) -> IncomeBuilder {
        IncomeBuilder {
            amount,
            currency: currency.into(),
            type_code: type_code.into(),
            wage_income: None,
            employment_status: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a new occupation covariate using the builder pattern
    #[must_use]
    pub fn occupation(code: impl Into<String>, classification: impl Into<String>) -> OccupationBuilder {
        OccupationBuilder {
            code: code.into(),
            classification: classification.into(),
            socio: None,
            socio02: None,
            pre_socio: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a new demographics covariate using the builder pattern
    #[must_use]
    pub fn demographics(family_size: i32, municipality: i32, family_type: impl Into<String>) -> DemographicsBuilder {
        DemographicsBuilder {
            family_size,
            municipality,
            family_type: family_type.into(),
            civil_status: None,
            gender: None,
            citizenship: None,
            age: None,
            children_count: None,
            metadata: HashMap::new(),
        }
    }

    // Education accessors
    #[must_use]
    pub fn get_education_level(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Education { level, .. } => Some(level),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_isced_code(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Education { isced_code, .. } => isced_code.as_deref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn get_education_years(&self) -> Option<f32> {
        match &self.value {
            CovariateValue::Education { years, .. } => *years,
            _ => None,
        }
    }

    // Income accessors
    #[must_use]
    pub const fn get_income_amount(&self) -> Option<f64> {
        match &self.value {
            CovariateValue::Income { amount, .. } => Some(*amount),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_currency(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Income { currency, .. } => Some(currency),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_income_type_code(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Income { type_code, .. } => Some(type_code),
            _ => None,
        }
    }
    
    #[must_use]
    pub const fn get_wage_income(&self) -> Option<f64> {
        match &self.value {
            CovariateValue::Income { wage_income, .. } => *wage_income,
            _ => None,
        }
    }
    
    #[must_use]
    pub const fn get_employment_status(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Income { employment_status, .. } => *employment_status,
            _ => None,
        }
    }

    // Occupation accessors
    #[must_use]
    pub fn get_occupation_code(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Occupation { code, .. } => Some(code),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_classification(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Occupation { classification, .. } => Some(classification),
            _ => None,
        }
    }
    
    #[must_use]
    pub const fn get_socio(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { socio, .. } => *socio,
            _ => None,
        }
    }
    
    #[must_use]
    pub const fn get_socio02(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { socio02, .. } => *socio02,
            _ => None,
        }
    }
    
    #[must_use]
    pub const fn get_pre_socio(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { pre_socio, .. } => *pre_socio,
            _ => None,
        }
    }

    // Demographics accessors
    #[must_use]
    pub const fn get_family_size(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { family_size, .. } => Some(*family_size),
            _ => None,
        }
    }

    #[must_use]
    pub const fn get_municipality(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { municipality, .. } => Some(*municipality),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_family_type(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Demographics { family_type, .. } => Some(family_type),
            _ => None,
        }
    }
    
    #[must_use]
    pub fn get_civil_status(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Demographics { civil_status, .. } => civil_status.as_deref(),
            _ => None,
        }
    }
    
    #[must_use]
    pub fn get_gender(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Demographics { gender, .. } => gender.as_deref(),
            _ => None,
        }
    }
    
    #[must_use]
    pub fn get_citizenship(&self) -> Option<&str> {
        match &self.value {
            CovariateValue::Demographics { citizenship, .. } => citizenship.as_deref(),
            _ => None,
        }
    }
    
    #[must_use]
    pub const fn get_age(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { age, .. } => *age,
            _ => None,
        }
    }
    
    #[must_use]
    pub const fn get_children_count(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { children_count, .. } => *children_count,
            _ => None,
        }
    }

    /// Add metadata to this covariate
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

// Builder for education covariates
pub struct EducationBuilder {
    level: String,
    isced_code: Option<String>,
    years: Option<f32>,
    metadata: HashMap<String, String>,
}

impl EducationBuilder {
    /// Add an ISCED code to this education covariate
    pub fn with_isced_code(mut self, code: impl Into<String>) -> Self {
        self.isced_code = Some(code.into());
        self
    }

    /// Add years of education to this education covariate
    pub fn with_years(mut self, years: f32) -> Self {
        self.years = Some(years);
        self
    }

    /// Add metadata to this education covariate
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the education covariate
    pub fn build(self) -> Covariate {
        Covariate {
            type_: CovariateType::Education,
            value: CovariateValue::Education {
                level: self.level,
                isced_code: self.isced_code,
                years: self.years,
            },
            metadata: self.metadata,
        }
    }
}

// Builder for income covariates
pub struct IncomeBuilder {
    amount: f64,
    currency: String,
    type_code: String,
    wage_income: Option<f64>,
    employment_status: Option<i32>,
    metadata: HashMap<String, String>,
}

impl IncomeBuilder {
    /// Add wage income to this income covariate
    pub fn with_wage_income(mut self, wage_income: f64) -> Self {
        self.wage_income = Some(wage_income);
        self
    }

    /// Add employment status to this income covariate
    pub fn with_employment_status(mut self, status: i32) -> Self {
        self.employment_status = Some(status);
        self
    }

    /// Add metadata to this income covariate
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the income covariate
    pub fn build(self) -> Covariate {
        Covariate {
            type_: CovariateType::Income,
            value: CovariateValue::Income {
                amount: self.amount,
                currency: self.currency,
                type_code: self.type_code,
                wage_income: self.wage_income,
                employment_status: self.employment_status,
            },
            metadata: self.metadata,
        }
    }
}

// Builder for occupation covariates
pub struct OccupationBuilder {
    code: String,
    classification: String,
    socio: Option<i32>,
    socio02: Option<i32>,
    pre_socio: Option<i32>,
    metadata: HashMap<String, String>,
}

impl OccupationBuilder {
    /// Add socio code to this occupation covariate
    pub fn with_socio(mut self, socio: i32) -> Self {
        self.socio = Some(socio);
        self
    }

    /// Add socio02 code to this occupation covariate
    pub fn with_socio02(mut self, socio02: i32) -> Self {
        self.socio02 = Some(socio02);
        self
    }

    /// Add pre_socio code to this occupation covariate
    pub fn with_pre_socio(mut self, pre_socio: i32) -> Self {
        self.pre_socio = Some(pre_socio);
        self
    }

    /// Add metadata to this occupation covariate
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the occupation covariate
    pub fn build(self) -> Covariate {
        Covariate {
            type_: CovariateType::Occupation,
            value: CovariateValue::Occupation {
                code: self.code,
                classification: self.classification,
                socio: self.socio,
                socio02: self.socio02,
                pre_socio: self.pre_socio,
            },
            metadata: self.metadata,
        }
    }
}

// Builder for demographics covariates
pub struct DemographicsBuilder {
    family_size: i32,
    municipality: i32,
    family_type: String,
    civil_status: Option<String>,
    gender: Option<String>,
    citizenship: Option<String>,
    age: Option<i32>,
    children_count: Option<i32>,
    metadata: HashMap<String, String>,
}

impl DemographicsBuilder {
    /// Add civil status to this demographics covariate
    pub fn with_civil_status(mut self, status: impl Into<String>) -> Self {
        self.civil_status = Some(status.into());
        self
    }

    /// Add gender to this demographics covariate
    pub fn with_gender(mut self, gender: impl Into<String>) -> Self {
        self.gender = Some(gender.into());
        self
    }

    /// Add citizenship to this demographics covariate
    pub fn with_citizenship(mut self, citizenship: impl Into<String>) -> Self {
        self.citizenship = Some(citizenship.into());
        self
    }

    /// Add age to this demographics covariate
    pub fn with_age(mut self, age: i32) -> Self {
        self.age = Some(age);
        self
    }

    /// Add children count to this demographics covariate
    pub fn with_children_count(mut self, count: i32) -> Self {
        self.children_count = Some(count);
        self
    }

    /// Add all demographic extras at once
    pub fn with_extras(mut self, extras: DemographicExtras) -> Self {
        self.civil_status = extras.civil_status;
        self.gender = extras.gender;
        self.citizenship = extras.citizenship;
        self.age = extras.age;
        self.children_count = extras.children_count;
        self
    }

    /// Add metadata to this demographics covariate
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the demographics covariate
    pub fn build(self) -> Covariate {
        Covariate {
            type_: CovariateType::Demographics,
            value: CovariateValue::Demographics {
                family_size: self.family_size,
                municipality: self.municipality,
                family_type: self.family_type,
                civil_status: self.civil_status,
                gender: self.gender,
                citizenship: self.citizenship,
                age: self.age,
                children_count: self.children_count,
            },
            metadata: self.metadata,
        }
    }
}