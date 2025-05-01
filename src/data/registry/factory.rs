use crate::data::registry::traits::{AnyRegisterLoader, RegisterLoader};
use crate::data::schema::traits::RegistrySchema;
use crate::error::{IdsError, Result};
use std::collections::HashMap;
use std::path::Path;

use super::loaders::akm::AkmRegister;
use super::loaders::lpr::{Lpr2Register, Lpr3Register};

/// Factory for creating registry loaders with any schema type
pub struct RegistryFactory;

impl RegistryFactory {
    /// Create a registry loader from a registry name
    pub fn from_name(name: &str) -> Result<AnyRegisterLoader> {
        match name.to_lowercase().as_str() {
            "akm" => Ok(Box::new(AkmRegister)),
            "lpr2" => Ok(Box::new(Lpr2Register)),
            "lpr3" => Ok(Box::new(Lpr3Register)),
            // Add more registries here as they are implemented
            _ => Err(IdsError::Validation(format!("Unknown registry: {name}"))),
        }
    }

    /// Create a registry loader from a path (inferring the registry type from the path)
    pub fn from_path(path: &Path) -> Result<AnyRegisterLoader> {
        let path_str = path.to_string_lossy().to_lowercase();

        // Try to infer registry from directory name
        if path_str.contains("akm") {
            return Ok(Box::new(AkmRegister));
        } else if path_str.contains("lpr_") || path_str.contains("lpr2") {
            return Ok(Box::new(Lpr2Register));
        } else if path_str.contains("lpr3") {
            return Ok(Box::new(Lpr3Register));
        }
        // Add more registries here as they are implemented

        // If we can't infer from the path, return an error
        Err(IdsError::Validation(format!(
            "Could not determine registry type from path: {}",
            path.display()
        )))
    }

    /// Create instances of all available loaders, mapped by name
    #[must_use] pub fn create_all() -> HashMap<&'static str, AnyRegisterLoader> {
        let mut loaders = HashMap::<&'static str, AnyRegisterLoader>::new();
        
        // Add AKM
        let akm = AkmRegister;
        loaders.insert(akm.register_name(), Box::new(akm) as AnyRegisterLoader);
        
        // Add LPR2
        let lpr2 = Lpr2Register;
        loaders.insert(lpr2.register_name(), Box::new(lpr2) as AnyRegisterLoader);
        
        // Add LPR3
        let lpr3 = Lpr3Register;
        loaders.insert(lpr3.register_name(), Box::new(lpr3) as AnyRegisterLoader);
        
        // Add more registries here as they are implemented
        loaders
    }
    
    /// Get schema name from the registry loader
    #[must_use] pub fn get_schema_name<S: RegistrySchema>() -> &'static str {
        std::any::type_name::<S>()
    }
}
