//! Handler for the population generation command

use log::info;
use std::fs;

use crate::algorithm::population::core::{generate_population, PopulationConfig};
use crate::commands::population::config::PopulationCommandConfig;
use crate::utils::date_utils;
use crate::error::{IdsError, Result};
use crate::registry::{BefRegister, MfrRegister, RegisterLoader};
use crate::utils::reports::save_population_summary;
use arrow::array::{Array, StringArray};
use arrow::record_batch::RecordBatch;
use chrono::Datelike;
use std::collections::HashMap;

/// Handle the population generation command
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

    // Read BEF data using BEF registry
    info!("Reading BEF data from: {:?}", config.bef_path);
    let bef_registry = BefRegister;
    let bef_data_vec = bef_registry.load(config.bef_path.to_str().unwrap_or(""), None)?;
    if bef_data_vec.is_empty() {
        return Err(crate::error::IdsError::Data(
            "No BEF data found".to_string(),
        ));
    }

    // Combine all batches into a single RecordBatch
    info!("Found {} BEF batches", bef_data_vec.len());
    let bef_data = combine_record_batches(&bef_data_vec)?;
    info!("Loaded {} rows from BEF data", bef_data.num_rows());

    // Read MFR data using MFR registry
    info!("Reading MFR data from: {:?}", config.mfr_path);
    let mfr_registry = MfrRegister;
    let mfr_data_vec = mfr_registry.load(config.mfr_path.to_str().unwrap_or(""), None)?;
    if mfr_data_vec.is_empty() {
        return Err(crate::error::IdsError::Data(
            "No MFR data found".to_string(),
        ));
    }

    // Combine all batches into a single RecordBatch
    info!("Found {} MFR batches", mfr_data_vec.len());
    let mfr_data = combine_record_batches(&mfr_data_vec)?;
    info!("Loaded {} rows from MFR data", mfr_data.num_rows());

    // Generate population data
    info!("Generating population data");

    // Add debug info about birth years
    debug_birth_years(&bef_data, "BEF")?;
    debug_birth_years(&mfr_data, "MFR")?;

    let (family_data, summary) = generate_population(&bef_data, &mfr_data, &algo_config)?;

    // Print detailed summary
    info!("Population Summary:");
    info!(" - Total BEF records: {}", summary.total_bef_records);
    info!(" - Total MFR records: {}", summary.total_mfr_records);
    info!(" - BEF missing father: {}", summary.bef_missing_father);
    info!(" - BEF missing mother: {}", summary.bef_missing_mother);
    info!(" - MFR missing father: {}", summary.mfr_missing_father);
    info!(" - MFR missing mother: {}", summary.mfr_missing_mother);
    info!(" - Records only in BEF: {}", summary.records_only_in_bef);
    info!(" - Records only in MFR: {}", summary.records_only_in_mfr);
    info!(
        " - Total combined records: {}",
        summary.total_combined_records
    );
    info!(
        " - Combined missing father: {}",
        summary.combined_missing_father
    );
    info!(
        " - Combined missing mother: {}",
        summary.combined_missing_mother
    );

    info!(
        "Generated population data with {} records",
        family_data.num_rows()
    );

    // Create output directory if it doesn't exist
    fs::create_dir_all(&config.output_dir)?;

    // Save population data
    let population_file = config.output_dir.join("population.parquet");
    info!("Saving population data to: {population_file:?}");

    // Use a File object and ParquetWriter to write the file
    let file = std::fs::File::create(&population_file)?;
    let schema = family_data.schema();
    let writer = parquet::arrow::ArrowWriter::try_new(file, schema, None).map_err(|e| {
        crate::error::IdsError::Data(format!("Failed to create parquet writer: {e}"))
    })?;

    let mut writer = writer;
    writer
        .write(&family_data)
        .map_err(|e| crate::error::IdsError::Data(format!("Failed to write parquet data: {e}")))?;

    writer.close().map_err(|e| {
        crate::error::IdsError::Data(format!("Failed to close parquet writer: {e}"))
    })?;

    // Save summary reports
    let reports_dir = config.output_dir.join("reports");
    info!("Saving summary reports to: {reports_dir:?}");
    save_population_summary(&family_data, &reports_dir, &summary, &summary)?;

    info!("Population generation completed successfully");
    Ok(())
}

