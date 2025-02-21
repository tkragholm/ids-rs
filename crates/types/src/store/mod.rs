use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
};

use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use dashmap::DashMap;

mod arrow_backend;
mod time_varying_backend;

pub use arrow_backend::ArrowStore;
pub use time_varying_backend::TimeVaryingStore;

#[derive(Debug, Hash, Eq, PartialEq)]
struct CacheKey {
    pnr: String,
    covariate_type: CovariateType,
    date: NaiveDate,
}

pub trait Store: Send + Sync {
    fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError>;

    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError>;
}

pub struct UnifiedStore {
    backend: StoreBackend,
    cache: DashMap<CacheKey, Covariate>,
}

enum StoreBackend {
    Arrow(ArrowStore),
    TimeVarying(TimeVaryingStore),
}

impl UnifiedStore {
    pub fn new_arrow() -> Self {
        Self {
            backend: StoreBackend::Arrow(ArrowStore::new()),
            cache: DashMap::new(),
        }
    }

    pub fn new_time_varying() -> Self {
        Self {
            backend: StoreBackend::TimeVarying(TimeVaryingStore::new()),
            cache: DashMap::new(),
        }
    }

    fn get_from_cache(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Option<Covariate> {
        let key = CacheKey {
            pnr: pnr.to_string(),
            covariate_type,
            date,
        };
        self.cache.get(&key).map(|v| v.clone())
    }

    fn store_in_cache(&self, pnr: &str, covariate: Covariate, date: NaiveDate) {
        let key = CacheKey {
            pnr: pnr.to_string(),
            covariate_type: covariate.get_type(),
            date,
        };
        self.cache.insert(key, covariate);
    }

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        match &mut self.backend {
            StoreBackend::Arrow(store) => store.load_family_relations(batches),
            StoreBackend::TimeVarying(_) => Err(IdsError::InvalidOperation(
                "Cannot load family relations into time varying store".to_string(),
            )),
        }
    }

    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        if let StoreBackend::Arrow(store) = &mut self.backend {
            store.add_akm_data(year, batches);
        }
    }

    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        if let StoreBackend::Arrow(store) = &mut self.backend {
            store.add_ind_data(year, batches);
        }
    }

    pub fn add_bef_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        if let StoreBackend::Arrow(store) = &mut self.backend {
            store.add_bef_data(period, batches);
        }
    }

    pub fn add_uddf_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        if let StoreBackend::Arrow(store) = &mut self.backend {
            store.add_uddf_data(period, batches);
        }
    }

    pub fn into_arrow_backend(self) -> Result<ArrowStore, IdsError> {
        match self.backend {
            StoreBackend::Arrow(store) => Ok(store),
            StoreBackend::TimeVarying(_) => Err(IdsError::InvalidOperation(
                "Cannot convert time varying store to arrow backend".to_string(),
            )),
        }
    }
}

impl Store for UnifiedStore {
    fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        // Try cache first
        if let Some(cached) = self.get_from_cache(pnr, covariate_type, date) {
            return Ok(Some(cached));
        }

        // If not in cache, get from backend
        let result = match &self.backend {
            StoreBackend::Arrow(store) => store.get_covariate(pnr, covariate_type, date)?,
            StoreBackend::TimeVarying(store) => store.get_covariate(pnr, covariate_type, date)?,
        };

        // Store in cache if found
        if let Some(ref covariate) = result {
            self.store_in_cache(pnr, covariate.clone(), date);
        }

        Ok(result)
    }

    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        match &self.backend {
            StoreBackend::Arrow(store) => store.get_family_relations(pnr),
            StoreBackend::TimeVarying(store) => store.get_family_relations(pnr),
        }
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        match &mut self.backend {
            StoreBackend::Arrow(store) => store.load_data(data),
            StoreBackend::TimeVarying(store) => store.load_data(data),
        }
    }
}
