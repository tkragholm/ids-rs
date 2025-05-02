use crate::core::IdsResult;
use crate::utils::reports::generate_structured_reports;
use core::utils::console::{format_duration_short, ConsoleOutput};
use covariates::balance::BalanceResults;
use covariates::data::matched_pairs::MatchedPairRecord;
use covariates::reporting::ComprehensiveReport;
use log::error;
use std::path::Path;
use std::time::Instant;

use super::config::BalanceCheckConfig;

/// Generate reports from balance results
///
/// # Arguments
/// * `config` - Balance check configuration
/// * `balance_result` - Result of balance calculation
/// * `matched_pair_records` - Matched pair records
///
/// # Returns
/// * `IdsResult<()>` - Success or error
///
/// # Errors
/// * Returns an error if report generation fails
pub fn generate_all_reports(
    config: &BalanceCheckConfig,
    balance_result: &BalanceResults,
    matched_pair_records: &[MatchedPairRecord],
) -> IdsResult<()> {
    ConsoleOutput::subsection("Generating Reports");
    let report_start = Instant::now();

    // Generate comprehensive report
    let report_dir = Path::new(config.output_dir).join("report");
    ConsoleOutput::info(&format!(
        "Generating comprehensive report in {}",
        report_dir.display()
    ));

    // Create report directory if it doesn't exist
    if !report_dir.exists() {
        std::fs::create_dir_all(&report_dir)?;
    }

    let report = ComprehensiveReport::new(balance_result.clone());
    if let Err(e) = report.save_to_files(&report_dir) {
        ConsoleOutput::error(&format!("Failed to generate report files: {e}"));
        error!("Failed to generate report files: {e}");
    } else {
        ConsoleOutput::success(&format!(
            "Comprehensive report saved to {}",
            report_dir.display()
        ));
    }

    // Generate plots
    let plots_dir = Path::new(config.output_dir).join("plots");
    if let Err(e) = report.generate_plots(&plots_dir) {
        ConsoleOutput::error(&format!("Failed to generate plots: {e}"));
        error!("Failed to generate plots: {e}");
    } else {
        ConsoleOutput::success(&format!("Plots generated in {}", plots_dir.display()));
    }

    // Generate CSV report with raw metric data
    // The CSV is already saved by the comprehensive report in the save_to_files method
    let csv_path = format!("{}/report/covariate_balance.csv", config.output_dir);
    ConsoleOutput::info(&format!("Balance metrics saved to {csv_path}"));

    // Generate structured reports if requested
    if config.generate_structured_output {
        if let Err(e) =
            generate_structured_reports(balance_result, matched_pair_records, config.output_dir)
        {
            ConsoleOutput::error(&format!("Failed to generate structured reports: {e:?}"));
            error!("Failed to generate structured reports: {e:?}");
        }
    }

    let report_time = report_start.elapsed();
    ConsoleOutput::success(&format!(
        "Report generation completed in {}",
        format_duration_short(report_time)
    ));

    Ok(())
}
