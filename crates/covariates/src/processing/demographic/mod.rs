use types::error::{IdsError, Result};
use types::models::{Covariate, CovariateType};
use types::traits::CovariateProcessor;
use crate::core::config::CovariateTypeConfig;
use crate::core::Error;
use crate::processing::processor::ConfigurableProcessor;

/// Processor for demographic covariates
pub struct DemographicsProcessor {
    name: String,
}

impl DemographicsProcessor {
    /// Create a new demographics processor
    pub fn new() -> Self {
        Self {
            name: "Demographics".to_string(),
        }
    }
}

impl CovariateProcessor for DemographicsProcessor {
    fn process(&self, _store: &dyn types::traits::access::Store, _year: i32) -> Result<Covariate> {
        // Default implementation - would be overridden by concrete implementation
        Err(IdsError::invalid_operation("Not implemented".to_string()))
    }
    
    fn covariate_type(&self) -> CovariateType {
        CovariateType::Demographics
    }
    
    fn required_fields(&self) -> Vec<String> {
        vec![
            "KOM".to_string(),
            "FAMILIE_TYPE".to_string(),
            "STATSB".to_string(),
            "ANTPERSF".to_string(),
        ]
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_categorical(&self) -> bool {
        // Demographics can be both, default to false
        false
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != CovariateType::Demographics {
            return None;
        }

        // Example implementation for age
        covariate.get_age().map(|age| age as f64)
    }

    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != CovariateType::Demographics {
            return None;
        }

        // Example implementation for gender
        covariate.get_gender().clone()
    }
}

impl ConfigurableProcessor for DemographicsProcessor {
    fn from_config(config: &CovariateTypeConfig) -> std::result::Result<Self, Error> {
        if config.covariate_type != CovariateType::Demographics {
            return Err(Error::config(
                format!("Invalid covariate type: expected Demographics, got {:?}", 
                config.covariate_type)
            ));
        }
        
        Ok(Self {
            name: config.name.clone(),
        })
    }
}

/// Default implementation
impl Default for DemographicsProcessor {
    fn default() -> Self {
        Self::new()
    }
}