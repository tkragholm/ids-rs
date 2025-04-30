//! Handler for the Population SCD command
//!
//! This module provides the implementation for handling the Population SCD command.

use crate::schema::parquet_utils;
use parquet::arrow::ArrowWriter;
use arrow::record_batch::RecordBatch;
use log::info;
use std::fs;
use std::path::Path;

use crate::algorithm::population::classification::{
    extract_scd_children, process_lpr_and_identify_scd, PopulationScdConfig,
};
use crate::error::{IdsError, Result};
use crate::registry::lpr::find_lpr_files;
use crate::registry::lpr::{
    Lpr3DiagnoserRegister, Lpr3KontakterRegister, LprAdmRegister, LprBesRegister, LprDiagRegister,
};
use crate::registry::RegisterLoader;
use crate::utils::reports::write_csv_report;

use super::config::PopulationScdCommandConfig;

/// Handle the Population SCD command
pub fn handle_population_scd_command(config: &PopulationScdCommandConfig) -> Result<()> {
    // Create output directory if it doesn't exist
    if !config.output_dir.exists() {
        fs::create_dir_all(&config.output_dir).map_err(IdsError::Io)?;
    }

    // Step 1: Load population data
    info!(
        "Loading population data from: {}",
        config.population_path.display()
    );
    let population_data = load_parquet_file(&config.population_path)?;
    info!("Loaded {} population records", population_data.num_rows());

    // Step 2: Find LPR files
    info!(
        "Searching for LPR files in: {}",
        config.lpr_data_path.display()
    );
    let lpr_paths = find_lpr_files(config.lpr_data_path.to_str().unwrap())?;

    info!("Found LPR files:");
    if let Some(path) = &lpr_paths.lpr_adm {
        info!("  LPR_ADM: {}", path.display());
    }
    if let Some(path) = &lpr_paths.lpr_diag {
        info!("  LPR_DIAG: {}", path.display());
    }
    if let Some(path) = &lpr_paths.lpr_bes {
        info!("  LPR_BES: {}", path.display());
    }
    if let Some(path) = &lpr_paths.lpr3_kontakter {
        info!("  LPR3_KONTAKTER: {}", path.display());
    }
    if let Some(path) = &lpr_paths.lpr3_diagnoser {
        info!("  LPR3_DIAGNOSER: {}", path.display());
    }

    // Step 3: Load LPR data
    info!("Loading LPR data...");

    // LPR2 data
    let mut lpr2_adm = None;
    let mut lpr2_diag = None;
    let mut lpr2_bes = None;

    if config.include_lpr2 {
        if let Some(path) = &lpr_paths.lpr_adm {
            info!("Loading LPR_ADM data...");
            let adm_loader = LprAdmRegister;
            let adm_data = adm_loader.load(path.to_str().unwrap(), None)?;
            let adm_batch_count = adm_data.len();
            lpr2_adm = Some(adm_data);
            info!("Loaded {adm_batch_count} LPR_ADM batches");
        }

        if let Some(path) = &lpr_paths.lpr_diag {
            info!("Loading LPR_DIAG data...");
            let diag_loader = LprDiagRegister;
            let diag_data = diag_loader.load(path.to_str().unwrap(), None)?;
            let diag_batch_count = diag_data.len();
            lpr2_diag = Some(diag_data);
            info!("Loaded {diag_batch_count} LPR_DIAG batches");
        }

        if let Some(path) = &lpr_paths.lpr_bes {
            info!("Loading LPR_BES data...");
            let bes_loader = LprBesRegister;
            let bes_data = bes_loader.load(path.to_str().unwrap(), None)?;
            let bes_batch_count = bes_data.len();
            lpr2_bes = Some(bes_data);
            info!("Loaded {bes_batch_count} LPR_BES batches");
        }
    }

    // LPR3 data
    let mut lpr3_kontakter = None;
    let mut lpr3_diagnoser = None;

    if config.include_lpr3 {
        if let Some(path) = &lpr_paths.lpr3_kontakter {
            info!("Loading LPR3_KONTAKTER data...");
            let kontakter_loader = Lpr3KontakterRegister;
            let kontakter_data = kontakter_loader.load(path.to_str().unwrap(), None)?;
            let kontakter_batch_count = kontakter_data.len();
            lpr3_kontakter = Some(kontakter_data);
            info!("Loaded {kontakter_batch_count} LPR3_KONTAKTER batches");
        }

        if let Some(path) = &lpr_paths.lpr3_diagnoser {
            info!("Loading LPR3_DIAGNOSER data...");
            let diagnoser_loader = Lpr3DiagnoserRegister;
            let diagnoser_data = diagnoser_loader.load(path.to_str().unwrap(), None)?;
            let diagnoser_batch_count = diagnoser_data.len();
            lpr3_diagnoser = Some(diagnoser_data);
            info!("Loaded {diagnoser_batch_count} LPR3_DIAGNOSER batches");
        }
    }

    // Step 4: Process LPR data and identify SCD in population
    info!("Processing LPR data and identifying SCD in population...");
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

    let (population_scd_data, scd_summary) = process_lpr_and_identify_scd(
        &population_data,
        lpr2_adm.as_deref(),
        lpr2_diag.as_deref(),
        lpr2_bes.as_deref(),
        lpr3_kontakter.as_deref(),
        lpr3_diagnoser.as_deref(),
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
    save_batch_as_parquet(&population_scd_data, &population_scd_path)?;
    info!(
        "Saved population with SCD indicators to: {}",
        population_scd_path.display()
    );

    // Save only SCD children
    let scd_children_path = config.output_dir.join("scd_children.parquet");
    save_batch_as_parquet(&scd_children_data, &scd_children_path)?;
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

/// Load a Parquet file as a `RecordBatch`
fn load_parquet_file(path: &Path) -> Result<RecordBatch> {
    // Use the existing read_parquet function
    let batches = parquet_utils::read_parquet(path, None, None)?;

    if batches.is_empty() {
        return Err(IdsError::Data("No data found in Parquet file".to_string()));
    }

    // Combine all batches into a single RecordBatch
    if batches.len() == 1 {
        Ok(batches[0].clone())
    } else {
        let schema = batches[0].schema();
        arrow::compute::concat_batches(&schema, &batches)
            .map_err(|e| IdsError::Data(format!("Failed to concatenate batches: {e}")))
    }
}

/// Save `RecordBatch` as a Parquet file
fn save_batch_as_parquet(batch: &RecordBatch, path: &Path) -> Result<()> {
    // Use existing helpers if possible, otherwise create a simple implementation
    let file = std::fs::File::create(path).map_err(IdsError::Io)?;

    let writer_props = ArrowWriter::try_new(file, batch.schema(), None)
        .map_err(|e| IdsError::Data(format!("Failed to create writer: {e}")))?;

    let mut writer = writer_props;
    writer
        .write(batch)
        .map_err(|e| IdsError::Data(format!("Failed to write batch: {e}")))?;

    writer
        .close()
        .map_err(|e| IdsError::Data(format!("Failed to close writer: {e}")))?;

    Ok(())
}
