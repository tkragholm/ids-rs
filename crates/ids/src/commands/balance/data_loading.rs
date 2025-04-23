use crate::core::IdsResult;
use crate::utils::paths::normalize_path;
use chrono::NaiveDate;
use core::utils::console::{format_duration_short, ConsoleOutput};
use covariates::data::matched_pairs::load_matched_pairs;
use hashbrown::HashMap;
use loader::{ParallelLoader, RegisterPathConfig, StoreLoader};
use log::{error, info};
use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

use super::config::BalanceCheckConfig;

// Type alias to reduce complexity
pub type MatchedPairData = (Vec<(String, NaiveDate, Vec<String>)>, HashSet<String>);

/// Load matched pair data from the specified file
///
/// # Arguments
/// * `matches_path` - Path to the matches file
///
/// # Returns
/// * `IdsResult<MatchedPairData>` - Matched pairs and unique PNRs
///
/// # Errors
/// * Returns an error if the file is not found or cannot be loaded
pub fn load_matched_pair_data(matches_path: &Path) -> IdsResult<MatchedPairData> {
    if !matches_path.exists() {
        let error_msg = format!("Matched pairs file not found: {}", matches_path.display());
        ConsoleOutput::error(&error_msg);
        return Err(crate::core::IdsError::data_loading(error_msg));
    }

    let loading_start = Instant::now();

    // Load the matched pairs data
    let matched_pairs = match load_matched_pairs(matches_path) {
        Ok(pairs) => pairs,
        Err(e) => {
            let error_msg = format!("Failed to load matched pairs: {e}");
            ConsoleOutput::error(&error_msg);
            return Err(crate::core::IdsError::data_loading(error_msg));
        }
    };

    // Extract all unique PNRs for diagnostic mode (will be used if register data loading fails)
    let mut all_unique_pnrs = std::collections::HashSet::new();
    for (case_pnr, _, control_pnrs) in &matched_pairs {
        all_unique_pnrs.insert(case_pnr.clone());
        for control_pnr in control_pnrs {
            all_unique_pnrs.insert(control_pnr.clone());
        }
    }

    let loading_time = loading_start.elapsed();

    // Log loading statistics
    ConsoleOutput::key_value("Matched pairs loaded", &matched_pairs.len().to_string());
    ConsoleOutput::key_value("Unique PNRs found", &all_unique_pnrs.len().to_string());
    ConsoleOutput::key_value("Loading time", &format_duration_short(loading_time));

    Ok((matched_pairs, all_unique_pnrs))
}

/// Set up data paths for registers based on configuration
///
/// # Arguments
/// * `config` - Balance check configuration
///
/// # Returns
/// * `IdsResult<(String, HashMap<String, String>)>` - Base path and custom paths
///
/// # Errors
/// * Returns an error if no paths are specified
pub fn setup_data_paths(
    config: &BalanceCheckConfig,
) -> IdsResult<(String, HashMap<String, String>)> {
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

    // Determine base path
    let base_path = if let Some(base) = config.covariate_dir {
        base.to_string()
    } else if has_custom_paths {
        ConsoleOutput::warning("No base covariate directory specified, using custom paths only");
        "".to_string()
    } else {
        ConsoleOutput::error("No covariate directory or custom paths specified");
        return Err(crate::core::IdsError::config(
            "No covariate directory or custom paths specified",
        ));
    };

    Ok((base_path, custom_paths))
}

/// Load register data using specified paths
///
/// # Arguments
/// * `base_path` - Base path for register data
/// * `custom_paths` - Map of custom paths for specific registers
///
/// # Returns
/// * `IdsResult<types::storage::ArrowBackend>` - Loaded arrow store
///
/// # Errors
/// * Returns an error if data loading fails
pub fn load_register_data(
    base_path: &str,
    custom_paths: &HashMap<String, String>,
) -> IdsResult<types::storage::ArrowBackend> {
    ConsoleOutput::subsection("Loading Register Data");

    let loader = ParallelLoader::new();
    let load_start = Instant::now();
    let has_custom_paths = !custom_paths.is_empty();

    // Log loading details
    ConsoleOutput::info(&format!("Loading register data from: {base_path}"));
    for (register, path) in custom_paths {
        ConsoleOutput::info(&format!("Using custom {register} path: {path}"));
    }

    // Load data with the ParallelLoader
    let result = if has_custom_paths {
        let mut config = RegisterPathConfig::new(base_path.to_string());
        // Use builder pattern with the with_custom_path method
        for (register_type, path) in custom_paths {
            config = config.with_custom_path(register_type, path);
        }
        loader.load_with_custom_paths(config)
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
            use crate::diagnostic::BalanceCheckerDiagnostic;
            use covariates::balance::BalanceChecker;
            let _diagnostic_checker = BalanceChecker::new_diagnostic();
            ConsoleOutput::warning(
                "Using diagnostic mode with limited functionality - results will be incomplete",
            );
            return Err(crate::core::IdsError::data_loading(format!(
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

    Ok(store)
}
