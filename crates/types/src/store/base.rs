use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Education, Income, Occupation, TimeVaryingValue},
    snapshot::CovariateSnapshot,
    storage::TimeVaryingStore,
    traits::{DataAccess, FamilyAccess, TimeVaryingAccess},
};
use chrono::NaiveDate;

pub struct BaseStore {
    education: TimeVaryingStore<Education>,
    income: TimeVaryingStore<Income>,
    occupation: TimeVaryingStore<Occupation>,
}

impl Default for BaseStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseStore {
    #[must_use] pub fn new() -> Self {
        Self {
            education: TimeVaryingStore::new(),
            income: TimeVaryingStore::new(),
            occupation: TimeVaryingStore::new(),
        }
    }

    fn get_latest_value<T: Clone>(
        &self,
        store: &TimeVaryingStore<T>,
        pnr: &str,
        date: NaiveDate,
    ) -> Option<T> {
        store.get_at_date(pnr, date)?.last().cloned()
    }
}

impl super::Store for BaseStore {
    fn load_education(&self, data: Vec<TimeVaryingValue<Education>>) -> Result<(), IdsError> {
        self.education.load_data(data)
    }

    fn load_income(&self, data: Vec<TimeVaryingValue<Income>>) -> Result<(), IdsError> {
        self.income.load_data(data)
    }

    fn load_occupation(&self, data: Vec<TimeVaryingValue<Occupation>>) -> Result<(), IdsError> {
        self.occupation.load_data(data)
    }
}

impl DataAccess for BaseStore {
    fn get_covariates_at_date(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<CovariateSnapshot, IdsError> {
        let education = self.get_latest_value(&self.education, pnr, date);
        let income = self.get_latest_value(&self.income, pnr, date);
        let occupation = self.get_latest_value(&self.occupation, pnr, date);

        Ok(CovariateSnapshot::new(date)
            .with_education(education)
            .with_income(income)
            .with_socioeconomic_status(occupation))
    }

    fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<CovariateSnapshot>, IdsError> {
        let family_relations = match self.get_family_relations(pnr) {
            Some(relations) => relations,
            None => return Ok(None),
        };

        let person_covariates = self.get_covariates_at_date(pnr, date)?;

        let father_covariates = match &family_relations.father_id {
            Some(father_id) => self.get_covariates_at_date(father_id, date).ok(),
            None => None,
        };

        let mother_covariates = match &family_relations.mother_id {
            Some(mother_id) => self.get_covariates_at_date(mother_id, date).ok(),
            None => None,
        };

        Ok(Some(CovariateSnapshot::combine(
            person_covariates,
            father_covariates,
            mother_covariates,
        )))
    }
}

impl FamilyAccess for BaseStore {
    fn get_family_relations(&self, _pnr: &str) -> Option<&FamilyRelations> {
        None
    }

    fn get_parents(&self, _pnr: &str) -> Option<(Option<String>, Option<String>)> {
        None
    }

    fn get_birth_date(&self, _pnr: &str) -> Option<NaiveDate> {
        None
    }
}
