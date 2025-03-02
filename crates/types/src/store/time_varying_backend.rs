use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
};
use chrono::NaiveDate;
use dashmap::DashMap;
use hashbrown::HashMap;

#[derive(Debug)]
pub struct TimeVaryingStore {
    data: DashMap<String, Vec<TimeVaryingValue<Covariate>>>,
    family_data: HashMap<String, FamilyRelations>,
}

impl TimeVaryingStore {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
            family_data: HashMap::new(),
        }
    }

    fn get_latest_value(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Option<Covariate> {
        self.data.get(pnr).and_then(|values| {
            values
                .iter()
                .filter(|v| v.date <= date && v.value.get_type() == covariate_type)
                .max_by_key(|v| v.date)
                .map(|v| v.value.clone())
        })
    }
}

impl super::Store for TimeVaryingStore {
    fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        Ok(self.get_latest_value(pnr, covariate_type, date))
    }

    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.family_data.get(pnr)
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        for value in data {
            self.data.entry(value.pnr.clone()).or_default().push(value);
        }
        Ok(())
    }
}
