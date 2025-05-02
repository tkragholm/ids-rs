use crate::core::IdsResult;
use chrono::NaiveDate;
use core::utils::console::{format_duration_short, ConsoleOutput};
use covariates::balance::{BalanceChecker, BalanceResults};
use covariates::data::matched_pairs::load_matched_pair_records;
use covariates::data::matched_pairs::MatchedPairRecord;
use std::path::Path;
use std::time::Instant;

/// Calculate balance metrics between cases and controls
///
/// # Arguments
/// * `matches_path` - Path to the matches file
/// * `store` - Arrow store containing register data
///
/// # Returns
/// * `IdsResult<(BalanceResults, Vec<MatchedPairRecord>)>` -
///   Balance result and matched pair records
///
/// # Errors
/// * Returns an error if balance calculation fails
pub fn calculate_balance_metrics(
    matches_path: &Path,
    store: &types::storage::ArrowBackend,
) -> IdsResult<(BalanceResults, Vec<MatchedPairRecord>)> {
    ConsoleOutput::subsection("Calculating Balance Metrics");
    let checker = BalanceChecker::new(store.clone());
    let calc_start = Instant::now();

    // Load the matched pair records directly from the file
    let matched_pair_records = match load_matched_pair_records(matches_path) {
        Ok(records) => records,
        Err(e) => {
            let error_msg = format!("Failed to process matched pairs: {e}");
            ConsoleOutput::error(&error_msg);
            return Err(crate::core::IdsError::balance_calculation(error_msg));
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
            let error_msg = format!("Failed to calculate balance metrics: {e}");
            ConsoleOutput::error(&error_msg);
            return Err(crate::core::IdsError::balance_calculation(error_msg));
        }
    };

    let calc_time = calc_start.elapsed();
    ConsoleOutput::success(&format!(
        "Balance calculations completed in {}",
        format_duration_short(calc_time)
    ));

    Ok((balance_result, matched_pair_records))
}

/// Display a summary of the balance analysis
///
/// # Arguments
/// * `start_time` - Time when the analysis started
/// * `matched_pairs` - List of matched pairs
/// * `balance_result` - Result of balance calculation
pub fn display_summary(
    start_time: Instant,
    matched_pairs: &[(String, NaiveDate, Vec<String>)],
    balance_result: &BalanceResults,
) {
    let total_time = start_time.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Matched pairs analyzed", &matched_pairs.len().to_string());
    ConsoleOutput::key_value(
        "Variables checked",
        &balance_result.summaries.len().to_string(),
    );
    ConsoleOutput::key_value("Total execution time", &format_duration_short(total_time));
    ConsoleOutput::success("Balance analysis completed successfully");
}
