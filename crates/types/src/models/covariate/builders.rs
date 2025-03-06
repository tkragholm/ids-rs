use hashbrown::HashMap;
use super::types::{Covariate, CovariateType, CovariateValue, DemographicExtras};

// Builder for education covariates
#[derive(Clone)]
pub struct EducationBuilder {
    pub(crate) level: String,
    pub(crate) isced_code: Option<String>,
    pub(crate) years: Option<f32>,
    pub(crate) metadata: HashMap<String, String>,
}

impl EducationBuilder {
    /// Create a new builder instance
    #[must_use]
    pub fn new(level: impl Into<String>) -> Self {
        Self {
            level: level.into(),
            isced_code: None,
            years: None,
            metadata: HashMap::new(),
        }
    }

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
#[derive(Clone)]
pub struct IncomeBuilder {
    pub(crate) amount: f64,
    pub(crate) currency: String,
    pub(crate) type_code: String,
    pub(crate) wage_income: Option<f64>,
    pub(crate) employment_status: Option<i32>,
    pub(crate) metadata: HashMap<String, String>,
}

impl IncomeBuilder {
    /// Create a new builder instance
    #[must_use]
    pub fn new(amount: f64, currency: impl Into<String>, type_code: impl Into<String>) -> Self {
        Self {
            amount,
            currency: currency.into(),
            type_code: type_code.into(),
            wage_income: None,
            employment_status: None,
            metadata: HashMap::new(),
        }
    }

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
#[derive(Clone)]
pub struct OccupationBuilder {
    pub(crate) code: String,
    pub(crate) classification: String,
    pub(crate) socio: Option<i32>,
    pub(crate) socio02: Option<i32>,
    pub(crate) pre_socio: Option<i32>,
    pub(crate) metadata: HashMap<String, String>,
}

impl OccupationBuilder {
    /// Create a new builder instance
    #[must_use]
    pub fn new(code: impl Into<String>, classification: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            classification: classification.into(),
            socio: None,
            socio02: None,
            pre_socio: None,
            metadata: HashMap::new(),
        }
    }

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
#[derive(Clone)]
pub struct DemographicsBuilder {
    pub(crate) family_size: i32,
    pub(crate) municipality: i32,
    pub(crate) family_type: String,
    pub(crate) civil_status: Option<String>,
    pub(crate) gender: Option<String>,
    pub(crate) citizenship: Option<String>,
    pub(crate) age: Option<i32>,
    pub(crate) children_count: Option<i32>,
    pub(crate) metadata: HashMap<String, String>,
}

impl DemographicsBuilder {
    /// Create a new builder instance
    #[must_use]
    pub fn new(family_size: i32, municipality: i32, family_type: impl Into<String>) -> Self {
        Self {
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