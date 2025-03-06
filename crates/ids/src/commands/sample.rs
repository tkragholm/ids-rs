use crate::error::IdsResult;
use core::utils::console::{format_duration_short, ConsoleOutput};
use core::utils::{load_records, validate_csv_format, MatchingCriteria};
use core::sampler::IncidenceDensitySampler;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Handle the sampling command
/// 
/// This function performs incidence density sampling to match controls to cases.
/// 
/// # Arguments
/// * `input` - Input CSV file containing case data
/// * `controls` - Number of controls to match per case
/// * `birth_window` - Birth date matching window in days
/// * `parent_window` - Parent age matching window in days
/// * `output_dir` - Directory where results will be saved
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
/// 
/// # Errors
/// Returns an error if sampling fails due to:
/// - Invalid input CSV format (missing required columns)
/// - File I/O errors when reading input or writing output
/// - Insufficient data to perform matching
/// - Statistical analysis or plot generation failures
pub fn handle_sampling(
    input: &str,
    controls: usize,
    birth_window: i64,
    parent_window: i64,
    output_dir: &str,
) -> IdsResult<()> {
    ConsoleOutput::section("Incidence Density Sampling");

    let start = Instant::now();
    
    // Validate input data
    ConsoleOutput::subsection("Data Validation");
    validate_and_load_data(input)?;

    // Configure matching criteria
    let criteria = MatchingCriteria {
        birth_date_window: birth_window,
        parent_date_window: parent_window,
    };

    ConsoleOutput::key_value("Birth date window", &format!("{birth_window} days"));
    ConsoleOutput::key_value("Parent date window", &format!("{parent_window} days"));

    // Create and initialize sampler
    let sampler = create_sampler(input, criteria)?;
    
    // Process sampling results
    process_sampling_results(&sampler, controls, output_dir)?;

    // Generate summary
    let total_time = start.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Input file", input);
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value("Total execution time", &format_duration_short(total_time));
    ConsoleOutput::success("Sampling completed successfully");

    Ok(())
}

/// Validate the CSV format and load data
/// 
/// # Arguments
/// * `input` - Path to the input CSV file
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
fn validate_and_load_data(input: &str) -> IdsResult<()> {
    let validation_start = Instant::now();
    match validate_csv_format(input) {
        Ok(()) => {
            let validation_time = validation_start.elapsed();
            ConsoleOutput::success(&format!(
                "CSV format validation completed in {validation_time:?}"
            ));
            Ok(())
        }
        Err(e) => {
            ConsoleOutput::error(&format!("CSV validation failed: {e}"));
            Err(crate::error::IdsError::validation(format!(
                "CSV validation failed: {e}"
            )))
        }
    }
}

/// Create and initialize the `IncidenceDensitySampler`
/// 
/// # Arguments
/// * `input` - Path to the input CSV file
/// * `criteria` - Matching criteria for the sampler
/// 
/// # Returns
/// * `IdsResult<IncidenceDensitySampler>` - Initialized sampler or error
fn create_sampler(
    input: &str,
    criteria: MatchingCriteria,
) -> IdsResult<IncidenceDensitySampler> {
    ConsoleOutput::subsection("Data Loading");
    ConsoleOutput::key_value("Input file", input);

    let start = Instant::now();
    let records = load_records(input).map_err(|e| crate::error::IdsError::data_loading(format!(
        "Failed to load records: {e}"
    )))?;
    let loading_time = start.elapsed();

    ConsoleOutput::key_value(
        "Records loaded",
        &ConsoleOutput::format_number(records.len()),
    );
    ConsoleOutput::key_value("Loading time", &format_duration_short(loading_time));

    ConsoleOutput::subsection("Sampler Initialization");
    let sampler_start = Instant::now();
    let sampler = IncidenceDensitySampler::new(records, criteria)
        .map_err(|e| crate::error::IdsError::sampling(format!(
            "Failed to initialize sampler: {e}"
        )))?;
    let init_time = sampler_start.elapsed();

    // Get statistics and display in a more structured way
    let stats = sampler.get_statistics();
    println!("{stats}");

    ConsoleOutput::key_value("Initialization time", &format_duration_short(init_time));

    Ok(sampler)
}

/// Process sampling results and save outputs
/// 
/// # Arguments
/// * `sampler` - The initialized sampler
/// * `controls` - Number of controls to sample per case
/// * `output_dir` - Directory where results will be saved
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
fn process_sampling_results(
    sampler: &IncidenceDensitySampler,
    controls: usize,
    output_dir: &str,
) -> IdsResult<()> {
    ConsoleOutput::subsection("Sampling Controls");
    ConsoleOutput::key_value("Requested controls per case", &controls.to_string());

    let sampling_start = Instant::now();
    let case_control_pairs = sampler.sample_controls(controls)
        .map_err(|e| crate::error::IdsError::sampling(format!(
            "Failed to sample controls: {e}"
        )))?;
    let sampling_time = sampling_start.elapsed();

    ConsoleOutput::key_value("Sampling time", &format_duration_short(sampling_time));
    ConsoleOutput::key_value_colored(
        "Matches found",
        &format!("{}", case_control_pairs.len()),
        !case_control_pairs.is_empty(),
    );

    ConsoleOutput::subsection("Saving Results");

    // Ensure output directory exists
    fs::create_dir_all(output_dir)?;

    // Save matched pairs
    let matches_path = Path::new(output_dir).join("matched_pairs.csv");
    match sampler.save_matches_to_csv(&case_control_pairs, &matches_path.to_string_lossy()) {
        Ok(()) => {
            ConsoleOutput::success(&format!("Matches saved to {}", matches_path.display()));
        }
        Err(e) => {
            ConsoleOutput::error(&format!("Error saving matches to CSV: {e}"));
            return Err(crate::error::IdsError::sampling(format!(
                "Error saving matches to CSV: {e}"
            )));
        }
    }

    // Save statistics
    let stats_path = Path::new(output_dir).join("matching_stats.csv");
    match sampler.save_matching_statistics(&case_control_pairs, &stats_path.to_string_lossy()) {
        Ok(()) => {
            ConsoleOutput::success(&format!("Statistics saved to {}", stats_path.display()));
        }
        Err(e) => {
            ConsoleOutput::error(&format!("Error saving statistics: {e}"));
            return Err(crate::error::IdsError::sampling(format!(
                "Error saving matching statistics: {e}"
            )));
        }
    }

    // Evaluate and display quality metrics
    ConsoleOutput::subsection("Matching Quality Analysis");
    let quality = sampler.evaluate_matching_quality(&case_control_pairs);
    println!("{}", quality.format_report());

    // Generate plots
    let plots_dir = Path::new(output_dir).join("plots");
    fs::create_dir_all(&plots_dir)?;

    match quality.generate_summary_plots(&plots_dir.to_string_lossy()) {
        Ok(()) => {
            ConsoleOutput::success(&format!(
                "Quality plots generated in {}",
                plots_dir.display()
            ));
        }
        Err(e) => {
            ConsoleOutput::error(&format!("Error generating plots: {e}"));
            return Err(crate::error::IdsError::sampling(format!(
                "Error generating plots: {e}"
            )));
        }
    }

    Ok(())
}