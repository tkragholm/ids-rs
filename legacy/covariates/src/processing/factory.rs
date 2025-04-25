use anyhow::Result;
use std::collections::HashMap;
use types::models::{Covariate, CovariateType};
use types::traits::{CovariateProcessor, VariableType};
use types::translation::{TranslationMaps, TranslationType};

use crate::core::config::{CovariateTypeConfig, CovariateVariableConfig, CovariatesConfig};
use crate::processing::processor::ConfigurableVariableProcessor;

/// A configurable covariate processor built from configuration
pub struct ConfigurableProcessorImpl {
    name: String,
    covariate_type: CovariateType,
    #[allow(dead_code)]
    variables: HashMap<String, VariableType>,
}

impl ConfigurableProcessorImpl {
    /// Create a new processor from configuration
    #[must_use] pub fn new(config: &CovariateTypeConfig) -> Self {
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

impl CovariateProcessor for ConfigurableProcessorImpl {
    fn process(
        &self,
        _store: &dyn types::traits::access::Store,
        _year: i32,
    ) -> types::error::Result<Covariate> {
        // Default implementation, will be overridden by concrete processors
        Err(types::error::IdsError::invalid_operation(
            "Not implemented".to_string(),
        ))
    }

    fn covariate_type(&self) -> CovariateType {
        self.covariate_type
    }

    fn required_fields(&self) -> Vec<String> {
        // Default to empty list - concrete implementations will provide actual fields
        Vec::new()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_categorical(&self) -> bool {
        // Default to false - specialized implementations will determine this
        false
    }

    fn process_numeric(&self, _covariate: &Covariate) -> Option<f64> {
        // Default to None - specialized implementations will be needed
        None
    }

    fn process_categorical(&self, _covariate: &Covariate) -> Option<String> {
        // Default to None - specialized implementations will be needed
        None
    }
}

/// A configurable variable processor built from configuration
pub struct ConfigurableVariableProcessorImpl {
    name: String,
    variable_type: VariableType,
    covariate_type: CovariateType,
    accessor: String,
    translation: Option<String>,
    translation_maps: Option<TranslationMaps>,
    description: Option<String>,
}

impl ConfigurableVariableProcessorImpl {
    /// Create a new variable processor from configuration
    #[must_use] pub fn new(config: &CovariateVariableConfig, covariate_type: CovariateType) -> Self {
        Self {
            name: config.name.clone(),
            variable_type: config.variable_type,
            covariate_type,
            accessor: config.accessor.clone(),
            translation: config.translation.clone(),
            translation_maps: None,
            description: config.description.clone(),
        }
    }

