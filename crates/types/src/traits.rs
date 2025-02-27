use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, TimeVaryingValue},
};
use chrono::{Datelike, NaiveDate};

// Import Storage from where it's actually defined
pub use crate::storage::Storage;

// Alias for backward compatibility
pub type DataAccess = dyn Storage;

/// Trait for accessing family relations
pub trait FamilyAccess {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;
    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)>;
    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate>;
}

/// Combined Store trait - both data and family access
pub trait Store: Storage + FamilyAccess {
    fn load_covariate(&self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError>;
}

/// Trait for accessing time-varying data
pub trait TimeVaryingAccess<T> {
    fn get_at_date(&self, pnr: &str, date: NaiveDate) -> Option<Vec<T>>;
    fn load_data(&self, data: Vec<TimeVaryingValue<T>>) -> Result<(), IdsError>;
}

/// Helper for date operations
pub trait DateHelpers: Datelike {
    fn get_quarter(&self) -> u32 {
        ((self.month() - 1) / 3) + 1
    }
}

impl DateHelpers for NaiveDate {}

// Implement FamilyAccess for any type that implements Storage
impl<T: Storage> FamilyAccess for T {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        Storage::get_family_relations(self, pnr)
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

// Default implementation of Store for any type that implements Storage + FamilyAccess
impl<T: Storage + FamilyAccess> Store for T {
    fn load_covariate(&self, _data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        // This is a default implementation that just returns an error
        // Concrete implementations should override this
        Err(IdsError::invalid_operation(
            "This store does not support loading covariates directly",
        ))
    }
}
