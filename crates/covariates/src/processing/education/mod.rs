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
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Education
    }

    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != CovariateType::Education {
            return None;
        }

        // Example implementation for education years
        covariate.get_education_years().map(|years| years as f64)
    }

    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != CovariateType::Education {
            return None;
        }

        // Example implementation for education level
        covariate.get_education_level().clone()
    }

    fn is_categorical(&self) -> bool {
        // Default to false
        false
    }
}

impl ConfigurableProcessor for EducationProcessor {
    fn from_config(config: &CovariateTypeConfig) -> Result<Self, Error> {
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