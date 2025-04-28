//! Handler for the study design command
//!
//! This module provides the implementation for handling the study design command,
//! which combines population generation, SCD identification, sampling/matching,
//! and covariate balance checking.

use arrow::array::{Array, BooleanArray, StringArray};
use arrow::compute;
use arrow::record_batch::RecordBatch;
use chrono::{Datelike, NaiveDate};
use log::info;
use rand::seq::IteratorRandom;
use std::path::Path;
use tokio::runtime::Runtime;

use crate::algorithm::balance::{calculate_balance, generate_balance_report};
use crate::algorithm::matching::{Matcher, MatchingCriteria};
use crate::commands::population::config::PopulationCommandConfig;
use crate::commands::population::handler::handle_population_command;
use crate::commands::population_scd::config::PopulationScdCommandConfig;
use crate::commands::population_scd::handler::handle_population_scd_command;
use crate::error::{IdsError, Result};
use crate::model::pnr::Pnr;
use crate::utils::date_utils;

use super::config::StudyDesignCommandConfig;

/// Handle the study design command (synchronous version)
pub fn handle_study_design_command(config: &StudyDesignCommandConfig) -> Result<()> {
    // For improved performance with slow storage, consider using the async version
    // You can call it from this synchronous handler using a runtime:
    if config.use_async_io {
        let rt = Runtime::new()
            .map_err(|e| IdsError::Data(format!("Failed to create async runtime: {e}")))?;
        return rt.block_on(handle_study_design_command_async(config));
    }

    // Regular synchronous implementation follows...
    // Step 1: Generate population
    info!("Step 1: Generating Population");
    let population_config = PopulationCommandConfig {
        bef_path: config.bef_path.clone(),
        mfr_path: config.mfr_path.clone(),
        output_dir: config.output_dir.join("01_population"),
        birth_inclusion_start_year: config.birth_inclusion_start_year,
        birth_inclusion_end_year: config.birth_inclusion_end_year,
    };

    // Create population output directory
    std::fs::create_dir_all(&population_config.output_dir)?;

    // Generate population data
    handle_population_command(&population_config)?;

    // The generated population file path
    let population_path = population_config.output_dir.join("population.parquet");

    // Step 2: Identify SCD in population
    info!("Step 2: Identifying SCD in Population");
    let population_scd_config = PopulationScdCommandConfig {
        population_path: population_path.clone(),
        lpr_data_path: config.lpr_data_path.clone(),
        output_dir: config.output_dir.join("02_scd"),
        include_lpr2: config.include_lpr2,
        include_lpr3: config.include_lpr3,
        start_date: config.start_date,
        end_date: config.end_date,
    };

    // Create SCD output directory
    std::fs::create_dir_all(&population_scd_config.output_dir)?;

    // Process SCD
    handle_population_scd_command(&population_scd_config)?;

    // The SCD children file path
    let scd_children_path = population_scd_config
        .output_dir
        .join("scd_children.parquet");

    // Step 3: Sample Controls and Match with Cases
    info!("Step 3: Matching Cases with Controls");

    // Load SCD children (cases)
    let scd_children = load_parquet_file(&scd_children_path)?;

    // Load full population data
    let population_scd_data_path = population_scd_config
        .output_dir
        .join("population_scd.parquet");
    let population_scd_data = load_parquet_file(&population_scd_data_path)?;

    // Extract controls (non-SCD children) from population
    let controls = extract_controls(&population_scd_data)?;

    // Create matching criteria
    let criteria = MatchingCriteria {
        birth_date_window_days: config.birth_date_window_days,
        parent_birth_date_window_days: config.parent_birth_date_window_days,
        require_both_parents: config.require_both_parents,
        require_same_gender: config.require_same_gender,
    };

    // Perform matching
    let matching_output_dir = config.output_dir.join("03_matching");
    std::fs::create_dir_all(&matching_output_dir)?;
    let (case_data, control_data) = perform_matching(
        &scd_children,
        &controls,
        &criteria,
        config.matching_ratio,
        &matching_output_dir,
    )?;

    // Step 4: Check Covariate Balance
    info!("Step 4: Checking Covariate Balance");
    let balance_dir = config.output_dir.join("04_balance");
    std::fs::create_dir_all(&balance_dir)?;

    let balance_report = calculate_balance(&[case_data], &[control_data])?;

    // Generate balance report
    let report_path = balance_dir.join("balance_report.csv");
    generate_balance_report(&report_path.to_string_lossy(), &balance_report)?;

    // Print summary
    info!("Study Design Pipeline Completed Successfully");
    info!("Balance Report Summary:");
    info!(
        " - Total Covariates: {}",
        balance_report.summary.total_covariates
    );
    info!(
        " - Imbalanced Covariates: {}",
        balance_report.summary.imbalanced_covariates
    );
    info!(
        " - Max Standardized Difference: {:.4}",
        balance_report.summary.max_standardized_difference
    );
    info!(
        " - Mean Absolute Standardized Difference: {:.4}",
        balance_report.summary.mean_absolute_standardized_difference
    );

    Ok(())
}

