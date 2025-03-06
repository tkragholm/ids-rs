use types::error::{IdsError, Result};
use types::models::{Covariate, CovariateType};
use types::traits::CovariateProcessor;
use crate::core::config::CovariateTypeConfig;
use crate::core::Error;
use crate::processing::processor::ConfigurableProcessor;

/// Processor for occupation covariates
pub struct OccupationProcessor {
    name: String,
}

impl OccupationProcessor {
    /// Create a new occupation processor
    pub fn new() -> Self {
        Self {
            name: "Occupation".to_string(),
        }
    }
}

impl CovariateProcessor for OccupationProcessor {
    fn process(&self, _store: &dyn types::traits::access::Store, _year: i32) -> Result<Covariate> {
        // Default implementation - would be overridden by concrete implementation
        Err(IdsError::invalid_operation("Not implemented".to_string()))
    }
    
    fn covariate_type(&self) -> CovariateType {
        CovariateType::Occupation
    }
    
    fn required_fields(&self) -> Vec<String> {
        vec![
            "DISCO08".to_string(),
            "SOCIO".to_string(),
        ]
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_categorical(&self) -> bool {
        true
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.type_() != CovariateType::Occupation {
            return None;
        }

        // Example: use socio as numeric value
        covariate.socio().map(|socio| socio as f64)
    }

    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.type_() != CovariateType::Occupation {
            return None;
        }

        // Example: use occupation code as categorical
        covariate.occupation_code().clone()
    }
}

impl ConfigurableProcessor for OccupationProcessor {
    fn from_config(config: &CovariateTypeConfig) -> std::result::Result<Self, Error> {
        if config.covariate_type != CovariateType::Occupation {
            return Err(Error::config(
                format!("Invalid covariate type: expected Occupation, got {:?}", 
                config.covariate_type)
            ));
        }
        
        Ok(Self {
            name: config.name.clone(),
        })
    }
}

/// Default implementation
impl Default for OccupationProcessor {
    fn default() -> Self {
        Self::new()
    }
}