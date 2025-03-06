use std::path::Path;
use chrono::NaiveDate;
use dashmap::DashMap;
use hashbrown::HashMap;

use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
    traits::Store,
};

/// Time-varying storage backend
#[derive(Debug)]
pub struct TimeVaryingBackend {
    data: DashMap<String, Vec<TimeVaryingValue<Covariate>>>,
    family_data: HashMap<String, FamilyRelations>,
}

impl Default for TimeVaryingBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeVaryingBackend {
    /// Create a new time-varying backend
    #[must_use] 
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
            family_data: HashMap::new(),
        }
    }

    /// Get the latest value at a specific date
    fn get_latest_value(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Option<Covariate> {
        self.data.get(pnr).and_then(|values| {
            values
                .iter()
                .filter(|v| v.date <= date && v.value.type_() == covariate_type)
                .max_by_key(|v| v.date)
                .map(|v| v.value.clone())
        })
    }

    /// Save data to CSV
    pub fn save_to_csv(&self, path: &Path) -> Result<(), IdsError> {
        let mut writer = csv::Writer::from_path(path).map_err(IdsError::Csv)?;

        writer
            .write_record(["PNR", "Date", "Covariate Type", "Value"])
            .map_err(IdsError::Csv)?;

        for entry in &self.data {
            for value in entry.value() {
                writer
                    .write_record([
                        &value.pnr,
                        &value.date.to_string(),
                        &format!("{:?}", value.value.type_()),
                        &format!("{:?}", value.value),
                    ])
                    .map_err(IdsError::Csv)?;
            }
        }

        writer.flush().map_err(IdsError::Io)?;
        Ok(())
    }
    
    /// Add family relation
    pub fn add_family_relation(&mut self, relation: FamilyRelations) {
        self.family_data.insert(relation.pnr.clone(), relation);
    }
}

impl Store for TimeVaryingBackend {
    fn covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        Ok(self.get_latest_value(pnr, covariate_type, date))
    }

    fn family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.family_data.get(pnr)
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        for value in data {
            self.data.entry(value.pnr.clone()).or_default().push(value);
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}