use std::collections::HashMap;
use types::models::{Covariate, CovariateType};
use types::traits::{CovariateProcessor, VariableType};
use types::translation::{TranslationMaps, TranslationType};

use crate::config::{CovariateTypeConfig, CovariateVariableConfig, CovariatesConfig};

/// A configurable covariate processor built from configuration
pub struct ConfigurableProcessor {
    name: String,
    covariate_type: CovariateType,
    #[allow(dead_code)]
    variables: HashMap<String, VariableType>,
}

impl ConfigurableProcessor {
    /// Create a new processor from configuration
    pub fn new(config: &CovariateTypeConfig) -> Self {
        let mut variables = HashMap::new();

        // Register all variables with their types
        for var in &config.variables {
            variables.insert(var.name.clone(), var.variable_type);
        }

        Self {
            name: config.name.clone(),
            covariate_type: config.covariate_type,
            variables,
        }
    }
}

impl CovariateProcessor for ConfigurableProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_covariate_type(&self) -> CovariateType {
        self.covariate_type
    }

    fn process_numeric(&self, _covariate: &Covariate) -> Option<f64> {
        // Default to None - specialized implementations will be needed
        None
    }

    fn process_categorical(&self, _covariate: &Covariate) -> Option<String> {
        // Default to None - specialized implementations will be needed
        None
    }

    fn is_categorical(&self) -> bool {
        // Default to false - specialized implementations will determine this
        false
    }
}

/// A configurable variable processor built from configuration
pub struct ConfigurableVariableProcessor {
    name: String,
    variable_type: VariableType,
    covariate_type: CovariateType,
    accessor: String,
    translation: Option<String>,
    translation_maps: Option<TranslationMaps>,
}

impl ConfigurableVariableProcessor {
    /// Create a new variable processor from configuration
    pub fn new(config: &CovariateVariableConfig, covariate_type: CovariateType) -> Self {
        Self {
            name: config.name.clone(),
            variable_type: config.variable_type,
            covariate_type,
            accessor: config.accessor.clone(),
            translation: config.translation.clone(),
            translation_maps: None,
        }
    }

    /// Create a new variable processor with translation maps
    pub fn with_translation_maps(
        config: &CovariateVariableConfig,
        covariate_type: CovariateType,
        translation_maps: TranslationMaps,
    ) -> Self {
        Self {
            name: config.name.clone(),
            variable_type: config.variable_type,
            covariate_type,
            accessor: config.accessor.clone(),
            translation: config.translation.clone(),
            translation_maps: Some(translation_maps),
        }
    }

    /// Process a covariate value based on the accessor method name
    fn process_value(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != self.covariate_type {
            return None;
        }

        match self.accessor.as_str() {
            // Demographics accessors
            "get_family_size" => covariate.get_family_size().map(|v| v as f64),
            "get_municipality" => covariate.get_municipality().map(|v| v as f64),
            "get_family_type" => covariate
                .get_family_type()
                .and_then(|v| v.parse::<f64>().ok()),
            "get_civil_status" => covariate
                .get_civil_status()
                .map(|v| v.bytes().next().unwrap_or(0) as f64),
            "get_gender" => covariate
                .get_gender()
                .map(|v| v.bytes().next().unwrap_or(0) as f64),
            "get_citizenship" => covariate.get_citizenship().map(|v| {
                let mut hash = 0.0;
                for (i, b) in v.bytes().enumerate() {
                    hash += (b as f64) * (i + 1) as f64;
                }
                hash
            }),
            "get_age" => covariate.get_age().map(|v| v as f64),
            "get_children_count" => covariate.get_children_count().map(|v| v as f64),

            // Income accessors
            "get_income_amount" => covariate.get_income_amount(),
            "get_wage_income" => covariate.get_wage_income(),
            "get_employment_status" => covariate.get_employment_status().map(|v| v as f64),

            // Education accessors
            "get_education_level" => covariate
                .get_education_level()
                .map(|v| self.categorical_to_numeric(&v)),
            "get_isced_code" => covariate
                .get_isced_code()
                .map(|v| self.categorical_to_numeric(&v)),
            "get_education_years" => covariate.get_education_years().map(|v| v as f64),

            // Occupation accessors
            "get_occupation_code" => covariate
                .get_occupation_code()
                .map(|v| self.categorical_to_numeric(&v)),
            "get_classification" => covariate
                .get_classification()
                .map(|v| self.categorical_to_numeric(&v)),
            "get_socio" => covariate.get_socio().map(|v| v as f64),
            "get_socio02" => covariate.get_socio02().map(|v| v as f64),
            "get_pre_socio" => covariate.get_pre_socio().map(|v| v as f64),

            // Unknown accessor
            _ => {
                log::warn!("Unknown accessor method: {}", self.accessor);
                None
            }
        }
    }

