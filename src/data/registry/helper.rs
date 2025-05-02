//! Helper functions for registry operations.
//!
//! This module provides helper functions for common registry operations.

use crate::data::registry::factory::RegistryFactory;
use crate::data::registry::loaders::bef::BefRegister;
use crate::data::registry::loaders::lpr::{Lpr2Register, Lpr3Register};
use crate::data::registry::loaders::mfr::MfrRegister;
use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::error::Result;
use crate::utils::runtime::get_runtime;
use arrow::record_batch::RecordBatch;
use std::path::Path;

/// Load registry data based on the registry name or path
///
/// This function simplifies loading registry data from a path by handling
/// the common pattern of creating a registry loader, downcasting, and loading.
///
/// # Arguments
/// * `path` - Path to the registry data
/// * `registry_name` - Optional registry name (if None, will infer from path)
/// * `pnr_filter` - Optional filter for PNRs
///
/// # Returns
/// * `Result<Vec<RecordBatch>>` - Arrow record batches containing the loaded data
pub async fn load_registry_data(
    path: &(impl AsRef<Path> + ?Sized),
    registry_name: Option<&str>,
    pnr_filter: Option<&PnrFilter>,
) -> Result<Vec<RecordBatch>> {
    // Create the registry loader - either by name or by path
    let registry = if let Some(name) = registry_name {
        RegistryFactory::from_name(name)?
    } else {
        RegistryFactory::from_path(path.as_ref())?
    };
    
    // Try to downcast to various types
    if let Some(loader) = registry.downcast_ref::<BefRegister>() {
        loader.load(path.as_ref().to_string_lossy().as_ref(), pnr_filter).await
    } else if let Some(loader) = registry.downcast_ref::<Lpr2Register>() {
        loader.load(path.as_ref().to_string_lossy().as_ref(), pnr_filter).await
    } else if let Some(loader) = registry.downcast_ref::<Lpr3Register>() {
        loader.load(path.as_ref().to_string_lossy().as_ref(), pnr_filter).await
    } else if let Some(loader) = registry.downcast_ref::<MfrRegister>() {
        loader.load(path.as_ref().to_string_lossy().as_ref(), pnr_filter).await
    } else {
        // If we can't determine the type, try to use a generic DataFusion approach
        let mut reader = crate::data::io::parquet::ParquetReader::new(path.as_ref());
        reader.read_async().await
    }
}

/// Synchronous version of `load_registry_data` that uses a Tokio runtime
pub fn load_registry_data_sync(
    path: &(impl AsRef<Path> + ?Sized),
    registry_name: Option<&str>,
    pnr_filter: Option<&PnrFilter>,
) -> Result<Vec<RecordBatch>> {
    // Get the runtime
    let runtime = get_runtime()?;
    
    // Run the async function in the runtime
    runtime.block_on(async {
        load_registry_data(path, registry_name, pnr_filter).await
    })
}