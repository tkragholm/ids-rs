//! Handler for the population generation command

use std::fs;
use log::info;

use crate::error::Result;
use crate::commands::population::config::PopulationCommandConfig;
use crate::algorithm::population::{PopulationConfig, generate_population};
use crate::utils::reports::save_population_summary;
use crate::schema::parquet::read_parquet;

/// Handle the population generation command
pub fn handle_population_command(config: &PopulationCommandConfig) -> Result<()> {
    info!("Starting population generation");
    
    // Create algorithm configuration
    let algo_config = PopulationConfig {
        birth_inclusion_start_year: config.birth_inclusion_start_year,
        birth_inclusion_end_year: config.birth_inclusion_end_year,
    };
    
    // Read BEF data
    info!("Reading BEF data from: {:?}", config.bef_path);
    let bef_data_vec = read_parquet(&config.bef_path, None, None)?;
    if bef_data_vec.is_empty() {
        return Err(crate::error::IdsError::Data("No BEF data found".to_string()));
    }
    let bef_data = &bef_data_vec[0]; // Take the first batch for now
    info!("Loaded {} rows from BEF data", bef_data.num_rows());
    
    // Read MFR data
    info!("Reading MFR data from: {:?}", config.mfr_path);
    let mfr_data_vec = read_parquet(&config.mfr_path, None, None)?;
    if mfr_data_vec.is_empty() {
        return Err(crate::error::IdsError::Data("No MFR data found".to_string()));
    }
    let mfr_data = &mfr_data_vec[0]; // Take the first batch for now
    info!("Loaded {} rows from MFR data", mfr_data.num_rows());
    
    // Generate population data
    info!("Generating population data");
    let (family_data, summary) = generate_population(bef_data, mfr_data, &algo_config)?;
    info!("Generated population data with {} records", family_data.num_rows());
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(&config.output_dir)?;
    
    // Save population data
    let population_file = config.output_dir.join("population.parquet");
    info!("Saving population data to: {population_file:?}");
    
    // Use a File object and ParquetWriter to write the file
    let file = std::fs::File::create(&population_file)?;
    let schema = family_data.schema();
    let writer = parquet::arrow::ArrowWriter::try_new(file, schema.clone(), None)
        .map_err(|e| crate::error::IdsError::Data(format!("Failed to create parquet writer: {e}")))?;
    
    let mut writer = writer;
    writer.write(&family_data)
        .map_err(|e| crate::error::IdsError::Data(format!("Failed to write parquet data: {e}")))?;
    
    writer.close()
        .map_err(|e| crate::error::IdsError::Data(format!("Failed to close parquet writer: {e}")))?;
    
    // Save summary reports
    let reports_dir = config.output_dir.join("reports");
    info!("Saving summary reports to: {reports_dir:?}");
    save_population_summary(
        &family_data,
        &reports_dir,
        &summary,
        &summary,
    )?;
    
    info!("Population generation completed successfully");
    Ok(())
}