    /// Create a new variable processor with translation maps
    #[must_use] pub fn with_translation_maps(
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
            description: config.description.clone(),
        }
    }

    /// Convert a categorical value to a numeric code for numeric operations
    fn categorical_to_numeric(&self, value: &str) -> f64 {
        // Simple hash function to convert categorical values to numeric
        let mut hash = 0.0;
        for (i, b) in value.bytes().enumerate() {
            hash += f64::from(b) * (i + 1) as f64;
        }
        hash
    }

    /// Process a covariate value based on the accessor method name
    fn process_value(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.type_() != self.covariate_type {
            return None;
        }

        match self.accessor.as_str() {
            // Demographics accessors - support both old and new method names
            "get_family_size" | "family_size" => covariate.family_size().map(f64::from),
            "get_municipality" | "municipality" => covariate.municipality().map(f64::from),
            "get_family_type" | "family_type" => {
                covariate.family_type().and_then(|v| v.parse::<f64>().ok())
            }
            "get_civil_status" | "civil_status" => covariate
                .civil_status()
                .map(|v| f64::from(v.bytes().next().unwrap_or(0))), // unwrap_or(0) is safe - default to 0 if empty string
            "get_gender" | "gender" => covariate
                .gender()
                .map(|v| f64::from(v.bytes().next().unwrap_or(0))), // unwrap_or(0) is safe - default to 0 if empty string
            "get_citizenship" | "citizenship" => covariate.citizenship().map(|v| {
                let mut hash = 0.0;
                for (i, b) in v.bytes().enumerate() {
                    hash += f64::from(b) * (i + 1) as f64;
                }
                hash
            }),
            "get_age" | "age" => covariate.age().map(f64::from),
            "get_children_count" | "children_count" => covariate.children_count().map(f64::from),

            // Income accessors
            "get_income_amount" | "income_amount" => covariate.income_amount(),
            "get_wage_income" | "wage_income" => covariate.wage_income(),
            "get_employment_status" | "employment_status" => {
                covariate.employment_status().map(f64::from)
            }

            // Education accessors
            "get_education_level" | "education_level" => covariate
                .education_level()
                .map(|v| self.categorical_to_numeric(&v)),
            "get_isced_code" | "isced_code" => covariate
                .isced_code()
                .map(|v| self.categorical_to_numeric(&v)),
            "get_education_years" | "education_years" => {
                covariate.education_years().map(f64::from)
            }

            // Occupation accessors
            "get_occupation_code" | "occupation_code" => covariate
                .occupation_code()
                .map(|v| self.categorical_to_numeric(&v)),
            "get_classification" | "classification" => covariate
                .classification()
                .map(|v| self.categorical_to_numeric(&v)),
            "get_socio" | "socio" => covariate.socio().map(f64::from),
            "get_socio02" | "socio02" => covariate.socio02().map(f64::from),
            "get_pre_socio" | "pre_socio" => covariate.pre_socio().map(f64::from),

            // Unknown accessor
            _ => {
                log::warn!("Unknown accessor method: {}", self.accessor);
                None
            }
        }
    }

    /// Process a categorical value based on the accessor method name
    fn process_categorical_value(&self, covariate: &Covariate) -> Option<String> {
        if covariate.type_() != self.covariate_type {
            return None;
        }

        let raw_value = match self.accessor.as_str() {
            // Demographics accessors - support both old and new method names
            "get_family_size" | "family_size" => {
                return covariate.family_size().map(|v| v.to_string())
            }
            "get_municipality" | "municipality" => {
                return covariate.municipality().map(|v| v.to_string())
            }
            "get_family_type" | "family_type" => covariate.family_type(),
            "get_civil_status" | "civil_status" => covariate.civil_status(),
            "get_gender" | "gender" => covariate.gender(),
            "get_citizenship" | "citizenship" => covariate.citizenship(),
            "get_age" | "age" => return covariate.age().map(|v| v.to_string()),
            "get_children_count" | "children_count" => {
                return covariate.children_count().map(|v| v.to_string())
            }

            // Income accessors
            "get_income_amount" | "income_amount" => {
                return covariate.income_amount().map(|v| v.to_string())
            }
            "get_wage_income" | "wage_income" => {
                return covariate.wage_income().map(|v| v.to_string())
            }
            "get_employment_status" | "employment_status" => {
                return covariate.employment_status().map(|v| v.to_string())
            }

            // Education accessors
            "get_education_level" | "education_level" => covariate.education_level(),
            "get_isced_code" | "isced_code" => covariate.isced_code(),
            "get_education_years" | "education_years" => {
                return covariate.education_years().map(|v| v.to_string())
            }

            // Occupation accessors
            "get_occupation_code" | "occupation_code" => covariate.occupation_code(),
            "get_classification" | "classification" => covariate.classification(),
            "get_socio" | "socio" => return covariate.socio().map(|v| v.to_string()),
            "get_socio02" | "socio02" => return covariate.socio02().map(|v| v.to_string()),
            "get_pre_socio" | "pre_socio" => return covariate.pre_socio().map(|v| v.to_string()),

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

impl CovariateProcessor for ConfigurableVariableProcessorImpl {
    fn process(
        &self,
        _store: &dyn types::traits::access::Store,
        _year: i32,
    ) -> types::error::Result<Covariate> {
        // Default implementation, will be overridden by concrete processors
        Err(types::error::IdsError::invalid_operation(
            "Not implemented".to_string(),
        ))
    }

    fn covariate_type(&self) -> CovariateType {
        self.covariate_type
    }

    fn required_fields(&self) -> Vec<String> {
        // Default to empty list - concrete implementations will provide actual fields
        Vec::new()
    }

    fn name(&self) -> &str {
        &self.name
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

    fn variable_type(&self) -> VariableType {
        self.variable_type
    }
}

impl ConfigurableVariableProcessor for ConfigurableVariableProcessorImpl {
    fn variable_type() -> VariableType {
        VariableType::Categorical // Default, will be overridden
    }

    fn from_config(_config: &CovariateVariableConfig) -> Result<Self, crate::core::Error>
    where
        Self: Sized,
    {
        // This implementation requires covariate type, so it's not fully implemented here
        Err(crate::core::Error::config(
            "Incomplete implementation - use factory",
        ))
    }

    fn process(&self, _covariate: &Covariate) -> Option<types::models::CovariateValue> {
        // This would be implemented in specialized processors
        None
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Factory for creating processors from configuration
pub struct ProcessorFactory {
    config: CovariatesConfig,
    translation_maps: Option<TranslationMaps>,
}

impl ProcessorFactory {
    /// Create a new factory from configuration
    #[must_use] pub fn new(config: CovariatesConfig) -> Self {
        Self {
            config,
            translation_maps: None,
        }
    }

    /// Set translation maps for the factory
    #[must_use] pub fn with_translation_maps(mut self, translation_maps: TranslationMaps) -> Self {
        self.translation_maps = Some(translation_maps);
        self
    }

    /// Create a processor for a covariate type
    #[must_use] pub fn create_processor(
        &self,
        covariate_type: CovariateType,
    ) -> Option<Box<dyn CovariateProcessor>> {
        self.config
            .get_covariate_type(covariate_type)
            .map(|config| {
                Box::new(ConfigurableProcessorImpl::new(config)) as Box<dyn CovariateProcessor>
            })
    }

    /// Create a processor for a specific variable
    #[must_use] pub fn create_variable_processor(
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
                Box::new(ConfigurableVariableProcessorImpl::with_translation_maps(
                    variable_config,
                    covariate_type,
                    translation_maps.clone(),
                )) as Box<dyn CovariateProcessor>,
            )
        } else {
            Some(Box::new(ConfigurableVariableProcessorImpl::new(
                variable_config,
                covariate_type,
            )) as Box<dyn CovariateProcessor>)
        }
    }

    /// Create all processors for all covariate types
    pub fn create_all_processors(&self) -> Result<Vec<Box<dyn CovariateProcessor>>> {
        // Early validation - check if configuration is valid
        if self.config.covariate_types.is_empty() {
            return Err(anyhow::anyhow!(
                "No covariate types defined in configuration"
            ));
        }

        // Log what we're creating
        log::debug!(
            "Creating processors for {} covariate types",
            self.config.covariate_types.len()
        );

        // Create processors
        let processors = self
            .config
            .covariate_types
            .iter()
            .map(|config| {
                // Log each processor being created
                log::trace!("Creating processor for {}", config.name);
                Box::new(ConfigurableProcessorImpl::new(config)) as Box<dyn CovariateProcessor>
            })
            .collect();

        Ok(processors)
    }

    /// Create processors for all variables in a specific covariate type
    pub fn create_variable_processors_for_type(
        &self,
        covariate_type: CovariateType,
    ) -> Result<Vec<Box<dyn CovariateProcessor>>> {
        if let Some(covariate_config) = self.config.get_covariate_type(covariate_type) {
            // Check if we have variables defined
            if covariate_config.variables.is_empty() {
                return Err(anyhow::anyhow!(
                    "No variables defined for covariate type {:?}",
                    covariate_type
                )
                .context(format!("Configuration for {}", covariate_config.name)));
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
                        Box::new(ConfigurableVariableProcessorImpl::with_translation_maps(
                            var_config,
                            covariate_type,
                            translation_maps.clone(),
                        )) as Box<dyn CovariateProcessor>
                    } else {
                        Box::new(ConfigurableVariableProcessorImpl::new(
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
                "No configuration found for covariate type {:?}",
                covariate_type
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::CovariatesConfig;
    use types::models::Covariate;

    #[test]
    fn test_create_processor_from_config() {
        let config = CovariatesConfig::default_config();
        let factory = ProcessorFactory::new(config);

        // Create a processor for demographics
        let processor = match factory.create_processor(CovariateType::Demographics) {
            Some(p) => p,
            None => panic!("Demographics processor should be available in default config"),
        };

        assert_eq!(processor.name(), "Demographics");
        assert_eq!(processor.covariate_type(), CovariateType::Demographics);
    }

    #[test]
    fn test_create_variable_processor() {
        let config = CovariatesConfig::default_config();
        let factory = ProcessorFactory::new(config);

        // Create a processor for a specific variable
        let processor = match factory.create_variable_processor(CovariateType::Demographics, "Age")
        {
            Some(p) => p,
            None => panic!("Age variable processor should be available in default config"),
        };

        assert_eq!(processor.name(), "Age");
        assert_eq!(processor.covariate_type(), CovariateType::Demographics);
        assert_eq!(processor.variable_type(), VariableType::Numeric);

        // Create a test covariate

        let covariate = Covariate::demographics(
            2,               // family_size
            101,             // municipality
            "1".to_string(), // family_type
        )
        .with_civil_status("G".to_string())
        .with_gender("M".to_string())
        .with_citizenship("5100".to_string())
        .with_age(42)
        .with_children_count(2)
        .build();

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
        let types: Vec<CovariateType> = processors.iter().map(|p| p.covariate_type()).collect();

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
            settings: HashMap::new(),
        };
        let factory = ProcessorFactory::new(config);

        // Should return an error because the config is empty
        let result = factory.create_all_processors();
        assert!(result.is_err());

        // Check if there is an error as expected
        if let Err(err) = result {
            assert!(err.to_string().contains("No covariate types defined"));
        } else {
            panic!("Expected an error but got Ok");
        }
    }

    #[test]
    fn test_create_variable_processors_for_type() -> Result<()> {
        let config = CovariatesConfig::default_config();
        let factory = ProcessorFactory::new(config);

        // Create all processors for Demographics
        let processors =
            factory.create_variable_processors_for_type(CovariateType::Demographics)?;

        // Should have the correct number of processors
        assert_eq!(processors.len(), 8); // 8 demographic variables

        // Check that we have processor for specific variables
        let variable_names: Vec<&str> = processors.iter().map(|p| p.name()).collect();

        assert!(variable_names.contains(&"Age"));
        assert!(variable_names.contains(&"Family Size"));
        assert!(variable_names.contains(&"Family Type"));
        assert!(variable_names.contains(&"Gender"));

        Ok(())
    }

    #[test]
    fn test_invalid_covariate_type_returns_error() {
        let config = CovariatesConfig::default_config();
        // Create a copy of just the Demographics type from the config
        let demographics_type = config.covariate_types[0].clone();

        // Create a test config that only has Demographics defined
        let reduced_config = CovariatesConfig {
            covariate_types: vec![
                demographics_type, // Just Demographics
            ],
            settings: HashMap::new(),
        };
        let reduced_factory = ProcessorFactory::new(reduced_config);

        // Should return error for a non-existent type
        let result = reduced_factory.create_variable_processors_for_type(CovariateType::Occupation);
        assert!(result.is_err());

        // Check if there is an error as expected
        if let Err(err) = result {
            assert!(err.to_string().contains("Occupation"));
        } else {
            panic!("Expected an error but got Ok");
        }
    }
}
