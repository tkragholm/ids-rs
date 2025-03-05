use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
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

/// Variable type for covariate processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VariableType {
    /// Numeric variable (f64)
    Numeric,
    /// Categorical variable (String)
    Categorical,
    /// Binary variable (0 or 1)
    Binary,
}

/// Trait that standardizes the processing of different covariate types
pub trait CovariateProcessor: Send + Sync {
    /// Get the name of this processor
    fn get_name(&self) -> &str;
    
    /// Get the covariate type this processor handles
    fn get_covariate_type(&self) -> CovariateType;
    
    /// Extract a numeric value from a covariate, returning None if not applicable
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64>;
    
    /// Extract a categorical value from a covariate, returning None if not applicable
    fn process_categorical(&self, covariate: &Covariate) -> Option<String>;
    
    /// Determine if this variable should be treated as categorical
    fn is_categorical(&self) -> bool;
    
    /// Get the variable type for this processor
    fn get_variable_type(&self) -> VariableType {
        if self.is_categorical() {
            VariableType::Categorical
        } else {
            VariableType::Numeric
        }
    }
    
    /// Convert a categorical value to a numeric representation if needed for calculations
    fn categorical_to_numeric(&self, value: &str) -> f64 {
        if let Ok(num) = value.parse::<f64>() { num } else {
            // Hash the string to create a stable numeric value
            let mut hash = 0.0;
            for (i, b) in value.bytes().enumerate() {
                hash += f64::from(b) * (i + 1) as f64;
            }
            hash
        }
    }
}
