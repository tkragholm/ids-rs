use crate::error::IdsResult;
use crate::utils::{generate_structured_reports, normalize_path};
use core::utils::console::{format_duration_short, ConsoleOutput};
use covariates::balance::BalanceChecker;
use covariates::matched_pairs::{load_matched_pairs, load_matched_pair_records};
use covariates::reporting::ComprehensiveReport;
use hashbrown::HashMap;
use loader::ParquetLoader;
use log::{error, info};
use std::path::Path;
use std::time::Instant;

/// Configuration for the balance check command
#[derive(Debug, Clone)]
pub struct BalanceCheckConfig<'a> {
    pub matches_file: &'a str,
    pub covariate_dir: Option<&'a str>,
    pub output_dir: &'a str,
    pub family_file: Option<&'a str>,
    pub akm_dir: Option<&'a str>,
    pub bef_dir: Option<&'a str>,
    pub ind_dir: Option<&'a str>,
    pub uddf_dir: Option<&'a str>,
    pub generate_structured_output: bool,
}

/// Handle the balance checking command
/// 
/// This function performs covariate balance analysis between matched cases and controls.
/// 
/// # Arguments
/// * `config` - Configuration for the balance check command
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
pub fn handle_balance_check(config: &BalanceCheckConfig) -> IdsResult<()> {
    ConsoleOutput::section("Covariate Balance Analysis");

    let start = Instant::now();

    // Step 1: Load matched pairs
    ConsoleOutput::subsection("Loading Matched Pairs");
    let matches_path = Path::new(config.matches_file);
    if !matches_path.exists() {
        ConsoleOutput::error(&format!(
            "Matched pairs file not found: {}",
            matches_path.display()
        ));
        return Err(crate::error::IdsError::data_loading(format!(
            "Matched pairs file not found: {}",
            matches_path.display()
        )));
    }

    let loading_start = Instant::now();
    let matched_pairs = match load_matched_pairs(matches_path) {
        Ok(pairs) => pairs,
        Err(e) => {
            ConsoleOutput::error(&format!("Failed to load matched pairs: {e}"));
            return Err(crate::error::IdsError::data_loading(format!(
                "Failed to load matched pairs: {e}"
            )));
        }
    };
    let loading_time = loading_start.elapsed();

    // Extract all unique PNRs for diagnostic mode (will be used if register data loading fails)
    let mut all_unique_pnrs = std::collections::HashSet::new();
    for (case_pnr, _, control_pnrs) in &matched_pairs {
        all_unique_pnrs.insert(case_pnr.clone());
        for control_pnr in control_pnrs {
            all_unique_pnrs.insert(control_pnr.clone());
        }
    }
    let unique_pnrs_vec: Vec<String> = all_unique_pnrs.into_iter().collect();

    ConsoleOutput::key_value("Matched pairs loaded", &matched_pairs.len().to_string());
    ConsoleOutput::key_value("Unique PNRs found", &unique_pnrs_vec.len().to_string());
    ConsoleOutput::key_value("Loading time", &format_duration_short(loading_time));

    // Step 2: Set up custom paths with proper resolution
    let mut custom_paths = HashMap::new();
    let mut has_custom_paths = false;

    // Handle family file path if specified
    if let Some(family_path) = config.family_file {
        let normalized_path = normalize_path(family_path, "family", config.covariate_dir);
        custom_paths.insert("family".to_string(), normalized_path);
        has_custom_paths = true;
    }

    // Handle register directory paths if specified
    if let Some(akm_dir) = config.akm_dir {
        let normalized_path = normalize_path(akm_dir, "akm", config.covariate_dir);
        custom_paths.insert("akm".to_string(), normalized_path);
        has_custom_paths = true;
    }

    if let Some(bef_dir) = config.bef_dir {
        let normalized_path = normalize_path(bef_dir, "bef", config.covariate_dir);
        custom_paths.insert("bef".to_string(), normalized_path);
        has_custom_paths = true;
    }

    if let Some(ind_dir) = config.ind_dir {
        let normalized_path = normalize_path(ind_dir, "ind", config.covariate_dir);
        custom_paths.insert("ind".to_string(), normalized_path);
        has_custom_paths = true;
    }

    if let Some(uddf_dir) = config.uddf_dir {
        let normalized_path = normalize_path(uddf_dir, "uddf", config.covariate_dir);
        custom_paths.insert("uddf".to_string(), normalized_path);
        has_custom_paths = true;
    }

    // Step 3: Load register data
    ConsoleOutput::subsection("Loading Register Data");
    let base_path = if let Some(base) = config.covariate_dir {
        base
    } else if has_custom_paths {
        ConsoleOutput::warning("No base covariate directory specified, using custom paths only");
        ""
    } else {
        ConsoleOutput::error("No covariate directory or custom paths specified");
        return Err(crate::error::IdsError::config(
            "No covariate directory or custom paths specified"
        ));
    };

    let loader = ParquetLoader::new();
    let load_start = Instant::now();

    ConsoleOutput::info(&format!("Loading register data from: {}", base_path));
    for (register, path) in &custom_paths {
        ConsoleOutput::info(&format!("Using custom {} path: {}", register, path));
    }

    // Load data with the ParquetLoader
    let result = if has_custom_paths {
        loader.load_with_custom_paths_map(base_path.to_string(), custom_paths)
    } else {
        loader.load_from_path(base_path.to_string())
    };

    // Check if loading was successful
    let store = match result {
        Ok(s) => s,
        Err(e) => {
            ConsoleOutput::error(&format!("Failed to load register data: {e}"));
            ConsoleOutput::warning("Will continue in diagnostic mode with limited functionality");
            error!("Failed to load register data: {}", e);
            
            // Create a diagnostic mode store with no data
            let _diagnostic_checker = BalanceChecker::new_diagnostic();
            ConsoleOutput::warning("Using diagnostic mode with limited functionality - results will be incomplete");
            return Err(crate::error::IdsError::data_loading(format!(
                "Failed to load register data: {e}"
            )));
        }
    };

    let load_time = load_start.elapsed();
    ConsoleOutput::success(&format!(
        "Register data loaded in {}",
        format_duration_short(load_time)
    ));
    info!("Register data loaded in {:?}", load_time);

    // Step 4: Calculate balance metrics
    ConsoleOutput::subsection("Calculating Balance Metrics");
    let checker = BalanceChecker::new(store);

    let calc_start = Instant::now();
    
    // Load the matched pair records directly from the file
    let matched_pair_records = match load_matched_pair_records(matches_path) {
        Ok(records) => records,
        Err(e) => {
            ConsoleOutput::error(&format!("Failed to process matched pairs: {e}"));
            return Err(crate::error::IdsError::balance_calculation(format!(
                "Failed to process matched pairs: {e}"
            )));
        }
    };

    // Extract cases and controls for balance calculation
    let mut cases = Vec::new();
    let mut controls = Vec::new();
    
    for record in &matched_pair_records {
        // The treatment date is the date we evaluate covariates at
        cases.push((record.case_pnr.clone(), record.case_treatment_date));
        
        // Each record has a single control, so we add it
        controls.push((record.control_pnr.clone(), record.case_treatment_date));
    }
    
    // Calculate balance metrics
    let balance_result = match checker.calculate_balance(&cases, &controls) {
        Ok(result) => result,
        Err(e) => {
            ConsoleOutput::error(&format!("Failed to calculate balance metrics: {e}"));
            return Err(crate::error::IdsError::balance_calculation(format!(
                "Failed to calculate balance metrics: {e}"
            )));
        }
    };
    let calc_time = calc_start.elapsed();
    ConsoleOutput::success(&format!(
        "Balance calculations completed in {}",
        format_duration_short(calc_time)
    ));

    // Step 5: Generate reports
    ConsoleOutput::subsection("Generating Reports");
    let report_start = Instant::now();

    // Generate comprehensive report
    let report_dir = Path::new(config.output_dir).join("report");
    ConsoleOutput::info(&format!("Generating comprehensive report in {}", report_dir.display()));

    // Create report directory if it doesn't exist
    if !report_dir.exists() {
        std::fs::create_dir_all(&report_dir)?;
    }

    let report = ComprehensiveReport::new(balance_result.clone());
    if let Err(e) = report.save_to_files(&report_dir) {
        ConsoleOutput::error(&format!("Failed to generate report files: {e}"));
        error!("Failed to generate report files: {}", e);
    } else {
        ConsoleOutput::success(&format!("Comprehensive report saved to {}", report_dir.display()));
    }
    
    // Generate plots
    let plots_dir = Path::new(config.output_dir).join("plots");
    if let Err(e) = report.generate_plots(&plots_dir) {
        ConsoleOutput::error(&format!("Failed to generate plots: {e}"));
        error!("Failed to generate plots: {}", e);
    } else {
        ConsoleOutput::success(&format!("Plots generated in {}", plots_dir.display()));
    }

    // Generate CSV report with raw metric data
    // The CSV is already saved by the comprehensive report in the save_to_files method
    // So we don't need to separately save the metrics
    let csv_path = format!("{}/report/covariate_balance.csv", config.output_dir);
    ConsoleOutput::info(&format!("Balance metrics saved to {}", csv_path));

    // Generate structured reports if requested
    if config.generate_structured_output {
        if let Err(e) = generate_structured_reports(&balance_result, &matched_pair_records, config.output_dir) {
            ConsoleOutput::error(&format!("Failed to generate structured reports: {e:?}"));
            error!("Failed to generate structured reports: {e:?}");
        }
    }

    let report_time = report_start.elapsed();
    ConsoleOutput::success(&format!(
        "Report generation completed in {}",
        format_duration_short(report_time)
    ));

    // Step 6: Summary
    let total_time = start.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Matched pairs analyzed", &matched_pairs.len().to_string());
    ConsoleOutput::key_value("Variables checked", &balance_result.summaries.len().to_string());
    ConsoleOutput::key_value("Total execution time", &format_duration_short(total_time));
    ConsoleOutput::success("Balance analysis completed successfully");

    Ok(())
}