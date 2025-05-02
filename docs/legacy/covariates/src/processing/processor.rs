use crate::core::config::{CovariateTypeConfig, CovariateVariableConfig};
use crate::core::Error;
use types::models::{Covariate, CovariateValue};
use types::traits::{CovariateProcessor, VariableType};

/// A processor that can be configured from configuration
pub trait ConfigurableProcessor: CovariateProcessor {
    /// Create a new processor from configuration
    fn from_config(config: &CovariateTypeConfig) -> std::result::Result<Self, Error>
    where
        Self: Sized;
}

/// A variable processor that can be configured
pub trait ConfigurableVariableProcessor {
    /// The variable type this processor handles
    fn variable_type() -> VariableType;

    /// Create a new processor from configuration
    fn from_config(config: &CovariateVariableConfig) -> std::result::Result<Self, Error>
    where
        Self: Sized;

    /// Process a covariate and return a value
    fn process(&self, covariate: &Covariate) -> Option<CovariateValue>;

    /// Get the name of the variable
    fn get_name(&self) -> &str;

    /// Get the description of the variable
    fn get_description(&self) -> Option<&str>;
}
