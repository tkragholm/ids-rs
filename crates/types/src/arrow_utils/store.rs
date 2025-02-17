use crate::{
    error::IdsError,
    family::{FamilyRelations, FamilyStore},
    models::{Education, Income, Occupation, TimeVaryingValue},
    snapshot::CovariateSnapshot,
    traits::*,
};
use arrow::record_batch::RecordBatch;
use chrono::Datelike;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ArrowStore {
    family_store: Option<Box<FamilyStore>>,
    akm_data: HashMap<i32, Vec<RecordBatch>>,
    bef_data: HashMap<String, Vec<RecordBatch>>,
    ind_data: HashMap<i32, Vec<RecordBatch>>,
    uddf_data: HashMap<String, Vec<RecordBatch>>,
}

impl ArrowStore {
    pub fn new() -> Self {
        Self {
            family_store: None,
            akm_data: HashMap::new(),
            bef_data: HashMap::new(),
            ind_data: HashMap::new(),
            uddf_data: HashMap::new(),
        }
    }

    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        self.akm_data.insert(year, batches);
    }

    pub fn add_bef_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        self.bef_data.insert(period, batches);
    }

    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        self.ind_data.insert(year, batches);
    }

    pub fn add_uddf_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        self.uddf_data.insert(period, batches);
    }

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        let mut store = FamilyStore::new(self.clone());
        store.load_family_relations(batches)?;
        self.family_store = Some(Box::new(store));
        Ok(())
    }
}

impl Store for ArrowStore {
    fn load_education(&self, _data: Vec<TimeVaryingValue<Education>>) -> Result<(), IdsError> {
        Ok(())
    }

    fn load_income(&self, _data: Vec<TimeVaryingValue<Income>>) -> Result<(), IdsError> {
        Ok(())
    }

    fn load_occupation(&self, _data: Vec<TimeVaryingValue<Occupation>>) -> Result<(), IdsError> {
        Ok(())
    }
}

impl FamilyAccess for ArrowStore {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.family_store
            .as_ref()
            .and_then(|store| store.get_family_relations(pnr))
    }

    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)> {
        self.get_family_relations(pnr)
            .map(|relations| (relations.father_id.clone(), relations.mother_id.clone()))
    }

    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate> {
        self.get_family_relations(pnr)
            .map(|relations| relations.birth_date)
    }
}

impl DataAccess for ArrowStore {
    fn get_covariates_at_date(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<CovariateSnapshot, IdsError> {
        use crate::traits::DateHelpers;

        let year = date.year();
        let quarter = date.get_quarter();
        let period = format!("{}{:02}", year, quarter * 3);

        let snapshot = CovariateSnapshot::new(date);

        // TODO: Implement data extraction
        let _ = (pnr, period); // Acknowledge usage until implemented

        Ok(snapshot)
    }

    fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<CovariateSnapshot>, IdsError> {
        let family = self
            .get_family_relations(pnr)
            .ok_or_else(|| IdsError::MissingData("Family info not found".to_string()))?;

        let person_covariates = self.get_covariates_at_date(pnr, date)?;

        let father_covariates = family
            .father_id
            .as_ref()
            .and_then(|id| self.get_covariates_at_date(id, date).ok());

        let mother_covariates = family
            .mother_id
            .as_ref()
            .and_then(|id| self.get_covariates_at_date(id, date).ok());

        Ok(Some(CovariateSnapshot::combine(
            person_covariates,
            father_covariates,
            mother_covariates,
        )))
    }
}
