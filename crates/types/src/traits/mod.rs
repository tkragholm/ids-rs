//! Traits for the ids-rs codebase
//!
//! This module contains trait definitions that provide
//! standardized interfaces for various components of the system.

// Submodules
mod cacheable;
pub mod access;
pub mod processing;
mod utils;

// Imports
use crate::{
    error::Result,
    models::{Covariate, CovariateType, TimeVaryingValue},
    OldFamilyRelations,
};
use chrono::NaiveDate;

// Re-exports
// ArrowAccess is now in storage::arrow
pub use crate::storage::arrow::access::ArrowAccess;
pub use self::cacheable::Cacheable;
pub use self::processing::{CovariateProcessor, VariableType};
pub use self::utils::DateHelpers;

// Store trait definition here directly
/// Combined Store trait - both data and family access
pub trait Store: Send + Sync {
    /// Get a covariate for a person at a specific date
    fn get_covariate(
        &self, 
        pnr: &str, 
        covariate_type: CovariateType, 
        date: NaiveDate
    ) -> Result<Option<Covariate>>;

    /// Get family relations for a person
    fn get_family_relations(&self, pnr: &str) -> Option<&OldFamilyRelations>;

    /// Load data into the store
    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<()>;

    /// Get all covariates for a person at a specific date
    fn get_covariates(
        &self, 
        pnr: &str, 
        date: NaiveDate
    ) -> Result<hashbrown::HashMap<CovariateType, Covariate>> {
        let mut covariates = hashbrown::HashMap::new();
        
        for covariate_type in &[
            CovariateType::Demographics,
            CovariateType::Education,
            CovariateType::Income,
            CovariateType::Occupation,
        ] {
            if let Some(covariate) = self.get_covariate(pnr, *covariate_type, date)? {
                covariates.insert(*covariate_type, covariate);
            }
        }
        
        Ok(covariates)
    }

    /// Get family covariates for a person at a specific date
    fn get_family_covariates(
        &self, 
        pnr: &str, 
        date: NaiveDate
    ) -> Result<Option<hashbrown::HashMap<CovariateType, Covariate>>> {
        let family = self.get_family_relations(pnr);
        
        if let Some(_family) = family {
            let covariates = self.get_covariates(pnr, date)?;
            if !covariates.is_empty() {
                return Ok(Some(covariates));
            }
        }
        
        Ok(None)
    }
    
    /// Convert to Any for dynamic casting
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Convert to Any for dynamic casting (mutable)
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Trait for accessing family relations
pub trait FamilyAccess {
    fn get_family_relations(&self, pnr: &str) -> Option<&OldFamilyRelations>;
    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)>;
    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate>;
}

/// Trait for accessing time-varying data
pub trait TimeVaryingAccess<T> {
    fn get_at_date(&self, pnr: &str, date: NaiveDate) -> Option<Vec<T>>;
    fn load_data(&self, data: Vec<TimeVaryingValue<T>>) -> Result<()>;
}

// Implement FamilyAccess for any type that implements Store
impl<T: Store> FamilyAccess for T {
    fn get_family_relations(&self, pnr: &str) -> Option<&OldFamilyRelations> {
        Store::get_family_relations(self, pnr)
    }

    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)> {
        self.get_family_relations(pnr)
            .map(|rel| (rel.father_id.clone(), rel.mother_id.clone()))
    }

    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate> {
        self.get_family_relations(pnr)
            .map(|rel| rel.birth_date)
    }
}