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
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Income
    }

    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != CovariateType::Income {
            return None;
        }

        covariate.get_income_amount()
    }

    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != CovariateType::Income {
            return None;
        }

        // Example: convert employment status to string
        covariate.get_employment_status().map(|status| status.to_string())
    }

    fn is_categorical(&self) -> bool {
        false
    }
}

impl ConfigurableProcessor for IncomeProcessor {
    fn from_config(config: &CovariateTypeConfig) -> Result<Self, Error> {
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