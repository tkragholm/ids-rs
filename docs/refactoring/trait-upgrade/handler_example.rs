use crate::algorithm::population::core::{generate_population, PopulationConfig};
use crate::commands::population::config::PopulationCommandConfig;
use crate::data::registry::traits_proposal::RegisterLoaderFactory;
use crate::error::{IdsError, Result};
use crate::utils::reports::save_population_summary;
use arrow::record_batch::RecordBatch;
use datafusion::common::config::TableParquetOptions;
use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::prelude::*;
use log::info;
use std::fs;
use tokio::runtime::Runtime;

/// Handle the population generation command - no downcasting required!
pub fn handle_population_command(config: &PopulationCommandConfig) -> Result<()> {
    info!("Starting population generation");

    // Create algorithm configuration
    let algo_config = PopulationConfig {
        birth_inclusion_start_year: config.birth_inclusion_start_year,
        birth_inclusion_end_year: config.birth_inclusion_end_year,
        include_death_data: true,
        include_death_cause_data: true,
        include_migration_data: true,
    };

    // Create a tokio runtime for async operations
    let runtime = Runtime::new()
        .map_err(|e| IdsError::Data(format!("Failed to create async runtime: {e}")))?;

    // Load BEF data using the trait object
    info!("Reading BEF data from: {:?}", config.bef_path);
    let bef_data_vec = runtime.block_on(async {
        // Get the registry loader using the factory
        let loader = RegisterLoaderFactory::from_name("bef")?;
        
        // Use the loader directly without downcasting
        let base_path = config.bef_path.to_str().unwrap_or("");
        loader.load(base_path, None).await
    })?;

    if bef_data_vec.is_empty() {
        return Err(IdsError::Data("No BEF data found".to_string()));
    }

    // Combine all batches into a single RecordBatch
    info!("Found {} BEF batches", bef_data_vec.len());
    let bef_data = combine_record_batches(&bef_data_vec)?;
    info!("Loaded {} rows from BEF data", bef_data.num_rows());

    // Load MFR data using the trait object
    info!("Reading MFR data from: {:?}", config.mfr_path);
    let mfr_data_vec = runtime.block_on(async {
        // Get the registry loader using the factory
        let loader = RegisterLoaderFactory::from_name("mfr")?;
        
        // Use the loader directly without downcasting
        let base_path = config.mfr_path.to_str().unwrap_or("");
        loader.load(base_path, None).await
    })?;

    // Rest of the function remains the same
    // ...

    Ok(())
}

// Helper function to combine record batches
fn combine_record_batches(batches: &[RecordBatch]) -> Result<RecordBatch> {
    // Implementation remains the same
    todo!()
}