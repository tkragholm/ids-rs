//! Handler for the SCD command
//!
//! This module provides the implementation for handling the SCD command.

use datafusion::common::config::TableParquetOptions;
use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::prelude::*;
use std::fs;
use tokio::runtime::Runtime;

use crate::algorithm::lpr::{process_lpr_data, LprConfig};
use crate::algorithm::scd::{apply_scd_algorithm, scd_results_to_record_batch, ScdConfig};
use crate::data::registry::traits::RegisterLoader;
use crate::error::{IdsError, Result};
use crate::utils::reports::write_csv_report;

// Import the new DataFusion-based registry loaders
use crate::data::registry::factory::RegistryFactory;
use crate::data::registry::loaders::lpr::find_lpr_files;

use super::config::ScdCommandConfig;

/// Handle the SCD command
pub fn handle_scd_command(config: &ScdCommandConfig) -> Result<()> {
    // Create output directory if it doesn't exist
    if !config.output_path.exists() {
        fs::create_dir_all(&config.output_path).map_err(IdsError::Io)?;
    }

    // Create a tokio runtime for async operations
    let runtime = Runtime::new()
        .map_err(|e| IdsError::Data(format!("Failed to create async runtime: {e}")))?;

    // Step 1: Find LPR files
    log::info!(
        "Searching for LPR files in: {}",
        config.lpr_data_path.display()
    );
    let lpr_paths = find_lpr_files(config.lpr_data_path.to_str().unwrap())?;

    log::info!("Found LPR files:");
    if let Some(path) = &lpr_paths.admin_path {
        log::info!("  LPR_ADM: {}", path.display());
    }
    if let Some(path) = &lpr_paths.diag_path {
        log::info!("  LPR_DIAG: {}", path.display());
    }
    if let Some(path) = &lpr_paths.proc_path {
        log::info!("  LPR_BES: {}", path.display());
    }
    if let Some(path) = &lpr_paths.kontakter_path {
        log::info!("  LPR3_KONTAKTER: {}", path.display());
    }
    if let Some(path) = &lpr_paths.diagnoser_path {
        log::info!("  LPR3_DIAGNOSER: {}", path.display());
    }

    // Step 2: Load LPR data using the new DataFusion-based registry loaders
    log::info!("Loading LPR data...");

    // LPR2 data
    let mut lpr2_adm = None;
    let mut lpr2_diag = None;
    let mut lpr2_bes = None;

    if config.include_lpr2 {
        if let Some(path) = &lpr_paths.admin_path {
            log::info!("Loading LPR_ADM data...");
            let adm_data = runtime.block_on(async {
                let adm_loader = RegistryFactory::from_name("lpr_adm")?;
                
                // Downcast to the actual type
                let loader = adm_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::lpr2_loader::Lpr2Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast LPR ADM register".to_string()))?;
                    
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let adm_batch_count = adm_data.len();
            lpr2_adm = Some(adm_data);
            log::info!("Loaded {adm_batch_count} LPR_ADM batches");
        }

        if let Some(path) = &lpr_paths.diag_path {
            log::info!("Loading LPR_DIAG data...");
            let diag_data = runtime.block_on(async {
                let diag_loader = RegistryFactory::from_name("lpr_diag")?;
                
                // Downcast to the actual type
                let loader = diag_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::lpr2_loader::Lpr2Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast LPR DIAG register".to_string()))?;
                    
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let diag_batch_count = diag_data.len();
            lpr2_diag = Some(diag_data);
            log::info!("Loaded {diag_batch_count} LPR_DIAG batches");
        }

        if let Some(path) = &lpr_paths.proc_path {
            log::info!("Loading LPR_BES data...");
            let bes_data = runtime.block_on(async {
                let bes_loader = RegistryFactory::from_name("lpr_bes")?;
                
                // Downcast to the actual type
                let loader = bes_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::lpr2_loader::Lpr2Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast LPR BES register".to_string()))?;
                    
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let bes_batch_count = bes_data.len();
            lpr2_bes = Some(bes_data);
            log::info!("Loaded {bes_batch_count} LPR_BES batches");
        }
    }

    // LPR3 data
    let mut lpr3_kontakter = None;
    let mut lpr3_diagnoser = None;

    if config.include_lpr3 {
        if let Some(path) = &lpr_paths.kontakter_path {
            log::info!("Loading LPR3_KONTAKTER data...");
            let kontakter_data = runtime.block_on(async {
                let kontakter_loader = RegistryFactory::from_name("lpr3_kontakter")?;
                
                // Downcast to the actual type
                let loader = kontakter_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::lpr3_loader::Lpr3Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast LPR3 KONTAKTER register".to_string()))?;
                    
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let kontakter_batch_count = kontakter_data.len();
            lpr3_kontakter = Some(kontakter_data);
            log::info!("Loaded {kontakter_batch_count} LPR3_KONTAKTER batches");
        }

        if let Some(path) = &lpr_paths.diagnoser_path {
            log::info!("Loading LPR3_DIAGNOSER data...");
            let diagnoser_data = runtime.block_on(async {
                let diagnoser_loader = RegistryFactory::from_name("lpr3_diagnoser")?;
                
                // Downcast to the actual type
                let loader = diagnoser_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::lpr3_loader::Lpr3Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast LPR3 DIAGNOSER register".to_string()))?;
                    
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let diagnoser_batch_count = diagnoser_data.len();
            lpr3_diagnoser = Some(diagnoser_data);
            log::info!("Loaded {diagnoser_batch_count} LPR3_DIAGNOSER batches");
        }
    }

    // Step 3: Process LPR data
    log::info!("Processing LPR data...");
    let lpr_config = LprConfig {
        include_lpr2: config.include_lpr2,
        include_lpr3: config.include_lpr3,
        start_date: config.start_date,
        end_date: config.end_date,
    };

    let processed_data = process_lpr_data(
        lpr2_adm.as_deref(),
        lpr2_diag.as_deref(),
        lpr2_bes.as_deref(),
        lpr3_kontakter.as_deref(),
        lpr3_diagnoser.as_deref(),
        None, // lpr3_procedurer
        &lpr_config,
    )?;

    log::info!("Processed data: {} rows", processed_data.num_rows());

    // Save the processed data using our utility function
    let processed_path = config.output_path.join("processed_lpr_data.parquet");
    runtime.block_on(crate::data::io::parquet::save_batch_to_parquet(
        &processed_data,
        &processed_path,
    ))?;
    log::info!("Saved processed data to: {}", processed_path.display());

    // Step 4: Apply SCD algorithm
    log::info!("Applying SCD algorithm...");
    let scd_config = ScdConfig {
        diagnosis_columns: config.diagnosis_columns.clone(),
        date_column: config.date_column.clone(),
        patient_id_column: config.patient_id_column.clone(),
    };

    let scd_results = apply_scd_algorithm(&processed_data, &scd_config)?;
    log::info!(
        "SCD analysis complete: {} patient records",
        scd_results.len()
    );

    // Calculate summary statistics
    let total_patients = scd_results.len();
    let scd_patients = scd_results.iter().filter(|r| r.is_scd).count();
    let scd_percentage = if total_patients > 0 {
        (scd_patients as f64 / total_patients as f64) * 100.0
    } else {
        0.0
    };

    log::info!("SCD Summary:");
    log::info!("  Total patients: {total_patients}");
    log::info!("  Patients with SCD: {scd_patients} ({scd_percentage:.2}%)");

    // Count by disease category
    let mut category_counts = std::collections::HashMap::new();
    for result in &scd_results {
        for (category, has_disease) in &result.disease_categories {
            if *has_disease {
                *category_counts.entry(category.clone()).or_insert(0) += 1;
            }
        }
    }

    log::info!("Disease categories:");
    for (category, count) in &category_counts {
        let percentage = if total_patients > 0 {
            (f64::from(*count) / total_patients as f64) * 100.0
        } else {
            0.0
        };
        log::info!("  {category}: {count} ({percentage:.2}%)");
    }

    // Step 5: Save results
    log::info!("Converting SCD results to RecordBatch...");
    let scd_batch = scd_results_to_record_batch(&scd_results)?;

    // Save as Parquet using DataFusion
    let scd_parquet_path = config.output_path.join("scd_results.parquet");
    log::info!("Saving SCD results to: {}", scd_parquet_path.display());

    // Use DataFusion to write the Parquet file
    runtime.block_on(async {
        // Create a session context
        let ctx = SessionContext::new();

        // Create a memory table from the record batch
        let table_name = "scd_results";
        ctx.register_batch(table_name, scd_batch.clone())?;

        // Create a DataFrame and write to Parquet
        let df = ctx.table(table_name).await?;

        // Write to Parquet using DataFusion (with optimal settings)
        df.write_parquet(
            scd_parquet_path.to_str().unwrap(),
            DataFrameWriteOptions::default(),
            Some(TableParquetOptions::new()),
        )
        .await
    })?;

    // Save summary as CSV
    let summary_path = config.output_path.join("scd_summary.csv");
    let mut summary_rows = vec![
        vec!["Metric".to_string(), "Value".to_string()],
        vec!["Total Patients".to_string(), total_patients.to_string()],
        vec!["SCD Patients".to_string(), scd_patients.to_string()],
        vec![
            "SCD Percentage".to_string(),
            format!("{:.2}%", scd_percentage),
        ],
    ];

    // Add category breakdowns
    for (category, count) in &category_counts {
        let percentage = if total_patients > 0 {
            (f64::from(*count) / total_patients as f64) * 100.0
        } else {
            0.0
        };
        summary_rows.push(vec![
            format!("Category: {}", category),
            format!("{} ({:.2}%)", count, percentage),
        ]);
    }

    write_csv_report(&summary_path, &summary_rows)?;
    log::info!("Saved SCD summary to: {}", summary_path.display());

    log::info!("SCD command completed successfully");
    Ok(())
}
