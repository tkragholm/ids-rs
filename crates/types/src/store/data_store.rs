use std::path::Path;
use std::sync::Arc;
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use dashmap::DashMap;

use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
    traits::Store,
    store::{ArrowBackend, TimeVaryingBackend},
};

/// Cache key for covariate lookups
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct CacheKey {
    pub pnr: String,
    pub covariate_type: CovariateType,
    pub date: NaiveDate,
}

/// Combined store implementation with different backend options and caching
pub struct DataStore {
    backend: Arc<dyn Store>,
    cache: DashMap<CacheKey, Covariate>,
}

impl DataStore {
    /// Create a new DataStore with an ArrowBackend
    pub fn new_arrow() -> Result<Self, IdsError> {
        Ok(Self {
            backend: Arc::new(ArrowBackend::new()?),
            cache: DashMap::new(),
        })
    }

    /// Create a new DataStore with a TimeVaryingBackend
    #[must_use] 
    pub fn new_time_varying() -> Self {
        Self {
            backend: Arc::new(TimeVaryingBackend::new()),
            cache: DashMap::new(),
        }
    }

    /// Get a covariate from the cache
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

    /// Store a covariate in the cache
    fn store_in_cache(&self, pnr: &str, covariate: Covariate, date: NaiveDate) {
        let key = CacheKey {
            pnr: pnr.to_string(),
            covariate_type: covariate.get_type(),
            date,
        };
        self.cache.insert(key, covariate);
    }

    /// Access the underlying arrow backend (if available)
    #[must_use] 
    pub fn as_arrow_backend(&self) -> Option<&ArrowBackend> {
        self.backend.as_any().downcast_ref::<ArrowBackend>()
    }

    /// Access the underlying arrow backend mutably (if available)
    pub fn as_arrow_backend_mut(&mut self) -> Option<&mut ArrowBackend> {
        Arc::get_mut(&mut self.backend)?
            .as_any_mut()
            .downcast_mut::<ArrowBackend>()
    }

    /// Load family relations data (only for arrow backend)
    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.load_family_relations(batches)
        } else {
            Err(IdsError::invalid_operation(
                "Cannot load family relations into this backend type",
            ))
        }
    }

    /// Add AKM (labor market) data
    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.add_akm_data(year, batches)?;
            Ok(())
        } else {
            Err(IdsError::invalid_operation(
                "Cannot add AKM data to this backend type",
            ))
        }
    }

    /// Add BEF (population) data
    pub fn add_bef_data(
        &mut self,
        period: String,
        batches: Vec<RecordBatch>,
    ) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.add_bef_data(period, batches)?;
            Ok(())
        } else {
            Err(IdsError::invalid_operation(
                "Cannot add BEF data to this backend type",
            ))
        }
    }

    /// Add IND (income) data
    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.add_ind_data(year, batches)?;
            Ok(())
        } else {
            Err(IdsError::invalid_operation(
                "Cannot add IND data to this backend type",
            ))
        }
    }

    /// Add UDDF (education) data
    pub fn add_uddf_data(
        &mut self,
        period: String,
        batches: Vec<RecordBatch>,
    ) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.add_uddf_data(period, batches)?;
            Ok(())
        } else {
            Err(IdsError::invalid_operation(
                "Cannot add UDDF data to this backend type",
            ))
        }
    }

    /// Save current covariates to CSV (only for time-varying backend)
    pub fn save_to_csv(&self, path: &Path) -> Result<(), IdsError> {
        if let Some(backend) = self.backend.as_any().downcast_ref::<TimeVaryingBackend>() {
            backend.save_to_csv(path)
        } else {
            Err(IdsError::invalid_operation(
                "Cannot save this backend type to CSV",
            ))
        }
    }
}

impl Store for DataStore {
    fn covariate(
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
        let result = self.backend.covariate(pnr, covariate_type, date)?;

        // Store in cache if found
        if let Some(ref covariate) = result {
            self.store_in_cache(pnr, covariate.clone(), date);
        }

        Ok(result)
    }

    fn family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.backend.family_relations(pnr)
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        if let Some(backend) = Arc::get_mut(&mut self.backend) {
            backend.load_data(data)
        } else {
            Err(IdsError::invalid_operation(
                "Cannot load data into a shared backend",
            ))
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}