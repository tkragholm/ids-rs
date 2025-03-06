use crate::core::IdsResult;
use core::utils::console::ConsoleOutput;
use std::path::Path;
use std::time::Instant;

use super::config::BalanceCheckConfig;
use super::data_loading::{load_matched_pair_data, setup_data_paths, load_register_data};
use super::metrics::{calculate_balance_metrics, display_summary};
use super::reporting::generate_all_reports;

/// Handle the balance checking command
/// 
/// This function performs covariate balance analysis between matched cases and controls.
/// 
/// # Arguments
/// * `config` - Configuration for the balance check command
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
///
/// # Errors
/// Returns an error if balance checking fails due to:
/// - Missing files or directories
/// - Failed data loading
/// - Invalid covariate processing
/// - CSV or report generation failures
pub fn handle_balance_check(config: &BalanceCheckConfig) -> IdsResult<()> {
    ConsoleOutput::section("Covariate Balance Analysis");
    let start = Instant::now();
    
    // Step 1: Load matched pairs
    ConsoleOutput::subsection("Loading Matched Pairs");
    let matches_path = Path::new(config.matches_file);
    let (matched_pairs, _all_unique_pnrs) = load_matched_pair_data(matches_path)?;
    
    // Step 2: Set up paths
    let (base_path, custom_paths) = setup_data_paths(config)?;
    
    // Step 3: Load register data
    let store = load_register_data(&base_path, &custom_paths)?;
    
    // Step 4: Calculate balance metrics
    let (balance_result, matched_pair_records) = calculate_balance_metrics(matches_path, &store)?;
    
    // Step 5: Generate reports
    generate_all_reports(config, &balance_result, &matched_pair_records)?;
    
    // Step 6: Display summary
    display_summary(start, &matched_pairs, &balance_result);
    
    Ok(())
}