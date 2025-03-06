use crate::models::{Covariate, CovariateType};

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