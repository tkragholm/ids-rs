use crate::data::registry::traits::RegisterLoader;
use crate::data::schema::traits::DynamicSchema;
use crate::error::{IdsError, Result};
use std::collections::HashMap;
use std::path::Path;

use super::loaders::akm::AkmRegister;

/// Factory for creating registry loaders
pub struct RegistryFactory;

impl RegistryFactory {
    /// Create a registry loader from a registry name
    pub fn from_name(name: &str) -> Result<Box<dyn RegisterLoader<SchemaType = DynamicSchema>>> {
        match name.to_lowercase().as_str() {
            "akm" => Ok(Box::new(AkmRegister)),
            // Add more registries here as they are implemented
            _ => Err(IdsError::Validation(format!("Unknown registry: {name}"))),
        }
    }

    /// Create a registry loader from a path (inferring the registry type from the path)
    pub fn from_path(path: &Path) -> Result<Box<dyn RegisterLoader>> {
        let path_str = path.to_string_lossy().to_lowercase();

        // Try to infer registry from directory name
        if path_str.contains("akm") {
            return Ok(Box::new(AkmRegister));
        }
        // Add more registries here as they are implemented

        // If we can't infer from the path, return an error
        Err(IdsError::Validation(format!(
            "Could not determine registry type from path: {}",
            path.display()
        )))
    }

    /// Create instances of all available loaders
    pub fn create_all() -> HashMap<&'static str, Box<dyn RegisterLoader>> {
        let mut loaders = HashMap::new();
        loaders.insert(
            AkmRegister::register_name(),
            Box::new(AkmRegister) as Box<dyn RegisterLoader>,
        );
        // Add more registries here as they are implemented
        loaders
    }
}
