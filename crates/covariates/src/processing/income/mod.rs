use types::error::{IdsError, Result};
use types::models::{Covariate, CovariateType};
use types::traits::CovariateProcessor;
use crate::core::config::CovariateTypeConfig;
use crate::core::Error;
use crate::processing::processor::ConfigurableProcessor;

/// Processor for income covariates
pub struct IncomeProcessor {
    name: String,
}

impl IncomeProcessor {
    /// Create a new income processor
    pub fn new() -> Self {
        Self {
            name: "Income".to_string(),
        }
    }
}

impl CovariateProcessor for IncomeProcessor {
    fn process(&self, _store: &dyn types::traits::access::Store, _year: i32) -> Result<Covariate> {
        // Default implementation - would be overridden by concrete implementation
        Err(IdsError::invalid_operation("Not implemented".to_string()))
    }
    
    fn covariate_type(&self) -> CovariateType {
        CovariateType::Income
    }
    
    fn required_fields(&self) -> Vec<String> {
        vec![
            "PERINDKIALT_13".to_string(),
        ]
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_categorical(&self) -> bool {
        false
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.type_() != CovariateType::Income {
            return None;
        }

        covariate.income_amount()
    }

    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.type_() != CovariateType::Income {
            return None;
        }

        // Example: convert employment status to string
        covariate.employment_status().map(|status| status.to_string())
    }
}

impl ConfigurableProcessor for IncomeProcessor {
    fn from_config(config: &CovariateTypeConfig) -> std::result::Result<Self, Error> {
        if config.covariate_type != CovariateType::Income {
            return Err(Error::config(
                format!("Invalid covariate type: expected Income, got {:?}", 
                config.covariate_type)
            ));
        }
        
        Ok(Self {
            name: config.name.clone(),
        })
    }
}

/// Default implementation
impl Default for IncomeProcessor {
    fn default() -> Self {
        Self::new()
    }
}