use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
};

use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use hashbrown::HashMap;
use std::sync::Arc;

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
    backend: Arc<Mutex<StoreBackend>>,
    // Use parking_lot RwLock with HashMap for the primary cache
    cache: Arc<RwLock<HashMap<CacheKey, Covariate>>>,
    // Keep DashMap as secondary concurrent cache for better read performance
    concurrent_cache: DashMap<CacheKey, Covariate>,
}

enum StoreBackend {
    Arrow(ArrowStore),
    TimeVarying(TimeVaryingStore),
}

impl UnifiedStore {
    pub fn new_arrow() -> Result<Self, IdsError> {
        Ok(Self {
            backend: Arc::new(Mutex::new(StoreBackend::Arrow(ArrowStore::new()?))),
            cache: Arc::new(RwLock::new(HashMap::new())),
            concurrent_cache: DashMap::new(),
        })
    }

    pub fn new_time_varying() -> Self {
        Self {
            backend: Arc::new(Mutex::new(StoreBackend::TimeVarying(TimeVaryingStore::new()))),
            cache: Arc::new(RwLock::new(HashMap::new())),
            concurrent_cache: DashMap::new(),
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
        
        // First try the primary cache with a read lock
        {
            let cache = self.cache.read();
            if let Some(value) = cache.get(&key) {
                return Some(value.clone());
            }
        }
        
        // Then try the concurrent cache
        self.concurrent_cache.get(&key).map(|v| v.clone())
    }

    fn store_in_cache(&self, pnr: &str, covariate: Covariate, date: NaiveDate) {
        let key = CacheKey {
            pnr: pnr.to_string(),
            covariate_type: covariate.get_type(),
            date,
        };
        
        // Update both caches
        {
            let mut cache = self.cache.write();
            cache.insert(key.clone(), covariate.clone());
        }
        self.concurrent_cache.insert(key, covariate);
    }

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        let mut backend_guard = self.backend.lock();
        match &mut *backend_guard {
            StoreBackend::Arrow(store) => store.load_family_relations(batches),
            StoreBackend::TimeVarying(_) => Err(IdsError::InvalidOperation(
                "Cannot load family relations into time varying store".to_string(),
            )),
        }
    }

    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        let mut backend_guard = self.backend.lock();
        if let StoreBackend::Arrow(store) = &mut *backend_guard {
            store.add_akm_data(year, batches);
        }
    }

    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        let mut backend_guard = self.backend.lock();
        if let StoreBackend::Arrow(store) = &mut *backend_guard {
            store.add_ind_data(year, batches);
        }
    }

    pub fn add_bef_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        let mut backend_guard = self.backend.lock();
        if let StoreBackend::Arrow(store) = &mut *backend_guard {
            store.add_bef_data(period, batches);
        }
    }

    pub fn add_uddf_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        let mut backend_guard = self.backend.lock();
        if let StoreBackend::Arrow(store) = &mut *backend_guard {
            store.add_uddf_data(period, batches);
        }
    }

    pub fn into_arrow_backend(self) -> Result<ArrowStore, IdsError> {
        // We need to unwrap the Arc<Mutex<>> before matching
        let backend = match Arc::try_unwrap(self.backend) {
            Ok(mutex) => mutex.into_inner(),
            Err(_) => return Err(IdsError::InvalidOperation(
                "Cannot unwrap backend due to active references".to_string(),
            )),
        };
        
        match backend {
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
        let backend_guard = self.backend.lock();
        let result = match &*backend_guard {
            StoreBackend::Arrow(store) => store.get_covariate(pnr, covariate_type, date)?,
            StoreBackend::TimeVarying(store) => store.get_covariate(pnr, covariate_type, date)?,
        };
        
        // Drop the lock before storing in cache
        drop(backend_guard);

        // Store in cache if found
        if let Some(ref covariate) = result {
            self.store_in_cache(pnr, covariate.clone(), date);
        }

        Ok(result)
    }

    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        let backend_guard = self.backend.lock();
        match &*backend_guard {
            StoreBackend::Arrow(store) => store.get_family_relations(pnr),
            StoreBackend::TimeVarying(store) => store.get_family_relations(pnr),
        }
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        let mut backend_guard = self.backend.lock();
        match &mut *backend_guard {
            StoreBackend::Arrow(store) => store.load_data(data),
            StoreBackend::TimeVarying(store) => store.load_data(data),
        }
    }
}