    /// Process a categorical value based on the accessor method name
    fn process_categorical_value(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != self.covariate_type {
            return None;
        }

        let raw_value = match self.accessor.as_str() {
            // Demographics accessors
            "get_family_size" => return covariate.get_family_size().map(|v| v.to_string()),
            "get_municipality" => return covariate.get_municipality().map(|v| v.to_string()),
            "get_family_type" => covariate.get_family_type(),
            "get_civil_status" => covariate.get_civil_status(),
            "get_gender" => covariate.get_gender(),
            "get_citizenship" => covariate.get_citizenship(),
            "get_age" => return covariate.get_age().map(|v| v.to_string()),
            "get_children_count" => return covariate.get_children_count().map(|v| v.to_string()),

            // Income accessors
            "get_income_amount" => return covariate.get_income_amount().map(|v| v.to_string()),
            "get_wage_income" => return covariate.get_wage_income().map(|v| v.to_string()),
            "get_employment_status" => {
                return covariate.get_employment_status().map(|v| v.to_string())
            }

            // Education accessors
            "get_education_level" => covariate.get_education_level(),
            "get_isced_code" => covariate.get_isced_code(),
            "get_education_years" => return covariate.get_education_years().map(|v| v.to_string()),

            // Occupation accessors
            "get_occupation_code" => covariate.get_occupation_code(),
            "get_classification" => covariate.get_classification(),
            "get_socio" => return covariate.get_socio().map(|v| v.to_string()),
            "get_socio02" => return covariate.get_socio02().map(|v| v.to_string()),
            "get_pre_socio" => return covariate.get_pre_socio().map(|v| v.to_string()),

            // Unknown accessor
            _ => {
                log::warn!("Unknown accessor method: {}", self.accessor);
                return None;
            }
        };

        // Apply translation if available
        if let (Some(translation_name), Some(translation_maps)) =
            (&self.translation, &self.translation_maps)
        {
            if let Some(value) = &raw_value {
                // Map translation type name to enum
                let translation_type = match translation_name.as_str() {
                    "statsb" => Some(TranslationType::Statsb),
                    "civst" => Some(TranslationType::Civst),
                    "family_type" => Some(TranslationType::FamilyType),
                    "fm_mark" => Some(TranslationType::FmMark),
                    "hustype" => Some(TranslationType::Hustype),
                    "reg" => Some(TranslationType::Reg),
                    "socio13" => Some(TranslationType::Socio13),
                    _ => None,
                };

                // Apply translation if type is recognized
                if let Some(trans_type) = translation_type {
                    if let Some(translated) = translation_maps.translate(trans_type, value) {
                        return Some(translated.to_string());
                    }
                }

                // Return original value if translation failed
                return Some(value.to_string());
            }
        }

        raw_value.map(|s| s.to_string())
    }
}

impl CovariateProcessor for ConfigurableVariableProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_covariate_type(&self) -> CovariateType {
        self.covariate_type
    }

    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        self.process_value(covariate)
    }

    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        self.process_categorical_value(covariate)
    }

    fn is_categorical(&self) -> bool {
        matches!(self.variable_type, VariableType::Categorical)
    }

    fn get_variable_type(&self) -> VariableType {
        self.variable_type
    }
}

