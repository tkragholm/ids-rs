use crate::models::{Education, Income, Occupation};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovariateSnapshot {
    pub date: NaiveDate,
    // Personal characteristics
    pub income: Option<Income>,
    pub education: Option<Education>,
    pub socioeconomic_status: Option<Occupation>,
    // Family characteristics
    pub family_size: Option<i32>,
    pub municipality: Option<i32>,
    pub family_type: Option<String>,
    pub immigrant_background: Option<String>,
    // Parent characteristics
    pub father_income: Option<Income>,
    pub father_education: Option<Education>,
    pub father_socioeconomic_status: Option<Occupation>,
    pub mother_income: Option<Income>,
    pub mother_education: Option<Education>,
    pub mother_socioeconomic_status: Option<Occupation>,
}

impl CovariateSnapshot {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            date,
            income: None,
            education: None,
            socioeconomic_status: None,
            family_size: None,
            municipality: None,
            family_type: None,
            immigrant_background: None,
            father_income: None,
            father_education: None,
            father_socioeconomic_status: None,
            mother_income: None,
            mother_education: None,
            mother_socioeconomic_status: None,
        }
    }

    pub fn with_education(mut self, education: Option<Education>) -> Self {
        self.education = education;
        self
    }

    pub fn with_income(mut self, income: Option<Income>) -> Self {
        self.income = income;
        self
    }

    pub fn with_socioeconomic_status(mut self, status: Option<Occupation>) -> Self {
        self.socioeconomic_status = status;
        self
    }

    pub fn combine(
        person: CovariateSnapshot,
        father: Option<CovariateSnapshot>,
        mother: Option<CovariateSnapshot>,
    ) -> Self {
        Self {
            date: person.date,
            income: person.income,
            education: person.education,
            socioeconomic_status: person.socioeconomic_status,
            family_size: person.family_size,
            municipality: person.municipality,
            family_type: person.family_type,
            immigrant_background: person.immigrant_background,
            father_income: father.as_ref().and_then(|f| f.income.clone()),
            father_education: father.as_ref().and_then(|f| f.education.clone()),
            father_socioeconomic_status: father
                .as_ref()
                .and_then(|f| f.socioeconomic_status.clone()),
            mother_income: mother.as_ref().and_then(|f| f.income.clone()),
            mother_education: mother.as_ref().and_then(|f| f.education.clone()),
            mother_socioeconomic_status: mother
                .as_ref()
                .and_then(|f| f.socioeconomic_status.clone()),
        }
    }
}
