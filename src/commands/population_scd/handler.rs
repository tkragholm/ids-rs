//! Handler for the Population SCD command
//!
//! This module provides the implementation for handling the Population SCD command.

use log::info;
use std::fs;

use crate::algorithm::population::classification::{
    extract_scd_children, identify_scd_in_population, PopulationScdConfig,
};
use crate::data::registry::loaders::lpr::find_lpr_files;
use crate::error::{IdsError, Result};
use crate::utils::reports::write_csv_report;
use crate::utils::runtime::get_runtime;

use super::config::PopulationScdCommandConfig;

/// Handle the Population SCD command
pub fn handle_population_scd_command(config: &PopulationScdCommandConfig) -> Result<()> {
    // Create output directory if it doesn't exist
    if !config.output_dir.exists() {
        fs::create_dir_all(&config.output_dir).map_err(IdsError::Io)?;
    }

    // Get the shared Tokio runtime
    let runtime = get_runtime()?;

    // Step 1: Load population data
    info!(
        "Loading population data from: {}",
        config.population_path.display()
    );
    
    // Use the DataFusion-based parquet loader
    let population_batches = runtime.block_on(async {
        crate::data::io::parquet::load_parquet_directory(&config.population_path, None, None).await
    })?;
    
    if population_batches.is_empty() {
        return Err(IdsError::Data("No population data found".to_string()));
    }
    
    // Combine batches if necessary
    let population_data = if population_batches.len() == 1 {
        population_batches[0].clone()
    } else {
        let schema = population_batches[0].schema();
        arrow::compute::concat_batches(&schema, &population_batches)
            .map_err(|e| IdsError::Data(format!("Failed to concatenate population batches: {e}")))?
    };
    
    info!("Loaded {} population records", population_data.num_rows());

    // Step 2: Find LPR files
    info!(
        "Searching for LPR files in: {}",
        config.lpr_data_path.display()
    );
    let lpr_paths = find_lpr_files(config.lpr_data_path.to_str().unwrap())?;

    info!("Found LPR files:");
    if let Some(path) = &lpr_paths.admin_path {
        info!("  LPR_ADM: {}", path.display());
    }
    if let Some(path) = &lpr_paths.diag_path {
        info!("  LPR_DIAG: {}", path.display());
    }
    if let Some(path) = &lpr_paths.proc_path {
        info!("  LPR_BES: {}", path.display());
    }
    if let Some(path) = &lpr_paths.kontakter_path {
        info!("  LPR3_KONTAKTER: {}", path.display());
    }
    if let Some(path) = &lpr_paths.diagnoser_path {
        info!("  LPR3_DIAGNOSER: {}", path.display());
    }

    // Step 3: Load LPR data using the new optimized approach
    info!("Loading LPR data...");

    // Create LPR config
    let lpr_config = crate::algorithm::health::lpr::LprConfig {
        include_lpr2: config.include_lpr2,
        include_lpr3: config.include_lpr3,
        start_date: config.start_date,
        end_date: config.end_date,
    };
    
    // Load and process LPR data in a single call
    // This avoids loading the same component (like lpr_adm) multiple times
    let lpr_data = runtime.block_on(async {
        crate::algorithm::health::lpr::load_and_process_lpr(
            config.lpr_data_path.to_str().unwrap(),
            &lpr_config,
            None, // No PNR filter for now
        ).await
    })?;
    
    info!("Loaded and processed LPR data with {} records", lpr_data.num_rows());
    
    // Now we can skip the step for calling process_lpr_and_identify_scd with individual components
    // and use the processed LPR data directly
    
    // Step 4: Process LPR data and identify SCD in population
    info!("Identifying SCD in population...");
    let scd_config = PopulationScdConfig {
        include_lpr2: config.include_lpr2,
        include_lpr3: config.include_lpr3,
        start_date: config.start_date,
        end_date: config.end_date,
        diagnosis_columns: vec![
            "primary_diagnosis".to_string(),
            "secondary_diagnosis".to_string(),
        ],
        patient_id_column: "patient_id".to_string(),
        date_column: "admission_date".to_string(),
        population_pnr_column: "PNR".to_string(),
    };
    
    // Use the pre-processed LPR data directly
    let (population_scd_data, scd_summary) = identify_scd_in_population(
        &population_data,
        &lpr_data,
        &scd_config,
    )?;

    // Step 5: Log summary statistics
    info!("Population SCD analysis complete:");
    info!(
        "  Total children in population: {}",
        scd_summary.total_children
    );
    info!(
        "  Children with SCD: {} ({:.2}%)",
        scd_summary.scd_children, scd_summary.scd_percentage
    );

    info!("SCD by disease category:");
    for (category, count) in &scd_summary.category_counts {
        let percentage = if scd_summary.total_children > 0 {
            (*count as f64 / scd_summary.total_children as f64) * 100.0
        } else {
            0.0
        };
        info!("  {category}: {count} ({percentage:.2}%)");
    }

    // Step 6: Extract SCD children
    info!("Extracting children with SCD...");
    let scd_children_data = extract_scd_children(&population_scd_data)?;
    info!(
        "Extracted {} children with SCD",
        scd_children_data.num_rows()
    );

    // Step 7: Save results

    // Save full population with SCD indicators
    let population_scd_path = config.output_dir.join("population_scd.parquet");
    runtime.block_on(async {
        crate::data::io::parquet::save_batch_to_parquet(&population_scd_data, &population_scd_path).await
    })?;
    info!(
        "Saved population with SCD indicators to: {}",
        population_scd_path.display()
    );

    // Save only SCD children
    let scd_children_path = config.output_dir.join("scd_children.parquet");
    runtime.block_on(async {
        crate::data::io::parquet::save_batch_to_parquet(&scd_children_data, &scd_children_path).await
    })?;
    info!("Saved SCD children to: {}", scd_children_path.display());

    // Save summary as CSV
    let summary_path = config.output_dir.join("population_scd_summary.csv");
    let mut summary_rows = vec![
        vec!["Metric".to_string(), "Value".to_string()],
        vec![
            "Total Children".to_string(),
            scd_summary.total_children.to_string(),
        ],
        vec![
            "Children with SCD".to_string(),
            scd_summary.scd_children.to_string(),
        ],
        vec![
            "SCD Percentage".to_string(),
            format!("{:.2}%", scd_summary.scd_percentage),
        ],
    ];

    // Add category breakdowns
    for (category, count) in &scd_summary.category_counts {
        let percentage = if scd_summary.total_children > 0 {
            (*count as f64 / scd_summary.total_children as f64) * 100.0
        } else {
            0.0
        };
        summary_rows.push(vec![
            format!("Category: {}", category),
            format!("{} ({:.2}%)", count, percentage),
        ]);
    }

    write_csv_report(&summary_path, &summary_rows)?;
    info!(
        "Saved population SCD summary to: {}",
        summary_path.display()
    );

    info!("Population SCD command completed successfully");
    Ok(())
}

