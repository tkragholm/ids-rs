use crate::error::Result;
use crate::models::covariate::{Covariate, CovariateType};
use crate::traits::access::Store;

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

/// Trait for processing covariate data from a data store
///
/// This trait defines the core functionality for processors that transform
/// raw data from a store into structured covariate values. Implementations
/// are responsible for handling specific covariate types and applying the
/// appropriate transformations.
pub trait CovariateProcessor: Send + Sync {
    /// Process covariate data for a specific year
    ///
    /// # Arguments
    /// * `store` - The data store containing source data
    /// * `year` - The year for which to process data
    ///
    /// # Returns
    /// * `Result<Covariate>` - The processed covariate data or an error
    ///
    /// # Errors
    /// Returns an error if:
    /// - Required data is missing from the store
    /// - Processing fails due to invalid or inconsistent data
    /// - Type conversion errors occur
    fn process(&self, store: &dyn Store, year: i32) -> Result<Covariate>;
    
    /// Get the type of covariate this processor handles
    ///
    /// # Returns
    /// * `CovariateType` - The type of covariate this processor generates
    fn covariate_type(&self) -> CovariateType;
    
    /// Get the field names required by this processor
    ///
    /// # Returns
    /// * `Vec<String>` - List of field names needed from the store
    fn required_fields(&self) -> Vec<String>;
    
    /// Check if the processor can run with the available data
    ///
    /// # Arguments
    /// * `store` - The data store to check
    /// * `year` - The year to check
    ///
    /// # Returns
    /// * `bool` - True if all required fields are available, false otherwise
    fn can_process(&self, store: &dyn Store, year: i32) -> bool {
        self.required_fields().iter().all(|field| store.has_data(year, field))
    }
    
    /// Get the name of this processor
    fn name(&self) -> &str;
    
    /// Determine if this variable should be treated as categorical
    fn is_categorical(&self) -> bool;
    
    /// Get the variable type for this processor
    fn variable_type(&self) -> VariableType {
        if self.is_categorical() {
            VariableType::Categorical
        } else {
            VariableType::Numeric
        }
    }
    
    /// Extract a numeric value from a covariate, returning None if not applicable
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64>;
    
    /// Extract a categorical value from a covariate, returning None if not applicable
    fn process_categorical(&self, covariate: &Covariate) -> Option<String>;
    
    /// Convert a categorical value to a numeric representation if needed for calculations
    fn categorical_to_numeric(&self, value: &str) -> f64 {
        if let Ok(num) = value.parse::<f64>() { 
            num 
        } else {
            // Hash the string to create a stable numeric value
            let mut hash = 0.0;
            for (i, b) in value.bytes().enumerate() {
                hash += f64::from(b) * (i + 1) as f64;
            }
            hash
        }
    }
}

/// Extension trait for covariate processors with additional utility methods
pub trait CovariateProcessorExt: CovariateProcessor {
    /// Process covariate data for multiple years
    ///
    /// # Arguments
    /// * `store` - The data store containing source data
    /// * `years` - The years for which to process data
    ///
    /// # Returns
    /// * `Result<Vec<Covariate>>` - The processed covariate data for each year or an error
    ///
    /// # Errors
    /// Returns an error if processing fails for any year
    fn process_years(&self, store: &dyn Store, years: &[i32]) -> Result<Vec<Covariate>>;
    
    /// Process all available years in the store
    ///
    /// # Arguments
    /// * `store` - The data store containing source data
    ///
    /// # Returns
    /// * `Result<Vec<Covariate>>` - The processed covariate data for all years or an error
    ///
    /// # Errors
    /// Returns an error if processing fails for any year
    fn process_all_years(&self, store: &dyn Store) -> Result<Vec<Covariate>>;
    
    /// Find the latest year that can be processed
    ///
    /// # Arguments
    /// * `store` - The data store to check
    ///
    /// # Returns
    /// * `Option<i32>` - The latest year that can be processed, or None if no year can be processed
    fn latest_processable_year(&self, store: &dyn Store) -> Option<i32>;
}

// Implement the extension trait for any type that implements CovariateProcessor
impl<T: CovariateProcessor> CovariateProcessorExt for T {
    fn process_years(&self, store: &dyn Store, years: &[i32]) -> Result<Vec<Covariate>> {
        let mut results = Vec::with_capacity(years.len());
        
        for &year in years {
            if self.can_process(store, year) {
                results.push(self.process(store, year)?);
            }
        }
        
        Ok(results)
    }
    
    fn process_all_years(&self, store: &dyn Store) -> Result<Vec<Covariate>> {
        let years = store.years();
        self.process_years(store, &years)
    }
    
    fn latest_processable_year(&self, store: &dyn Store) -> Option<i32> {
        store.years().into_iter()
            .filter(|&year| self.can_process(store, year))
            .max()
    }
}

// For backward compatibility
pub trait LegacyCovariateProcessor: CovariateProcessor {
    fn get_name(&self) -> &str {
        self.name()
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        self.covariate_type()
    }
    
    fn get_variable_type(&self) -> VariableType {
        self.variable_type()
    }
}

// Implement LegacyCovariateProcessor for all CovariateProcessor implementors
impl<T: CovariateProcessor + ?Sized> LegacyCovariateProcessor for T {}