/// Load a Parquet file as a `RecordBatch`
///
/// This is the synchronous version of the function.
fn load_parquet_file(path: &Path) -> Result<RecordBatch> {
    // Use the existing read_parquet function
    let batches = crate::schema::parquet_utils::read_parquet(path, None, None)?;

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

/// Load a Parquet file asynchronously as a `RecordBatch`
///
/// This version uses the optimized async Parquet reader for better performance
/// with slow storage devices.
async fn load_parquet_file_async(path: &Path) -> Result<RecordBatch> {
    // Use the async read_parquet function
    let batches = crate::schema::parquet_async::read_parquet_async(path, None, None).await?;

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

/// Extract controls (non-SCD children) from the population data
fn extract_controls(population_data: &RecordBatch) -> Result<RecordBatch> {
    // Get the is_scd column
    let is_scd_idx = population_data
        .schema()
        .index_of("is_scd")
        .map_err(|e| IdsError::Data(format!("is_scd column not found: {e}")))?;

    let is_scd_col = population_data.column(is_scd_idx);
    let is_scd_array = is_scd_col
        .as_any()
        .downcast_ref::<BooleanArray>()
        .ok_or_else(|| IdsError::Data("is_scd column is not a boolean array".to_string()))?;

    // Create a mask for rows where is_scd is false
    let mask = BooleanArray::from(
        (0..is_scd_array.len())
            .map(|i| {
                if is_scd_array.is_null(i) {
                    None
                } else {
                    Some(!is_scd_array.value(i)) // Note the negation here
                }
            })
            .collect::<Vec<Option<bool>>>(),
    );

    // Apply the mask to all columns
    let mut filtered_columns = Vec::with_capacity(population_data.num_columns());
    for col in population_data.columns() {
        let filtered_col = compute::filter(col, &mask)
            .map_err(|e| IdsError::Data(format!("Failed to filter column: {e}")))?;
        filtered_columns.push(filtered_col);
    }

    // Create filtered batch
    let filtered_batch = RecordBatch::try_new(population_data.schema(), filtered_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))?;

    Ok(filtered_batch)
}

/// Extract PNR and birth date pairs from a `RecordBatch`
fn extract_pnr_and_birth_date(batch: &RecordBatch) -> Result<Vec<(Pnr, NaiveDate)>> {
    let pnr_idx = batch
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found: {e}")))?;

    let birth_date_idx = batch
        .schema()
        .index_of("FOED_DAG")
        .map_err(|e| IdsError::Data(format!("FOED_DAG column not found: {e}")))?;

    let pnr_col = batch.column(pnr_idx);
    let birth_date_col = batch.column(birth_date_idx);

    let pnr_array = pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;

    let mut pairs = Vec::new();

    for i in 0..batch.num_rows() {
        if pnr_array.is_null(i) {
            continue;
        }

        let pnr_str = pnr_array.value(i);
        let pnr = Pnr::from(pnr_str);

        if let Some(date) = date_utils::extract_date_from_array(birth_date_col.as_ref(), i) {
            pairs.push((pnr, date));
        }
    }

    Ok(pairs)
}

/// Find eligible controls for a case
fn find_eligible_controls(
    case_pnr: &Pnr,
    case_birth_date: NaiveDate,
    controls: &[(Pnr, NaiveDate)],
    criteria: &MatchingCriteria,
) -> Result<Vec<usize>> {
    let mut eligible_indices = Vec::new();

    for (idx, (control_pnr, control_birth_date)) in controls.iter().enumerate() {
        // Skip if case and control are the same person
        if case_pnr.value() == control_pnr.value() {
            continue;
        }

        // Check birth date window
        let diff = (*control_birth_date - case_birth_date).num_days().abs() as i32;
        if diff > criteria.birth_date_window_days {
            continue;
        }

        // Additional criteria checks would go here
        // (gender, parents, etc.)

        eligible_indices.push(idx);
    }

    Ok(eligible_indices)
}

/// Find a record by PNR in a `RecordBatch`
fn find_record_by_pnr(batch: &RecordBatch, pnr: &Pnr) -> Result<RecordBatch> {
    let pnr_idx = batch
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found: {e}")))?;

    let pnr_col = batch.column(pnr_idx);
    let pnr_array = pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;

    let mut row_idx = None;
    for i in 0..pnr_array.len() {
        if !pnr_array.is_null(i) && pnr_array.value(i) == pnr.value() {
            row_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = row_idx {
        // Create a mask with just this row selected
        let mut mask_values = vec![false; batch.num_rows()];
        mask_values[idx] = true;

        let mask = BooleanArray::from(mask_values);

        // Apply the mask to all columns
        let mut filtered_columns = Vec::with_capacity(batch.num_columns());
        for col in batch.columns() {
            let filtered_col = compute::filter(col, &mask)
                .map_err(|e| IdsError::Data(format!("Failed to filter column: {e}")))?;
            filtered_columns.push(filtered_col);
        }

        let filtered_batch = RecordBatch::try_new(batch.schema(), filtered_columns)
            .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))?;

        Ok(filtered_batch)
    } else {
        Err(IdsError::Data(format!(
            "PNR {} not found in batch",
            pnr.value()
        )))
    }
}

/// Perform matching between cases and controls
fn perform_matching(
    cases: &RecordBatch,
    controls: &RecordBatch,
    criteria: &MatchingCriteria,
    matching_ratio: usize,
    output_dir: &Path,
) -> Result<(RecordBatch, RecordBatch)> {
    // Pre-extract PNR column indexes for faster access later
    let pnr_idx_cases = cases
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found in cases: {e}")))?;

    let pnr_idx_controls = controls
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found in controls: {e}")))?;

    // Convert cases and controls to the format needed for matching
    let case_pairs = extract_pnr_and_birth_date(cases)?;
    let control_pairs = extract_pnr_and_birth_date(controls)?;

    // Create reference to cases PNR column for quick lookups
    let cases_pnr_col = cases.column(pnr_idx_cases);
    let cases_pnr_array = cases_pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column in cases is not a string array".to_string()))?;

    // Create reference to controls PNR column for quick lookups
    let controls_pnr_col = controls.column(pnr_idx_controls);
    let controls_pnr_array = controls_pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| {
            IdsError::Data("PNR column in controls is not a string array".to_string())
        })?;

    // Build index maps for fast PNR lookups (O(1) instead of O(n))
    let mut case_pnr_to_idx = std::collections::HashMap::with_capacity(cases.num_rows());
    for i in 0..cases_pnr_array.len() {
        if !cases_pnr_array.is_null(i) {
            case_pnr_to_idx.insert(cases_pnr_array.value(i).to_string(), i);
        }
    }

    let mut control_pnr_to_idx = std::collections::HashMap::with_capacity(controls.num_rows());
    for i in 0..controls_pnr_array.len() {
        if !controls_pnr_array.is_null(i) {
            control_pnr_to_idx.insert(controls_pnr_array.value(i).to_string(), i);
        }
    }

    // Create a birth_date index for controls to speed up matching
    // Group controls by birth date window buckets (30-day periods)
    let window_days = criteria.birth_date_window_days;
    let mut birth_date_index: std::collections::HashMap<i32, Vec<usize>> =
        std::collections::HashMap::new();

    for (idx, (_, birth_date)) in control_pairs.iter().enumerate() {
        // Use integer division to create buckets of days
        let bucket = birth_date.num_days_from_ce() / window_days;
        birth_date_index.entry(bucket).or_default().push(idx);

        // Also add to adjacent buckets to handle boundary cases
        birth_date_index.entry(bucket - 1).or_default().push(idx);
        birth_date_index.entry(bucket + 1).or_default().push(idx);
    }

    info!(
        "Matching {} cases with {} controls (ratio 1:{})",
        case_pairs.len(),
        control_pairs.len(),
        matching_ratio
    );

    // Preallocate memory for the result (estimate based on matching ratio)
    let estimated_matches = case_pairs.len() * matching_ratio;
    let mut matched_case_indices = Vec::with_capacity(case_pairs.len());
    let mut matched_control_indices = Vec::with_capacity(estimated_matches);

    // Used to track controls that have been used
    let mut used_control_indices = std::collections::HashSet::with_capacity(estimated_matches);

    // Process cases in chunks for better cache locality and progress reporting
    const CHUNK_SIZE: usize = 1000;
    let total_chunks = case_pairs.len().div_ceil(CHUNK_SIZE);

    for chunk_idx in 0..total_chunks {
        let start_idx = chunk_idx * CHUNK_SIZE;
        let end_idx = std::cmp::min(start_idx + CHUNK_SIZE, case_pairs.len());

        // Process each case in this chunk
        for case_idx in start_idx..end_idx {
            let (case_pnr, case_birth_date) = &case_pairs[case_idx];

            // Use birth date index to find potential controls more efficiently
            let bucket = case_birth_date.num_days_from_ce() / window_days;
            let empty_vec: Vec<usize> = Vec::new();
            let potential_controls = birth_date_index.get(&bucket).unwrap_or(&empty_vec);

            // Find eligible controls from the potential ones
            let mut eligible_control_indices = Vec::new();
            for &ctrl_idx in potential_controls {
                // Skip if already used
                if used_control_indices.contains(&ctrl_idx) {
                    continue;
                }

                let (control_pnr, control_birth_date) = &control_pairs[ctrl_idx];

                // Skip if case and control are the same person
                if case_pnr.value() == control_pnr.value() {
                    continue;
                }

                // Check birth date window
                let diff = (*control_birth_date - *case_birth_date).num_days().abs() as i32;
                if diff > criteria.birth_date_window_days {
                    continue;
                }

                // Additional criteria checks can be added here
                // (gender, parents, etc.)

                eligible_control_indices.push(ctrl_idx);
            }

            if eligible_control_indices.is_empty() {
                // If no quick matches found, do a more thorough search
                // as a fallback (less efficient but ensures we don't miss matches)
                for (ctrl_idx, (control_pnr, control_birth_date)) in
                    control_pairs.iter().enumerate()
                {
                    // Skip if already used
                    if used_control_indices.contains(&ctrl_idx) {
                        continue;
                    }

                    // Skip if case and control are the same person
                    if case_pnr.value() == control_pnr.value() {
                        continue;
                    }

                    // Check birth date window
                    let diff = (*control_birth_date - *case_birth_date).num_days().abs() as i32;
                    if diff > criteria.birth_date_window_days {
                        continue;
                    }

                    eligible_control_indices.push(ctrl_idx);
                }
            }

            if eligible_control_indices.is_empty() {
                log::debug!(
                    "No eligible controls found for case {} ({}/{})",
                    case_pnr.value(),
                    case_idx + 1,
                    case_pairs.len()
                );
                continue;
            }

            // Select up to matching_ratio controls randomly
            let mut rng = rand::rng();
            let num_to_select = std::cmp::min(matching_ratio, eligible_control_indices.len());
            let selected_indices: Vec<_> = eligible_control_indices
                .into_iter()
                .choose_multiple(&mut rng, num_to_select);

            // Add this case to the matched cases
            matched_case_indices.push(*case_pnr_to_idx.get(case_pnr.value()).ok_or_else(|| {
                IdsError::Data(format!("Case PNR {} not found in index", case_pnr.value()))
            })?);

            // Add selected controls and mark them as used
            for ctrl_idx in selected_indices {
                let control_pnr = &control_pairs[ctrl_idx].0;
                matched_control_indices.push(
                    *control_pnr_to_idx.get(control_pnr.value()).ok_or_else(|| {
                        IdsError::Data(format!(
                            "Control PNR {} not found in index",
                            control_pnr.value()
                        ))
                    })?,
                );
                used_control_indices.insert(ctrl_idx);
            }
        }

        // Log progress after each chunk or at the end
        info!(
            "Matched {}/{} cases ({:.1}%)",
            end_idx,
            case_pairs.len(),
            end_idx as f64 * 100.0 / case_pairs.len() as f64
        );
    }

    if matched_case_indices.is_empty() {
        return Err(IdsError::Validation(
            "No matches found for any cases".to_string(),
        ));
    }

    // Create masks for the matched cases and controls
    let mut case_mask = vec![false; cases.num_rows()];
    for idx in &matched_case_indices {
        case_mask[*idx] = true;
    }

    let mut control_mask = vec![false; controls.num_rows()];
    for idx in &matched_control_indices {
        control_mask[*idx] = true;
    }

    // Apply the masks to get the matched batches (much more efficient than individual filters)
    let case_bool_array = BooleanArray::from(case_mask);
    let control_bool_array = BooleanArray::from(control_mask);

    // Apply case mask to all case columns
    let mut filtered_case_columns = Vec::with_capacity(cases.num_columns());
    for col in cases.columns() {
        let filtered_col = compute::filter(col, &case_bool_array)
            .map_err(|e| IdsError::Data(format!("Failed to filter case column: {e}")))?;
        filtered_case_columns.push(filtered_col);
    }

    // Apply control mask to all control columns
    let mut filtered_control_columns = Vec::with_capacity(controls.num_columns());
    for col in controls.columns() {
        let filtered_col = compute::filter(col, &control_bool_array)
            .map_err(|e| IdsError::Data(format!("Failed to filter control column: {e}")))?;
        filtered_control_columns.push(filtered_col);
    }

    // Create final batches
    let matched_cases_batch = RecordBatch::try_new(cases.schema(), filtered_case_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create matched cases batch: {e}")))?;

    let matched_controls_batch = RecordBatch::try_new(controls.schema(), filtered_control_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create matched controls batch: {e}")))?;

    info!(
        "Final matched dataset: {} cases and {} controls",
        matched_cases_batch.num_rows(),
        matched_controls_batch.num_rows()
    );

    // Save matched cases and controls
    save_batch_as_parquet(
        &matched_cases_batch,
        &output_dir.join("matched_cases.parquet"),
    )?;
    save_batch_as_parquet(
        &matched_controls_batch,
        &output_dir.join("matched_controls.parquet"),
    )?;

    // Return the matched data
    Ok((matched_cases_batch, matched_controls_batch))
}

/// Combines multiple record batches into a single batch
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

    // Use arrow's concat_batches function
    arrow::compute::concat_batches(&schema, batches)
        .map_err(|e| IdsError::Data(format!("Failed to concatenate batches: {e}")))
}

/// Save `RecordBatch` as a Parquet file (synchronous version)
fn save_batch_as_parquet(batch: &RecordBatch, path: &Path) -> Result<()> {
    let file = std::fs::File::create(path).map_err(IdsError::Io)?;

    let writer_props = parquet::arrow::ArrowWriter::try_new(file, batch.schema(), None)
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

/// Save `RecordBatch` as a Parquet file asynchronously
///
/// This uses Tokio's async file operations for better performance with slow
/// storage devices.
async fn save_batch_as_parquet_async(batch: &RecordBatch, path: &Path) -> Result<()> {
    // Create the file asynchronously
    let file = tokio::fs::File::create(path).await.map_err(|e| {
        IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create file asynchronously: {e}"),
        ))
    })?;

    // We need to convert the tokio file to a std file since parquet-arrow
    // doesn't have async writer support yet
    let std_file = file.into_std().await;

    // Now proceed with the normal parquet writing
    let writer_props = parquet::arrow::ArrowWriter::try_new(std_file, batch.schema(), None)
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

/// Handle the study design command using async I/O
///
/// This version uses the async Parquet reader/writer for better performance
/// with slow storage devices.
pub async fn handle_study_design_command_async(config: &StudyDesignCommandConfig) -> Result<()> {
    // Step 1: Generate population
    // This step still uses synchronous calls since we haven't updated those handlers yet
    info!("Step 1: Generating Population (sync)");
    let population_config = PopulationCommandConfig {
        bef_path: config.bef_path.clone(),
        mfr_path: config.mfr_path.clone(),
        output_dir: config.output_dir.join("01_population"),
        birth_inclusion_start_year: config.birth_inclusion_start_year,
        birth_inclusion_end_year: config.birth_inclusion_end_year,
    };

    // Create population output directory
    tokio::fs::create_dir_all(&population_config.output_dir)
        .await
        .map_err(IdsError::Io)?;

    // Generate population data (still synchronous)
    handle_population_command(&population_config)?;

    // The generated population file path
    let population_path = population_config.output_dir.join("population.parquet");

    // Step 2: Identify SCD in population (still synchronous)
    info!("Step 2: Identifying SCD in Population (sync)");
    let population_scd_config = PopulationScdCommandConfig {
        population_path: population_path.clone(),
        lpr_data_path: config.lpr_data_path.clone(),
        output_dir: config.output_dir.join("02_scd"),
        include_lpr2: config.include_lpr2,
        include_lpr3: config.include_lpr3,
        start_date: config.start_date,
        end_date: config.end_date,
    };

    // Create SCD output directory
    tokio::fs::create_dir_all(&population_scd_config.output_dir)
        .await
        .map_err(IdsError::Io)?;

    // Process SCD (still synchronous)
    handle_population_scd_command(&population_scd_config)?;

    // The SCD children file path
    let scd_children_path = population_scd_config
        .output_dir
        .join("scd_children.parquet");

    // Step 3: Sample Controls and Match with Cases (async loading)
    info!("Step 3: Matching Cases with Controls (async)");

    // Load SCD children (cases) with async reader
    let scd_children = load_parquet_file_async(&scd_children_path).await?;

    // Load full population data with async reader
    let population_scd_data_path = population_scd_config
        .output_dir
        .join("population_scd.parquet");
    let population_scd_data = load_parquet_file_async(&population_scd_data_path).await?;

    // Extract controls (non-SCD children) from population
    let controls = extract_controls(&population_scd_data)?;

    // Create matching criteria
    let criteria = MatchingCriteria {
        birth_date_window_days: config.birth_date_window_days,
        parent_birth_date_window_days: config.parent_birth_date_window_days,
        require_both_parents: config.require_both_parents,
        require_same_gender: config.require_same_gender,
    };

    // Perform matching
    let matching_output_dir = config.output_dir.join("03_matching");
    tokio::fs::create_dir_all(&matching_output_dir)
        .await
        .map_err(IdsError::Io)?;

    // Matching process is the same but uses async file operations
    // Extract case and control pairs
    let case_pairs = extract_pnr_and_birth_date(&scd_children)?;
    let control_pairs = extract_pnr_and_birth_date(&controls)?;

    // Create matcher with the given criteria
    let _matcher = Matcher::new(criteria.clone());

    // For each case, find multiple controls
    let matched_cases: Vec<usize> = Vec::new();
    let matched_controls: Vec<usize> = Vec::new();

    // Use our optimized matching implementation from the synchronous version
    // Pre-extract PNR column indexes for faster access later
    let pnr_idx_cases = scd_children
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found in cases: {e}")))?;

    let pnr_idx_controls = controls
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found in controls: {e}")))?;

    // Create reference to cases PNR column for quick lookups
    let cases_pnr_col = scd_children.column(pnr_idx_cases);
    let cases_pnr_array = cases_pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column in cases is not a string array".to_string()))?;

    // Create reference to controls PNR column for quick lookups
    let controls_pnr_col = controls.column(pnr_idx_controls);
    let controls_pnr_array = controls_pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| {
            IdsError::Data("PNR column in controls is not a string array".to_string())
        })?;

    // Build index maps for fast PNR lookups (O(1) instead of O(n))
    let mut case_pnr_to_idx = std::collections::HashMap::with_capacity(scd_children.num_rows());
    for i in 0..cases_pnr_array.len() {
        if !cases_pnr_array.is_null(i) {
            case_pnr_to_idx.insert(cases_pnr_array.value(i).to_string(), i);
        }
    }

    let mut control_pnr_to_idx = std::collections::HashMap::with_capacity(controls.num_rows());
    for i in 0..controls_pnr_array.len() {
        if !controls_pnr_array.is_null(i) {
            control_pnr_to_idx.insert(controls_pnr_array.value(i).to_string(), i);
        }
    }

    // Create a birth_date index for controls to speed up matching
    // Group controls by birth date window buckets (30-day periods)
    let window_days = criteria.birth_date_window_days;
    let mut birth_date_index: std::collections::HashMap<i32, Vec<usize>> =
        std::collections::HashMap::new();

    for (idx, (_, birth_date)) in control_pairs.iter().enumerate() {
        // Use integer division to create buckets of days
        let bucket = birth_date.num_days_from_ce() / window_days;
        birth_date_index.entry(bucket).or_default().push(idx);

        // Also add to adjacent buckets to handle boundary cases
        birth_date_index.entry(bucket - 1).or_default().push(idx);
        birth_date_index.entry(bucket + 1).or_default().push(idx);
    }

    // Preallocate memory for the result (estimate based on matching ratio)
    let estimated_matches = case_pairs.len() * config.matching_ratio;
    let mut matched_case_indices = Vec::with_capacity(case_pairs.len());
    let mut matched_control_indices = Vec::with_capacity(estimated_matches);

    // Used to track controls that have been used
    let mut used_control_indices = std::collections::HashSet::with_capacity(estimated_matches);

    // Process cases in chunks for better cache locality and progress reporting
    const CHUNK_SIZE: usize = 1000;
    let total_chunks = case_pairs.len().div_ceil(CHUNK_SIZE);

    for chunk_idx in 0..total_chunks {
        let start_idx = chunk_idx * CHUNK_SIZE;
        let end_idx = std::cmp::min(start_idx + CHUNK_SIZE, case_pairs.len());

        // Process each case in this chunk
        for case_idx in start_idx..end_idx {
            let (case_pnr, case_birth_date) = &case_pairs[case_idx];

            // Use birth date index to find potential controls more efficiently
            let bucket = case_birth_date.num_days_from_ce() / window_days;
            let empty_vec: Vec<usize> = Vec::new();
            let potential_controls = birth_date_index.get(&bucket).unwrap_or(&empty_vec);

            // Find eligible controls from the potential ones
            let mut eligible_control_indices = Vec::new();
            for &ctrl_idx in potential_controls {
                // Skip if already used
                if used_control_indices.contains(&ctrl_idx) {
                    continue;
                }

                let (control_pnr, control_birth_date) = &control_pairs[ctrl_idx];

                // Skip if case and control are the same person
                if case_pnr.value() == control_pnr.value() {
                    continue;
                }

                // Check birth date window
                let diff = (*control_birth_date - *case_birth_date).num_days().abs() as i32;
                if diff > criteria.birth_date_window_days {
                    continue;
                }

                // Additional criteria checks can be added here
                // (gender, parents, etc.)

                eligible_control_indices.push(ctrl_idx);
            }

            if eligible_control_indices.is_empty() {
                // If no quick matches found, do a more thorough search
                // as a fallback (less efficient but ensures we don't miss matches)
                for (ctrl_idx, (control_pnr, control_birth_date)) in
                    control_pairs.iter().enumerate()
                {
                    // Skip if already used
                    if used_control_indices.contains(&ctrl_idx) {
                        continue;
                    }

                    // Skip if case and control are the same person
                    if case_pnr.value() == control_pnr.value() {
                        continue;
                    }

                    // Check birth date window
                    let diff = (*control_birth_date - *case_birth_date).num_days().abs() as i32;
                    if diff > criteria.birth_date_window_days {
                        continue;
                    }

                    eligible_control_indices.push(ctrl_idx);
                }
            }

            if eligible_control_indices.is_empty() {
                log::debug!(
                    "No eligible controls found for case {} ({}/{})",
                    case_pnr.value(),
                    case_idx + 1,
                    case_pairs.len()
                );
                continue;
            }

            // Select up to matching_ratio controls randomly
            let mut rng = rand::rng();
            let num_to_select =
                std::cmp::min(config.matching_ratio, eligible_control_indices.len());
            let selected_indices: Vec<_> = eligible_control_indices
                .into_iter()
                .choose_multiple(&mut rng, num_to_select);

            // Add this case to the matched cases
            matched_case_indices.push(*case_pnr_to_idx.get(case_pnr.value()).ok_or_else(|| {
                IdsError::Data(format!("Case PNR {} not found in index", case_pnr.value()))
            })?);

            // Add selected controls and mark them as used
            for ctrl_idx in selected_indices {
                let control_pnr = &control_pairs[ctrl_idx].0;
                matched_control_indices.push(
                    *control_pnr_to_idx.get(control_pnr.value()).ok_or_else(|| {
                        IdsError::Data(format!(
                            "Control PNR {} not found in index",
                            control_pnr.value()
                        ))
                    })?,
                );
                used_control_indices.insert(ctrl_idx);
            }
        }

        // Log progress after each chunk or at the end
        info!(
            "Matched {}/{} cases ({:.1}%)",
            end_idx,
            case_pairs.len(),
            end_idx as f64 * 100.0 / case_pairs.len() as f64
        );
    }

    if matched_case_indices.is_empty() {
        return Err(IdsError::Validation(
            "No matches found for any cases".to_string(),
        ));
    }

    // Create masks for the matched cases and controls
    let mut case_mask = vec![false; scd_children.num_rows()];
    for idx in &matched_case_indices {
        case_mask[*idx] = true;
    }

    let mut control_mask = vec![false; controls.num_rows()];
    for idx in &matched_control_indices {
        control_mask[*idx] = true;
    }

    // Apply the masks to get the matched batches (much more efficient than individual filters)
    let case_bool_array = BooleanArray::from(case_mask);
    let control_bool_array = BooleanArray::from(control_mask);

    // Apply case mask to all case columns
    let mut filtered_case_columns = Vec::with_capacity(scd_children.num_columns());
    for col in scd_children.columns() {
        let filtered_col = compute::filter(col, &case_bool_array)
            .map_err(|e| IdsError::Data(format!("Failed to filter case column: {e}")))?;
        filtered_case_columns.push(filtered_col);
    }

    // Apply control mask to all control columns
    let mut filtered_control_columns = Vec::with_capacity(controls.num_columns());
    for col in controls.columns() {
        let filtered_col = compute::filter(col, &control_bool_array)
            .map_err(|e| IdsError::Data(format!("Failed to filter control column: {e}")))?;
        filtered_control_columns.push(filtered_col);
    }

    // Create final batches
    let matched_cases_batch = RecordBatch::try_new(scd_children.schema(), filtered_case_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create matched cases batch: {e}")))?;

    let matched_controls_batch = RecordBatch::try_new(controls.schema(), filtered_control_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create matched controls batch: {e}")))?;

    if matched_cases_batch.num_rows() == 0 {
        return Err(IdsError::Validation(
            "No matches found for any cases".to_string(),
        ));
    }

    info!(
        "Final matched dataset: {} cases and {} controls",
        matched_cases_batch.num_rows(),
        matched_controls_batch.num_rows()
    );

    // Save matched cases and controls asynchronously
    save_batch_as_parquet_async(
        &matched_cases_batch,
        &matching_output_dir.join("matched_cases.parquet"),
    )
    .await?;
    save_batch_as_parquet_async(
        &matched_controls_batch,
        &matching_output_dir.join("matched_controls.parquet"),
    )
    .await?;

    // Step 4: Check Covariate Balance
    info!("Step 4: Checking Covariate Balance");
    let balance_dir = config.output_dir.join("04_balance");
    tokio::fs::create_dir_all(&balance_dir)
        .await
        .map_err(IdsError::Io)?;

    let balance_report = calculate_balance(&[matched_cases_batch], &[matched_controls_batch])?;

    // Generate balance report
    let report_path = balance_dir.join("balance_report.csv");
    generate_balance_report(&report_path.to_string_lossy(), &balance_report)?;

    // Print summary
    info!("Study Design Pipeline Completed Successfully (async)");
    info!("Balance Report Summary:");
    info!(
        " - Total Covariates: {}",
        balance_report.summary.total_covariates
    );
    info!(
        " - Imbalanced Covariates: {}",
        balance_report.summary.imbalanced_covariates
    );
    info!(
        " - Max Standardized Difference: {:.4}",
        balance_report.summary.max_standardized_difference
    );
    info!(
        " - Mean Absolute Standardized Difference: {:.4}",
        balance_report.summary.mean_absolute_standardized_difference
    );

    Ok(())
}