/// Combines multiple record batches into a single batch
///
/// This function takes a vector of Arrow `RecordBatches` with the same schema
/// and combines them into a single `RecordBatch` containing all rows.
fn combine_record_batches(batches: &[RecordBatch]) -> Result<RecordBatch> {
    // If empty, return an error
    if batches.is_empty() {
        return Err(IdsError::Data("No record batches to combine".to_string()));
    }

    // If only one batch, return a clone of it
    if batches.len() == 1 {
        return Ok(batches[0].clone());
    }

    // Get schema from first batch
    let schema = batches[0].schema();

    // For each column, concatenate all arrays
    let mut combined_columns = Vec::with_capacity(schema.fields().len());

    for i in 0..schema.fields().len() {
        // Extract arrays in proper format for arrow::compute::concat
        let mut arrays = Vec::with_capacity(batches.len());
        for batch in batches {
            let col = batch.column(i);
            arrays.push(col.as_ref());
        }

        // Use arrow's concat to combine the arrays
        let concatenated = arrow::compute::kernels::concat::concat(&arrays)
            .map_err(|e| IdsError::Data(format!("Failed to concatenate arrays: {e}")))?;

        combined_columns.push(concatenated);
    }

    // Create a new RecordBatch with the combined columns
    RecordBatch::try_new(schema, combined_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create combined record batch: {e}")))
}

/// Debug function to analyze birth years in a dataset
fn debug_birth_years(batch: &RecordBatch, dataset_name: &str) -> Result<()> {
    // Determine the date column name based on dataset
    let date_column = if dataset_name == "MFR" {
        "FOEDSELSDATO"
    } else {
        "FOED_DAG"
    };

    // Get the date column
    let date_col = batch.column_by_name(date_column).ok_or_else(|| {
        IdsError::Data(format!(
            "Missing {date_column} column in {dataset_name}"
        ))
    })?;

    // Count occurrences by year
    let mut year_counts: HashMap<i32, usize> = HashMap::new();
    let mut null_count = 0;
    let mut invalid_count = 0;

    for i in 0..date_col.len() {
        if date_col.is_null(i) {
            null_count += 1;
            continue;
        }

        if let Some(date) = date_utils::extract_date_from_array(date_col.as_ref(), i) {
            let year = date.year();
            *year_counts.entry(year).or_insert(0) += 1;
        } else {
            invalid_count += 1;
        }
    }

    // Sort years
    let mut years: Vec<_> = year_counts.keys().collect();
    years.sort();

    // Log summary
    info!("{dataset_name} birth year distribution:");
    for year in years {
        let count = year_counts[year];
        info!(" - {year}: {count} records");
    }
    info!("{dataset_name} null date values: {null_count}");
    info!("{dataset_name} invalid date values: {invalid_count}");

    // Check if PNR column exists
    if let Some(pnr_col) = batch.column_by_name("PNR").or_else(|| {
        // For MFR, try CPR_BARN instead
        if dataset_name == "MFR" {
            batch.column_by_name("CPR_BARN")
        } else {
            None
        }
    }) {
        // Count non-null PNRs
        if let Some(pnr_array) = pnr_col.as_any().downcast_ref::<StringArray>() {
            let valid_pnr_count = (0..pnr_array.len())
                .filter(|&i| !pnr_array.is_null(i) && !pnr_array.value(i).is_empty())
                .count();
            info!(
                "{} valid PNR count: {}/{}",
                dataset_name,
                valid_pnr_count,
                pnr_array.len()
            );
        }
    }

    Ok(())
}
