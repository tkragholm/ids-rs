use crate::core::IdsResult;
use datagen::{GeneratorConfig, RegisterGenerator};
use std::path::Path;
use std::time::Instant;
use utils::RichConsole;

/// Handle the register generation command
/// 
/// This function generates synthetic register data for research purposes.
/// 
/// # Arguments
/// * `output_dir` - Directory for register data output
/// * `num_records` - Number of total records to generate
/// * `num_cases` - Number of treatment cases to generate
/// * `start_year` - Start year for data generation
/// * `end_year` - End year for data generation
/// * `seed` - Random seed for reproducibility
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
/// 
/// # Errors
/// Returns an error if register generation fails due to:
/// - Invalid configuration parameters
/// - File I/O errors during data writing
/// - Memory allocation issues with large datasets
/// - Path resolution problems
pub fn handle_generate_registers(
    output_dir: &str,
    num_records: usize,
    num_cases: usize,
    start_year: i32,
    end_year: i32,
    seed: Option<u64>,
) -> IdsResult<()> {
    let console = RichConsole::new();
    let start = Instant::now();

    // Header
    console.header("Synthetic Register Data Generation")?;

    // Configuration panel
    console.panel("Configuration", |ui| {
        ui.key_value("Output directory", output_dir)?;
        ui.key_value("Total records", RichConsole::format_number(num_records))?;
        ui.key_value("Case records", RichConsole::format_number(num_cases))?;
        ui.key_value("Year range", format!("{start_year} - {end_year}"))?;

        if let Some(s) = seed {
            ui.key_value("Random seed", s.to_string())?;
        } else {
            ui.key_value("Random seed", "None (using system random)")?;
        }
        Ok(())
    })?;

    // Create configuration
    let mut config = GeneratorConfig::new(num_records, num_cases, output_dir.to_string())
        .with_year_range(start_year, end_year);
    if let Some(s) = seed {
        config = config.with_seed(s);
    }

    // Generation
    console.subheader("Generating Data")?;
    let generation_start = Instant::now();
    let mut generator = RegisterGenerator::new(config)?;
    generator.generate_all()?;
    let generation_time = generation_start.elapsed();

    console.key_value("Generation time", RichConsole::format_duration(generation_time))?;

    // Pediatric data
    console.subheader("Generating Pediatric Data")?;
    let pediatric_start = Instant::now();
    let pediatric_path = Path::new(output_dir).join("pediatric.csv");
    let pediatric_path_str = pediatric_path
        .to_str()
        .ok_or_else(|| crate::core::IdsError::path_resolution(format!(
            "Invalid path: {}", pediatric_path.display()
        )))?;
    generator.generate_pediatric(pediatric_path_str)?;
    let pediatric_time = pediatric_start.elapsed();

    console.key_value("Pediatric data file", pediatric_path.display().to_string())?;
    console.key_value("Pediatric generation time", RichConsole::format_duration(pediatric_time))?;

    // Summary table
    console.header("Summary")?;
    
    // Show registry file summary in a table
    console.table(
        &["Registry", "Records", "Format", "Path"],
        &[
            vec!["BEF".to_string(), RichConsole::format_number(num_records), "Parquet".to_string(), format!("{}/bef/", output_dir)],
            vec!["AKM".to_string(), RichConsole::format_number(num_records), "Parquet".to_string(), format!("{}/akm/", output_dir)],
            vec!["IND".to_string(), RichConsole::format_number(num_records), "Parquet".to_string(), format!("{}/ind/", output_dir)],
            vec!["UDDF".to_string(), RichConsole::format_number(num_records), "Parquet".to_string(), format!("{}/uddf/", output_dir)],
            vec!["Family".to_string(), RichConsole::format_number(num_records), "Parquet".to_string(), format!("{}/family.parquet", output_dir)],
            vec!["Pediatric".to_string(), RichConsole::format_number(num_cases), "CSV".to_string(), format!("{}/pediatric.csv", output_dir)],
        ]
    )?;

    // Final summary panel
    console.panel("Generation Complete", |ui| {
        ui.key_value("Total execution time", RichConsole::format_duration(start.elapsed()))?;
        ui.key_value("Total records", RichConsole::format_number(num_records))?;
        ui.key_value("Case records", RichConsole::format_number(num_cases))?;
        ui.key_value("Output directory", output_dir)?;
        Ok(())
    })?;
    
    console.success("Register data generation completed successfully")?;

    Ok(())
}