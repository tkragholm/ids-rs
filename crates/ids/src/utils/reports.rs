use crate::error::IdsResult;
use covariates::balance::results::BalanceResults;
use covariates::matched_pairs::record::MatchedPairRecord;
use covariates::reporting::StructuredOutputManager;
use core::utils::console::ConsoleOutput;
use std::time::Instant;

/// Generate structured reports from balance results and matched pairs data
/// 
/// This function creates organized, web-friendly output reports, including
/// HTML visualizations and structured data files.
/// 
/// # Arguments
/// * `balance_results` - The balance calculation results
/// * `matched_pairs` - The matched pairs data
/// * `output_dir` - The directory to save reports to
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
pub fn generate_structured_reports(
    balance_results: &BalanceResults,
    matched_pairs: &[MatchedPairRecord],
    output_dir: &str,
) -> IdsResult<()> {
    // Create structured output manager
    ConsoleOutput::subsection("Generating Structured Reports");
    let start_time = Instant::now();
    
    let output_manager = match StructuredOutputManager::new(output_dir) {
        Ok(manager) => manager
            .with_runtime_info("command", "generate-structured-reports")
            .with_runtime_info("timestamp", chrono::Local::now().to_string()),
        Err(e) => return Err(crate::error::IdsError::covariate(e))
    };
    
    // Output balance results
    if let Err(e) = output_manager.output_balance_results(balance_results, None) {
        return Err(crate::error::IdsError::covariate(e));
    }
    
    // Output matched pairs data
    if let Err(e) = output_manager.output_matched_pairs(matched_pairs, None) {
        return Err(crate::error::IdsError::covariate(e));
    }
    
    // Generate HTML reports
    if let Err(e) = output_manager.generate_index_html() {
        return Err(crate::error::IdsError::covariate(e));
    }
    
    if let Err(e) = output_manager.generate_data_quality_report() {
        return Err(crate::error::IdsError::covariate(e));
    }
    
    // Log completion
    ConsoleOutput::success(&format!("Generated structured reports in {} seconds", 
        start_time.elapsed().as_secs()));
    ConsoleOutput::info(&format!("Reports available at: {output_dir}/report/"));
    
    Ok(())
}