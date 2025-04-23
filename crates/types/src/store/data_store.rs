use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use std::path::Path;

use crate::{
    error::{IdsError, Result},
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
    storage::{self, arrow::backend::ArrowBackend, ThreadSafeStore},
    store::TimeVaryingBackend,
    traits::Store,
};

// Use the shared CacheKey from storage module
pub use storage::CacheKey;

/// Combined store implementation with different backend options and thread-safety
pub enum DataStore {
    Arrow(ThreadSafeStore<ArrowBackend>),
    TimeVarying(ThreadSafeStore<TimeVaryingBackend>),
}

impl DataStore {
    /// Create a new DataStore with an ArrowBackend
    pub fn new_arrow() -> Result<Self> {
        Ok(Self::Arrow(ThreadSafeStore::new(ArrowBackend::new()?)))
    }

    /// Create a new DataStore with a TimeVaryingBackend
    #[must_use]
    pub fn new_time_varying() -> Self {
        Self::TimeVarying(ThreadSafeStore::new(TimeVaryingBackend::new()))
    }

    /// Access the underlying arrow store (thread-safe)
    #[must_use]
    pub fn as_arrow_store(&self) -> Option<&ThreadSafeStore<ArrowBackend>> {
        match self {
            Self::Arrow(store) => Some(store),
            _ => None,
        }
    }

    /// Access the underlying time-varying store (thread-safe)
    #[must_use]
    pub fn as_time_varying_store(&self) -> Option<&ThreadSafeStore<TimeVaryingBackend>> {
        match self {
            Self::TimeVarying(store) => Some(store),
            _ => None,
        }
    }

    /// Check if this data store contains a specific backend type
    #[must_use]
    pub fn has_backend_type<T: Store + 'static>(&self) -> bool {
        match self {
            Self::Arrow(_) => std::any::TypeId::of::<ArrowBackend>() == std::any::TypeId::of::<T>(),
            Self::TimeVarying(_) => {
                std::any::TypeId::of::<TimeVaryingBackend>() == std::any::TypeId::of::<T>()
            }
        }
    }

    /// Load family relations data (only for arrow backend)
    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<()> {
        match self {
            Self::Arrow(store) => {
                let mut backend = store.write();
                backend.load_family_relations(batches)
            }
            _ => Err(IdsError::invalid_operation(
                "Cannot load family relations into this backend type",
            )),
        }
    }

    /// Add AKM (labor market) data
    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) -> Result<()> {
        match self {
            Self::Arrow(store) => {
                let mut backend = store.write();
                backend.add_akm_data(year, batches)
            }
            _ => Err(IdsError::invalid_operation(
                "Cannot add AKM data to this backend type",
            )),
        }
    }

    /// Add BEF (population) data
    pub fn add_bef_data(&mut self, period: String, batches: Vec<RecordBatch>) -> Result<()> {
        match self {
            Self::Arrow(store) => {
                let mut backend = store.write();
                backend.add_bef_data(period, batches)
            }
            _ => Err(IdsError::invalid_operation(
                "Cannot add BEF data to this backend type",
            )),
        }
    }

    /// Add IND (income) data
    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) -> Result<()> {
        match self {
            Self::Arrow(store) => {
                let mut backend = store.write();
                backend.add_ind_data(year, batches)
            }
            _ => Err(IdsError::invalid_operation(
                "Cannot add IND data to this backend type",
            )),
        }
    }

    /// Add UDDF (education) data
    pub fn add_uddf_data(&mut self, period: String, batches: Vec<RecordBatch>) -> Result<()> {
        match self {
            Self::Arrow(store) => {
                let mut backend = store.write();
                backend.add_uddf_data(period, batches)
            }
            _ => Err(IdsError::invalid_operation(
                "Cannot add UDDF data to this backend type",
            )),
        }
    }

    /// Save current covariates to CSV (only for time-varying backend)
    pub fn save_to_csv(&self, path: &Path) -> Result<()> {
        match self {
            Self::TimeVarying(store) => {
                let backend = store.read();
                backend.save_to_csv(path)
            }
            _ => Err(IdsError::invalid_operation(
                "Cannot save this backend type to CSV",
            )),
        }
    }
}

impl Store for DataStore {
    fn covariate(
        &mut self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>> {
        // Delegate to the appropriate backend with proper locking
        match self {
            Self::Arrow(store) => {
                let mut backend = store.write();
                backend.covariate(pnr, covariate_type, date)
            }
            Self::TimeVarying(store) => {
                let mut backend = store.write();
                backend.covariate(pnr, covariate_type, date)
            }
        }
    }

    fn family_relations(&self, _pnr: &str) -> Option<&FamilyRelations> {
        // This implementation is inherently problematic with our RwLock approach.
        // We can't return a reference to data inside the lock, as the lock would be released.
        // For now, we'll return None, but this API needs to be restructured.
        None
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<()> {
        match self {
            Self::Arrow(store) => {
                let mut backend = store.write();
                backend.load_data(data)
            }
            Self::TimeVarying(store) => {
                let mut backend = store.write();
                backend.load_data(data)
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
