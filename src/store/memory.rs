use std::collections::HashMap;
use chrono::NaiveDate;

use crate::error::Result;
use crate::model::covariate::Covariate;
use crate::model::pnr::Pnr;
use super::Store;

/// In-memory data store implementation
pub struct MemoryStore {
    /// Data structure: PNR -> Covariate name -> Date -> Covariate
    data: HashMap<String, HashMap<String, HashMap<NaiveDate, Covariate>>>,
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore {
    /// Create a new empty memory store
    #[must_use] pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    /// Get the number of people in the store
    #[must_use] pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if the store is empty
    #[must_use] pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Clear all data from the store
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Store for MemoryStore {
    fn get_covariate(&self, pnr: &Pnr, name: &str, date: NaiveDate) -> Result<Option<Covariate>> {
        Ok(self.data
            .get(pnr.value())
            .and_then(|covariates| covariates.get(name))
            .and_then(|dates| dates.get(&date))
            .cloned())
    }

    fn get_covariates(&self, pnr: &Pnr, date: NaiveDate) -> Result<Vec<Covariate>> {
        Ok(self.data
            .get(pnr.value())
            .map(|covariates| {
                covariates
                    .values()
                    .filter_map(|dates| dates.get(&date).cloned())
                    .collect()
            })
            .unwrap_or_default())
    }

    fn add_covariate(&mut self, pnr: &Pnr, covariate: Covariate, date: NaiveDate) -> Result<()> {
        let name = covariate.name().to_string();
        self.data
            .entry(pnr.value().to_string())
            .or_default()
            .entry(name)
            .or_default()
            .insert(date, covariate);
        Ok(())
    }
}