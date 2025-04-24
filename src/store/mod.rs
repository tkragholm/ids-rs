//! Data storage for the IDS-RS library
//!
//! This module contains the storage abstractions and implementations.

use crate::model::covariate::Covariate;
use crate::model::pnr::Pnr;
use crate::error::Result;
use chrono::NaiveDate;

pub mod memory;
pub mod arrow;

/// Storage interface for data access
pub trait Store {
    /// Get a specific covariate for a person at a given date
    fn get_covariate(&self, pnr: &Pnr, name: &str, date: NaiveDate) -> Result<Option<Covariate>>;
    
    /// Get all covariates for a person at a given date
    fn get_covariates(&self, pnr: &Pnr, date: NaiveDate) -> Result<Vec<Covariate>>;
    
    /// Add a covariate for a person at a given date
    fn add_covariate(&mut self, pnr: &Pnr, covariate: Covariate, date: NaiveDate) -> Result<()>;
}

/// Type of store to create
pub enum StoreType {
    /// In-memory store
    Memory,
    
    /// Arrow-based store with schema
    Arrow(arrow::ArrowSchema),
}

/// Create a new store of the specified type
#[must_use] pub fn create_store(store_type: StoreType) -> Box<dyn Store> {
    match store_type {
        StoreType::Memory => Box::new(memory::MemoryStore::new()),
        StoreType::Arrow(schema) => Box::new(arrow::ArrowStore::new(schema)),
    }
}