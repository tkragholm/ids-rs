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
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Occupation
    }

    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != CovariateType::Occupation {
            return None;
        }

        // Example: use socio as numeric value
        covariate.get_socio().map(|socio| socio as f64)
    }

    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != CovariateType::Occupation {
            return None;
        }

        // Example: use occupation code as categorical
        covariate.get_occupation_code().clone()
    }

    fn is_categorical(&self) -> bool {
        true
    }
}

impl ConfigurableProcessor for OccupationProcessor {
    fn from_config(config: &CovariateTypeConfig) -> Result<Self, Error> {
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