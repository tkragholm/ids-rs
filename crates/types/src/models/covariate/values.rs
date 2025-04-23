use super::builders::*;
use super::types::{Covariate, CovariateType, CovariateValue};
use hashbrown::HashMap;

impl Covariate {
    /// Get the type of this covariate
    #[must_use]
    pub const fn type_(&self) -> CovariateType {
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
    pub fn income(
        amount: f64,
        currency: impl Into<String>,
        type_code: impl Into<String>,
    ) -> IncomeBuilder {
        IncomeBuilder {
            amount,
            currency: currency.into(),
            type_code: type_code.into(),
            wage_income: None,
            employment_status: None,
            metadata: HashMap::new(),
        }
    }

    /// Create an extended income covariate with wage and employment details
    #[must_use]
    pub fn income_extended(
        amount: f64,
        currency: impl Into<String>,
        type_code: impl Into<String>,
        wage_income: Option<f64>,
        employment_status: Option<i32>,
    ) -> IncomeBuilder {
        IncomeBuilder {
            amount,
            currency: currency.into(),
            type_code: type_code.into(),
            wage_income,
            employment_status,
            metadata: HashMap::new(),
        }
    }

    /// Create a new occupation covariate using the builder pattern
    #[must_use]
    pub fn occupation(
        code: impl Into<String>,
        classification: impl Into<String>,
    ) -> OccupationBuilder {
        OccupationBuilder {
            code: code.into(),
            classification: classification.into(),
            socio: None,
            socio02: None,
            pre_socio: None,
            metadata: HashMap::new(),
        }
    }

    /// Create an extended occupation covariate with additional socio classifications
    #[must_use]
    pub fn occupation_extended(
        code: impl Into<String>,
        classification: impl Into<String>,
        socio: Option<i32>,
        socio02: Option<i32>,
        pre_socio: Option<i32>,
    ) -> OccupationBuilder {
        OccupationBuilder {
            code: code.into(),
            classification: classification.into(),
            socio,
            socio02,
            pre_socio,
            metadata: HashMap::new(),
        }
    }

    /// Create a new demographics covariate using the builder pattern
    #[must_use]
    pub fn demographics(
        family_size: i32,
        municipality: i32,
        family_type: impl Into<String>,
    ) -> DemographicsBuilder {
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

    /// Create a demographics covariate with all demographic extras
    #[must_use]
    pub fn demographics_with_extras(
        family_size: i32,
        municipality: i32,
        family_type: impl Into<String>,
        extras: super::types::DemographicExtras,
    ) -> DemographicsBuilder {
        DemographicsBuilder {
            family_size,
            municipality,
            family_type: family_type.into(),
            civil_status: extras.civil_status,
            gender: extras.gender,
            citizenship: extras.citizenship,
            age: extras.age,
            children_count: extras.children_count,
            metadata: HashMap::new(),
        }
    }

    // Education accessors
    #[must_use]
    pub fn education_level(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Education { level, .. } => Some(level.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn isced_code(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Education { isced_code, .. } => isced_code.clone(),
            _ => None,
        }
    }

    #[must_use]

    pub const fn education_years(&self) -> Option<f32> {
        match &self.value {
            CovariateValue::Education { years, .. } => *years,
            _ => None,
        }
    }

    #[must_use]

    // Income accessors

    pub const fn income_amount(&self) -> Option<f64> {
        match &self.value {
            CovariateValue::Income { amount, .. } => Some(*amount),
            _ => None,
        }
    }

    #[must_use]

    pub fn currency(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Income { currency, .. } => Some(currency.clone()),
            _ => None,
        }
    }

    #[must_use]

    pub fn income_type_code(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Income { type_code, .. } => Some(type_code.clone()),
            _ => None,
        }
    }

    #[must_use]

    pub const fn wage_income(&self) -> Option<f64> {
        match &self.value {
            CovariateValue::Income { wage_income, .. } => *wage_income,
            _ => None,
        }
    }

    #[must_use]

    pub const fn employment_status(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Income {
                employment_status, ..
            } => *employment_status,
            _ => None,
        }
    }

    #[must_use]

    // Occupation accessors

    pub fn occupation_code(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Occupation { code, .. } => Some(code.clone()),
            _ => None,
        }
    }

    #[must_use]

    pub fn classification(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Occupation { classification, .. } => Some(classification.clone()),
            _ => None,
        }
    }

    #[must_use]

    pub const fn socio(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { socio, .. } => *socio,
            _ => None,
        }
    }

    #[must_use]

    pub const fn socio02(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { socio02, .. } => *socio02,
            _ => None,
        }
    }

    #[must_use]

    pub const fn pre_socio(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Occupation { pre_socio, .. } => *pre_socio,
            _ => None,
        }
    }

    #[must_use]

    // Demographics accessors

    pub const fn family_size(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { family_size, .. } => Some(*family_size),
            _ => None,
        }
    }

    #[must_use]

    pub const fn municipality(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { municipality, .. } => Some(*municipality),
            _ => None,
        }
    }

    #[must_use]

    pub fn family_type(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Demographics { family_type, .. } => Some(family_type.clone()),
            _ => None,
        }
    }

    #[must_use]

    pub fn civil_status(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Demographics { civil_status, .. } => civil_status.clone(),
            _ => None,
        }
    }

    #[must_use]

    pub fn gender(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Demographics { gender, .. } => gender.clone(),
            _ => None,
        }
    }

    #[must_use]

    pub fn citizenship(&self) -> Option<String> {
        match &self.value {
            CovariateValue::Demographics { citizenship, .. } => citizenship.clone(),
            _ => None,
        }
    }

    #[must_use]

    pub const fn age(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { age, .. } => *age,
            _ => None,
        }
    }

    #[must_use]

    pub const fn children_count(&self) -> Option<i32> {
        match &self.value {
            CovariateValue::Demographics { children_count, .. } => *children_count,
            _ => None,
        }
    }

    #[must_use]

    /// Add metadata to this covariate
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
