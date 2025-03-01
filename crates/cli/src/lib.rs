pub mod cli;
pub use cli::{Cli, Commands};

/// Generate structured reports from balance results and matched pairs data
/// 
/// This function demonstrates the use of the structured output manager
/// to create a more organized, web-friendly output structure.
/// 
/// # Arguments
/// * `balance_results` - The balance calculation results
/// * `matched_pairs` - The matched pairs data
/// * `output_dir` - The directory to save reports to
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub fn generate_structured_reports(
    balance_results: &covariates::balance::results::BalanceResults,
    matched_pairs: &[covariates::matched_pairs::record::MatchedPairRecord],
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use covariates::reporting::StructuredOutputManager;
    use core::utils::console::ConsoleOutput;
    use std::time::Instant;
    
    // Create structured output manager
    ConsoleOutput::subsection("Generating Structured Reports");
    let start_time = Instant::now();
    
    let output_manager = StructuredOutputManager::new(output_dir)?
        .with_runtime_info("command", "generate-structured-reports")
        .with_runtime_info("timestamp", &chrono::Local::now().to_string());
    
    // Output balance results
    output_manager.output_balance_results(balance_results, None)?;
    
    // Output matched pairs data
    output_manager.output_matched_pairs(matched_pairs, None)?;
    
    // Generate HTML reports
    output_manager.generate_index_html()?;
    output_manager.generate_data_quality_report()?;
    
    // Log completion
    ConsoleOutput::success(&format!("Generated structured reports in {} seconds", 
        start_time.elapsed().as_secs()));
    ConsoleOutput::info(&format!("Reports available at: {}/report/", output_dir));
    
    Ok(())
}