/// Factory for creating processors from configuration
pub struct ProcessorFactory {
    config: CovariatesConfig,
    translation_maps: Option<TranslationMaps>,
}

impl ProcessorFactory {
    /// Create a new factory from configuration
    pub fn new(config: CovariatesConfig) -> Self {
        Self {
            config,
            translation_maps: None,
        }
    }

    /// Set translation maps for the factory
    pub fn with_translation_maps(mut self, translation_maps: TranslationMaps) -> Self {
        self.translation_maps = Some(translation_maps);
        self
    }

    /// Create a processor for a covariate type
    pub fn create_processor(
        &self,
        covariate_type: CovariateType,
    ) -> Option<Box<dyn CovariateProcessor>> {
        self.config
            .get_covariate_type(covariate_type)
            .map(|config| {
                Box::new(ConfigurableProcessor::new(config)) as Box<dyn CovariateProcessor>
            })
    }

    /// Create a processor for a specific variable
    pub fn create_variable_processor(
        &self,
        covariate_type: CovariateType,
        variable_name: &str,
    ) -> Option<Box<dyn CovariateProcessor>> {
        let covariate_config = self.config.get_covariate_type(covariate_type)?;
        let variable_config = covariate_config
            .variables
            .iter()
            .find(|v| v.name == variable_name)?;

        if let Some(translation_maps) = &self.translation_maps {
            Some(
                Box::new(ConfigurableVariableProcessor::with_translation_maps(
                    variable_config,
                    covariate_type,
                    translation_maps.clone(),
                )) as Box<dyn CovariateProcessor>,
            )
        } else {
            Some(Box::new(ConfigurableVariableProcessor::new(
                variable_config,
                covariate_type,
            )) as Box<dyn CovariateProcessor>)
        }
    }

    /// Create all processors for all covariate types
    pub fn create_all_processors(&self) -> anyhow::Result<Vec<Box<dyn CovariateProcessor>>> {
        // Early validation - check if configuration is valid
        if self.config.covariate_types.is_empty() {
            return Err(anyhow::anyhow!("No covariate types defined in configuration"));
        }
        
        // Log what we're creating
        log::debug!("Creating processors for {} covariate types", self.config.covariate_types.len());
        
        // Create processors
        let processors = self.config
            .covariate_types
            .iter()
            .map(|config| {
                // Log each processor being created
                log::trace!("Creating processor for {}", config.name);
                Box::new(ConfigurableProcessor::new(config)) as Box<dyn CovariateProcessor>
            })
            .collect();
            
        Ok(processors)
    }

