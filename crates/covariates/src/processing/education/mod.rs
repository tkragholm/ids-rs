use types::error::{IdsError, Result};
use types::models::{Covariate, CovariateType};
use types::traits::CovariateProcessor;
use crate::core::config::CovariateTypeConfig;
use crate::core::Error;
use crate::processing::processor::ConfigurableProcessor;

/// Processor for education covariates
pub struct EducationProcessor {
    name: String,
}

impl EducationProcessor {
    /// Create a new education processor
    pub fn new() -> Self {
        Self {
            name: "Education".to_string(),
        }
    }
}

impl CovariateProcessor for EducationProcessor {
    fn process(&self, _store: &dyn types::traits::access::Store, _year: i32) -> Result<Covariate> {
        // Default implementation - would be overridden by concrete implementation
        Err(IdsError::invalid_operation("Not implemented".to_string()))
    }
    
    fn covariate_type(&self) -> CovariateType {
        CovariateType::Education
    }
    
    fn required_fields(&self) -> Vec<String> {
        vec![
            "HFAUDD".to_string(),
        ]
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_categorical(&self) -> bool {
        // Default to false
        false
    }

    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.type_() != CovariateType::Education {
            return None;
        }

        // Example implementation for education years
        covariate.education_years().map(|years| years as f64)
    }

    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.type_() != CovariateType::Education {
            return None;
        }

        // Example implementation for education level
        covariate.education_level().clone()
    }
}

impl ConfigurableProcessor for EducationProcessor {
    fn from_config(config: &CovariateTypeConfig) -> std::result::Result<Self, Error> {
        if config.covariate_type != CovariateType::Education {
            return Err(Error::config(
                format!("Invalid covariate type: expected Education, got {:?}", 
                config.covariate_type)
            ));
        }
        
        Ok(Self {
            name: config.name.clone(),
        })
    }
}

/// Default implementation
impl Default for EducationProcessor {
    fn default() -> Self {
        Self::new()
    }
}