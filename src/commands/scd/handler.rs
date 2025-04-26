//! Handler for the SCD command
//!
//! This module provides the implementation for handling the SCD command.

use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs;
use std::path::Path;

use crate::algorithm::lpr::{process_lpr_data, LprConfig};
use crate::algorithm::scd::{apply_scd_algorithm, scd_results_to_record_batch, ScdConfig};
use crate::error::{IdsError, Result};
use crate::registry::lpr::find_lpr_files;
use crate::registry::lpr::{
    Lpr3DiagnoserRegister, Lpr3KontakterRegister, LprAdmRegister, LprBesRegister, LprDiagRegister,
};
use crate::registry::RegisterLoader;
use crate::utils::reports::write_csv_report;

use super::config::ScdCommandConfig;

/// Handle the SCD command
pub fn handle_scd_command(config: &ScdCommandConfig) -> Result<()> {
    // Create output directory if it doesn't exist
    if !config.output_path.exists() {
        fs::create_dir_all(&config.output_path).map_err(|e| IdsError::Io(e))?;
    }

    // Step 1: Find LPR files
    log::info!(
        "Searching for LPR files in: {}",
        config.lpr_data_path.display()
    );
    let lpr_paths = find_lpr_files(config.lpr_data_path.to_str().unwrap())?;

    log::info!("Found LPR files:");
    if let Some(path) = &lpr_paths.lpr_adm {
        log::info!("  LPR_ADM: {}", path.display());
    }
    if let Some(path) = &lpr_paths.lpr_diag {
        log::info!("  LPR_DIAG: {}", path.display());
    }
    if let Some(path) = &lpr_paths.lpr_bes {
        log::info!("  LPR_BES: {}", path.display());
    }
    if let Some(path) = &lpr_paths.lpr3_kontakter {
        log::info!("  LPR3_KONTAKTER: {}", path.display());
    }
    if let Some(path) = &lpr_paths.lpr3_diagnoser {
        log::info!("  LPR3_DIAGNOSER: {}", path.display());
    }

    // Step 2: Load LPR data
    log::info!("Loading LPR data...");

    // LPR2 data
    let mut lpr2_adm = None;
    let mut lpr2_diag = None;
    let mut lpr2_bes = None;

    if config.include_lpr2 {
        if let Some(path) = &lpr_paths.lpr_adm {
            log::info!("Loading LPR_ADM data...");
            let adm_loader = LprAdmRegister;
            let adm_data = adm_loader.load(path.to_str().unwrap(), None)?;
            let adm_batch_count = adm_data.len();
            lpr2_adm = Some(adm_data);
            log::info!("Loaded {} LPR_ADM batches", adm_batch_count);
        }

        if let Some(path) = &lpr_paths.lpr_diag {
            log::info!("Loading LPR_DIAG data...");
            let diag_loader = LprDiagRegister;
            let diag_data = diag_loader.load(path.to_str().unwrap(), None)?;
            let diag_batch_count = diag_data.len();
            lpr2_diag = Some(diag_data);
            log::info!("Loaded {} LPR_DIAG batches", diag_batch_count);
        }

        if let Some(path) = &lpr_paths.lpr_bes {
            log::info!("Loading LPR_BES data...");
            let bes_loader = LprBesRegister;
            let bes_data = bes_loader.load(path.to_str().unwrap(), None)?;
            let bes_batch_count = bes_data.len();
            lpr2_bes = Some(bes_data);
            log::info!("Loaded {} LPR_BES batches", bes_batch_count);
        }
    }

    // LPR3 data
    let mut lpr3_kontakter = None;
    let mut lpr3_diagnoser = None;

    if config.include_lpr3 {
        if let Some(path) = &lpr_paths.lpr3_kontakter {
            log::info!("Loading LPR3_KONTAKTER data...");
            let kontakter_loader = Lpr3KontakterRegister;
            let kontakter_data = kontakter_loader.load(path.to_str().unwrap(), None)?;
            let kontakter_batch_count = kontakter_data.len();
            lpr3_kontakter = Some(kontakter_data);
            log::info!("Loaded {} LPR3_KONTAKTER batches", kontakter_batch_count);
        }

        if let Some(path) = &lpr_paths.lpr3_diagnoser {
            log::info!("Loading LPR3_DIAGNOSER data...");
            let diagnoser_loader = Lpr3DiagnoserRegister;
            let diagnoser_data = diagnoser_loader.load(path.to_str().unwrap(), None)?;
            let diagnoser_batch_count = diagnoser_data.len();
            lpr3_diagnoser = Some(diagnoser_data);
            log::info!("Loaded {} LPR3_DIAGNOSER batches", diagnoser_batch_count);
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
        &lpr_config,
    )?;

    log::info!("Processed data: {} rows", processed_data.num_rows());

    // Save the processed data
    let processed_path = config.output_path.join("processed_lpr_data.parquet");
    save_batch_as_parquet(&processed_data, &processed_path)?;
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
    log::info!("  Total patients: {}", total_patients);
    log::info!(
        "  Patients with SCD: {} ({:.2}%)",
        scd_patients,
        scd_percentage
    );

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
            (*count as f64 / total_patients as f64) * 100.0
        } else {
            0.0
        };
        log::info!("  {}: {} ({:.2}%)", category, count, percentage);
    }

    // Step 5: Save results
    log::info!("Converting SCD results to RecordBatch...");
    let scd_batch = scd_results_to_record_batch(&scd_results)?;

    // Save as Parquet
    let scd_parquet_path = config.output_path.join("scd_results.parquet");
    save_batch_as_parquet(&scd_batch, &scd_parquet_path)?;
    log::info!("Saved SCD results to: {}", scd_parquet_path.display());

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
            (*count as f64 / total_patients as f64) * 100.0
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

/// Save RecordBatch as a Parquet file
fn save_batch_as_parquet(batch: &RecordBatch, path: &Path) -> Result<()> {
    let file = fs::File::create(path).map_err(|e| IdsError::Io(e))?;

    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))
        .map_err(|e| IdsError::Data(e.to_string()))?;

    writer
        .write(batch)
        .map_err(|e| IdsError::Data(e.to_string()))?;

    writer.close().map_err(|e| IdsError::Data(e.to_string()))?;

    Ok(())
}
