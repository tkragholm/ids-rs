use std::collections::HashMap;
use types::models::CovariateType;
use types::traits::CovariateProcessor;
use types::translation::TranslationMaps;

use crate::core::config::CovariatesConfig;
use crate::processing::demographic::DemographicsProcessor;
use crate::processing::education::EducationProcessor;
use crate::processing::factory::ProcessorFactory;
use crate::processing::income::IncomeProcessor;
use crate::processing::occupation::OccupationProcessor;

/// Registry for covariate processors
pub struct CovariateProcessorRegistry {
    processors: HashMap<CovariateType, Box<dyn CovariateProcessor>>,
}

impl CovariateProcessorRegistry {
    /// Create a new registry with all standard processors
    #[must_use] pub fn new() -> Self {
        let mut registry = Self {
            processors: HashMap::new(),
        };

        // Register all standard processors
        registry.register(Box::new(DemographicsProcessor::new()));
        registry.register(Box::new(EducationProcessor::new()));
        registry.register(Box::new(IncomeProcessor::new()));
        registry.register(Box::new(OccupationProcessor::new()));

        registry
    }

    /// Create a new registry from configuration
    #[must_use] pub fn from_config(config: CovariatesConfig) -> Self {
        Self::from_config_with_translations(config, None)
    }

    /// Create a new registry from configuration with translation maps
    #[must_use] pub fn from_config_with_translations(
        config: CovariatesConfig,
        translation_maps: Option<TranslationMaps>,
    ) -> Self {
        let mut registry = Self {
            processors: HashMap::new(),
        };

        let factory = match translation_maps {
            Some(maps) => ProcessorFactory::new(config).with_translation_maps(maps),
            None => ProcessorFactory::new(config),
        };

        // Create and register processors for all covariate types
        match factory.create_all_processors() {
            Ok(processors) => {
                for processor in processors {
                    registry.register(processor);
                }
            }
            Err(e) => {
                // Log the error but continue with empty registry
                log::warn!("Failed to create processors: {e}");
            }
        }

        registry
    }

    /// Register a new processor
    pub fn register(&mut self, processor: Box<dyn CovariateProcessor>) {
        self.processors
            .insert(processor.covariate_type(), processor);
    }

    /// Get a processor by covariate type
    #[must_use] pub fn get(&self, covariate_type: CovariateType) -> Option<&dyn CovariateProcessor> {
        self.processors.get(&covariate_type).map(std::convert::AsRef::as_ref)
    }

    /// Get all registered processors
    #[must_use] pub fn get_all(&self) -> Vec<&dyn CovariateProcessor> {
        self.processors.values().map(std::convert::AsRef::as_ref).collect()
    }

    /// Get all covariate types
    #[must_use] pub fn get_all_types(&self) -> Vec<CovariateType> {
        self.processors.keys().copied().collect()
    }

    /// Get all processors for a specific covariate type
    #[must_use] pub fn get_processors_for_type(
        &self,
        covariate_type: CovariateType,
    ) -> Vec<&dyn CovariateProcessor> {
        // In the current implementation, we only have one processor per type,
        // but we return a Vec for future extensibility
        if let Some(processor) = self.get(covariate_type) {
            vec![processor]
        } else {
            Vec::new()
        }
    }
}

impl Default for CovariateProcessorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
