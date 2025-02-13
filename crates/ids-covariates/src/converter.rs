use crate::models::PersonCovariates;
use ids_arrow::CovariateSnapshot;

pub trait IntoSnapshot {
    fn into_snapshot(self, date: chrono::NaiveDate) -> CovariateSnapshot;
}

impl IntoSnapshot for PersonCovariates {
    fn into_snapshot(self, date: chrono::NaiveDate) -> CovariateSnapshot {
        let income = self.income.last().map(|i| i.value.amount);
        let education = self.education.last().map(|e| e.value.level.clone());
        let occupation_code = self.occupation.last().map(|o| o.value.code.clone());

        CovariateSnapshot {
            date,
            income,
            education,
            socioeconomic_status: occupation_code.and_then(|code| code.parse().ok()),
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
}
