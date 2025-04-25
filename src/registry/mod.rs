//! Registry definitions and loaders for the IDS-RS library
//!
//! This module contains registry definitions and loaders for various Danish registry data sources.
//! 
//! Available registries:
//! - AKM (Arbejdsklassifikationsmodulet): Employment information
//! - BEF (Befolkning): Population demographic information
//! - IDAN (Integrated Database for Labor Market Research): Employment information
//! - IND (Indkomst): Income and tax information
//! - UDDF (Uddannelse): Educational information

use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use std::collections::HashSet;
use std::path::Path;

/// Base trait for registry loaders
pub trait RegisterLoader {
    /// Get the name of the register
    fn get_register_name(&self) -> &'static str;
    
    /// Load records from the register
    fn load(&self, base_path: &str, pnr_filter: Option<&HashSet<String>>) -> Result<Vec<RecordBatch>>;
}

pub mod akm;
pub mod bef;
pub mod idan;
pub mod ind;
pub mod uddf;
pub mod transform;

// Re-export registry structs for easier access
pub use akm::AkmRegister;
pub use bef::BefRegister;
pub use idan::IdanRegister;
pub use ind::IndRegister;
pub use uddf::UddfRegister;

// Re-export transform functions
pub use transform::{
    transform_records,
    filter_by_date_range,
    add_year_column,
    filter_out_missing_values,
    map_categorical_values,
    scale_numeric_values,
    add_postal_code_region,
};

/// Create a registry loader from a registry name
pub fn registry_from_name(name: &str) -> Result<Box<dyn RegisterLoader>> {
    match name.to_lowercase().as_str() {
        "akm" => Ok(Box::new(akm::AkmRegister)),
        "bef" => Ok(Box::new(bef::BefRegister)),
        "idan" => Ok(Box::new(idan::IdanRegister)),
        "ind" => Ok(Box::new(ind::IndRegister)),
        "uddf" => Ok(Box::new(uddf::UddfRegister)),
        _ => Err(IdsError::Validation(format!("Unknown registry: {name}"))),
    }
}

/// Create a registry loader based on a path (inferring the registry type from the path)
pub fn registry_from_path(path: &str) -> Result<Box<dyn RegisterLoader>> {
    let path = Path::new(path);
    
    // Try to infer registry from directory name
    if let Some(dir_name) = path.file_name().and_then(|f| f.to_str()) {
        let lower_name = dir_name.to_lowercase();
        
        // Check for registry name patterns in the path
        if lower_name.contains("akm") {
            return Ok(Box::new(akm::AkmRegister));
        } else if lower_name.contains("bef") {
            return Ok(Box::new(bef::BefRegister));
        } else if lower_name.contains("idan") {
            return Ok(Box::new(idan::IdanRegister));
        } else if lower_name.contains("ind") {
            return Ok(Box::new(ind::IndRegister));
        } else if lower_name.contains("uddf") || lower_name.contains("uddannelse") {
            return Ok(Box::new(uddf::UddfRegister));
        }
    }
    
    // If we can't infer from the path, return an error
    Err(IdsError::Validation(format!(
        "Could not determine registry type from path: {}",
        path.display()
    )))
}

/// Load data from multiple registries and combine them
pub fn load_multiple_registries(
    base_paths: &[(&str, &str)], // (registry_name, path)
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>> {
    let mut all_batches = Vec::new();
    
    for (registry_name, path) in base_paths {
        let registry = registry_from_name(registry_name)?;
        let batches = registry.load(path, pnr_filter)?;
        all_batches.extend(batches);
    }
    
    Ok(all_batches)
}