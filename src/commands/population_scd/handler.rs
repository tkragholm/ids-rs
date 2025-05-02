//! Handler for the Population SCD command
//!
//! This module provides the implementation for handling the Population SCD command.

use log::info;
use std::fs;

use crate::algorithm::population::classification::{
    extract_scd_children, process_lpr_and_identify_scd, PopulationScdConfig,
};
use crate::data::registry::loaders::lpr::find_lpr_files;
use crate::data::registry::traits::RegisterLoader;
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

    // Step 3: Load LPR data
    info!("Loading LPR data...");

    // LPR2 data
    let mut lpr2_adm = None;
    let mut lpr2_diag = None;
    let mut lpr2_bes = None;

    if config.include_lpr2 {
        if let Some(path) = &lpr_paths.admin_path {
            info!("Loading LPR_ADM data...");
            // Create a Lpr2Register from RegistryFactory
            let adm_loader = crate::data::registry::factory::RegistryFactory::from_name("lpr2")?;
            // Use the shared tokio runtime
            let adm_data = runtime.block_on(async {
                let loader = adm_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::Lpr2Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast register".to_string()))?;
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let adm_batch_count = adm_data.len();
            lpr2_adm = Some(adm_data);
            info!("Loaded {adm_batch_count} LPR_ADM batches");
        }

        if let Some(path) = &lpr_paths.diag_path {
            info!("Loading LPR_DIAG data...");
            // Create a Lpr2Register from RegistryFactory
            let diag_loader = crate::data::registry::factory::RegistryFactory::from_name("lpr2")?;
            // Use the shared tokio runtime
            let diag_data = runtime.block_on(async {
                let loader = diag_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::Lpr2Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast register".to_string()))?;
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let diag_batch_count = diag_data.len();
            lpr2_diag = Some(diag_data);
            info!("Loaded {diag_batch_count} LPR_DIAG batches");
        }

        if let Some(path) = &lpr_paths.proc_path {
            info!("Loading LPR_BES data...");
            // Create a Lpr2Register from RegistryFactory
            let bes_loader = crate::data::registry::factory::RegistryFactory::from_name("lpr2")?;
            // Use the shared tokio runtime
            let bes_data = runtime.block_on(async {
                let loader = bes_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::Lpr2Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast register".to_string()))?;
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let bes_batch_count = bes_data.len();
            lpr2_bes = Some(bes_data);
            info!("Loaded {bes_batch_count} LPR_BES batches");
        }
    }

    // LPR3 data
    let mut lpr3_kontakter = None;
    let mut lpr3_diagnoser = None;

    if config.include_lpr3 {
        if let Some(path) = &lpr_paths.kontakter_path {
            info!("Loading LPR3_KONTAKTER data...");
            // Create a Lpr3Register from RegistryFactory
            let kontakter_loader =
                crate::data::registry::factory::RegistryFactory::from_name("lpr3")?;
            // Use the shared tokio runtime
            let kontakter_data = runtime.block_on(async {
                let loader = kontakter_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::Lpr3Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast register".to_string()))?;
                loader.load(path.to_str().unwrap(), None).await
            })?;
            let kontakter_batch_count = kontakter_data.len();
            lpr3_kontakter = Some(kontakter_data);
            info!("Loaded {kontakter_batch_count} LPR3_KONTAKTER batches");
        }

        if let Some(path) = &lpr_paths.diagnoser_path {
            info!("Loading LPR3_DIAGNOSER data...");
            // Create a Lpr3Register from RegistryFactory
            let diagnoser_loader =
                crate::data::registry::factory::RegistryFactory::from_name("lpr3")?;
            // Use the shared tokio runtime
            let diagnoser_data = runtime.block_on(async {
                let loader = diagnoser_loader
                    .downcast_ref::<crate::data::registry::loaders::lpr::Lpr3Register>()
                    .ok_or_else(|| IdsError::Data("Failed to downcast register".to_string()))?;
                loader.load(path.to_str().unwrap(), None).await
            })?;
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