    /// Create processors for all variables in a specific covariate type
    pub fn create_variable_processors_for_type(
        &self,
        covariate_type: CovariateType,
    ) -> anyhow::Result<Vec<Box<dyn CovariateProcessor>>> {
        if let Some(covariate_config) = self.config.get_covariate_type(covariate_type) {
            // Check if we have variables defined
            if covariate_config.variables.is_empty() {
                return Err(anyhow::anyhow!(
                    "No variables defined for covariate type {:?}", covariate_type
                ).context(format!("Configuration for {}", covariate_config.name)));
            }
            
            // Log what we're creating
            log::debug!(
                "Creating processors for {} variables of type {:?}", 
                covariate_config.variables.len(), 
                covariate_type
            );
            
            // Create variable processors
            let processors = covariate_config
                .variables
                .iter()
                .map(|var_config| {
                    log::trace!("Creating processor for variable {}", var_config.name);
                    
                    if let Some(translation_maps) = &self.translation_maps {
                        Box::new(ConfigurableVariableProcessor::with_translation_maps(
                            var_config,
                            covariate_type,
                            translation_maps.clone(),
                        )) as Box<dyn CovariateProcessor>
                    } else {
                        Box::new(ConfigurableVariableProcessor::new(
                            var_config,
                            covariate_type,
                        )) as Box<dyn CovariateProcessor>
                    }
                })
                .collect();
                
            Ok(processors)
        } else {
            // No configuration for this type
            Err(anyhow::anyhow!(
                "No configuration found for covariate type {:?}", covariate_type
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use crate::config::CovariatesConfig;
    use types::models::{Covariate, DemographicExtras};

    #[test]
    fn test_create_processor_from_config() {
        let config = CovariatesConfig::default_config();
        let factory = ProcessorFactory::new(config);

        // Create a processor for demographics
        let processor = factory
            .create_processor(CovariateType::Demographics)
            .unwrap();

        assert_eq!(processor.get_name(), "Demographics");
        assert_eq!(processor.get_covariate_type(), CovariateType::Demographics);
    }

    #[test]
    fn test_create_variable_processor() {
        let config = CovariatesConfig::default_config();
        let factory = ProcessorFactory::new(config);

        // Create a processor for a specific variable
        let processor = factory
            .create_variable_processor(CovariateType::Demographics, "Age")
            .unwrap();

        assert_eq!(processor.get_name(), "Age");
        assert_eq!(processor.get_covariate_type(), CovariateType::Demographics);
        assert_eq!(processor.get_variable_type(), VariableType::Numeric);

        // Create a test covariate
        let covariate = Covariate::demographics_with_extras(
            2,               // family_size
            101,             // municipality
            "1".to_string(), // family_type
            DemographicExtras {
                civil_status: Some("G".to_string()),
                gender: Some("M".to_string()),
                citizenship: Some("5100".to_string()),
                age: Some(42),
                children_count: Some(2),
            },
        );

        // Test the processor
        let value = processor.process_numeric(&covariate);
        assert_eq!(value, Some(42.0));
    }

    #[test]
    fn test_create_all_processors() -> Result<()> {
        let config = CovariatesConfig::default_config();
        let factory = ProcessorFactory::new(config);

        // Create all processors
        let processors = factory.create_all_processors()?;

        // Should have one processor for each covariate type
        assert_eq!(processors.len(), 4);

        // Verify that we have a processor for each type
        let types: Vec<CovariateType> = processors.iter().map(|p| p.get_covariate_type()).collect();

        assert!(types.contains(&CovariateType::Demographics));
        assert!(types.contains(&CovariateType::Income));
        assert!(types.contains(&CovariateType::Education));
        assert!(types.contains(&CovariateType::Occupation));
        
        Ok(())
    }
    
    #[test]
    fn test_empty_config_returns_error() {
        let config = CovariatesConfig {
            covariate_types: vec![], // Empty config
        };
        let factory = ProcessorFactory::new(config);
        
        // Should return an error because the config is empty
        let result = factory.create_all_processors();
        assert!(result.is_err());
        
        // Check error message contains useful information
        let err = result.unwrap_err();
        assert!(err.to_string().contains("No covariate types defined"));
    }

    #[test]
    fn test_create_variable_processors_for_type() -> Result<()> {
        let config = CovariatesConfig::default_config();
        let factory = ProcessorFactory::new(config);

        // Create all processors for Demographics
        let processors = factory.create_variable_processors_for_type(CovariateType::Demographics)?;

        // Should have the correct number of processors
        assert_eq!(processors.len(), 8); // 8 demographic variables

        // Check that we have processor for specific variables
        let variable_names: Vec<&str> = processors.iter().map(|p| p.get_name()).collect();

        assert!(variable_names.contains(&"Age"));
        assert!(variable_names.contains(&"Family Size"));
        assert!(variable_names.contains(&"Family Type"));
        assert!(variable_names.contains(&"Gender"));
        
        Ok(())
    }
    
    #[test]
    fn test_invalid_covariate_type_returns_error() {
        let config = CovariatesConfig::default_config();
        let factory = ProcessorFactory::new(config);
        
        // Create a test config that only has Demographics defined
        let reduced_config = CovariatesConfig {
            covariate_types: vec![
                config.covariate_types[0].clone(), // Just Demographics
            ],
        };
        let reduced_factory = ProcessorFactory::new(reduced_config);
        
        // Should return error for a non-existent type
        let result = reduced_factory.create_variable_processors_for_type(CovariateType::Occupation);
        assert!(result.is_err());
        
        // Check error message contains the missing type
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Occupation"));
    }
}
