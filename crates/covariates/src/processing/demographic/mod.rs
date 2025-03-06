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
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Demographics
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

    fn is_categorical(&self) -> bool {
        // Demographics can be both, default to false
        false
    }
}

impl ConfigurableProcessor for DemographicsProcessor {
    fn from_config(config: &CovariateTypeConfig) -> Result<Self, Error> {
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