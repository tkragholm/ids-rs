use crate::error::IdsResult;
use core::utils::console::{format_duration_short, ConsoleOutput};
use datagen::{GeneratorConfig, RegisterGenerator};
use std::path::Path;
use std::time::Instant;

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
pub fn handle_generate_registers(
    output_dir: &str,
    num_records: usize,
    num_cases: usize,
    start_year: i32,
    end_year: i32,
    seed: Option<u64>,
) -> IdsResult<()> {
    ConsoleOutput::section("Synthetic Register Data Generation");

    let start = Instant::now();

    // Configuration
    ConsoleOutput::subsection("Configuration");
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value("Total records", &ConsoleOutput::format_number(num_records));
    ConsoleOutput::key_value("Case records", &ConsoleOutput::format_number(num_cases));
    ConsoleOutput::key_value("Year range", &format!("{start_year} - {end_year}"));

    if let Some(s) = seed {
        ConsoleOutput::key_value("Random seed", &s.to_string());
    } else {
        ConsoleOutput::key_value("Random seed", "None (using system random)");
    }

    // Create configuration
    let mut config = GeneratorConfig::new(num_records, num_cases, output_dir.to_string())
        .with_year_range(start_year, end_year);
    if let Some(s) = seed {
        config = config.with_seed(s);
    }

    // Generation
    ConsoleOutput::subsection("Generating Data");
    let generation_start = Instant::now();
    let mut generator = RegisterGenerator::new(config)?;
    generator.generate_all()?;
    let generation_time = generation_start.elapsed();

    ConsoleOutput::key_value("Generation time", &format_duration_short(generation_time));

    // Pediatric data
    ConsoleOutput::subsection("Generating Pediatric Data");
    let pediatric_start = Instant::now();
    let pediatric_path = Path::new(output_dir).join("pediatric.csv");
    let pediatric_path_str = pediatric_path
        .to_str()
        .ok_or_else(|| crate::error::IdsError::path_resolution(format!(
            "Invalid path: {}", pediatric_path.display()
        )))?;
    generator.generate_pediatric(pediatric_path_str)?;
    let pediatric_time = pediatric_start.elapsed();

    ConsoleOutput::key_value("Pediatric data file", &pediatric_path.display().to_string());
    ConsoleOutput::key_value(
        "Pediatric generation time",
        &format_duration_short(pediatric_time),
    );

    // Summary
    let total_time = start.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value(
        "Records generated",
        &ConsoleOutput::format_number(num_records),
    );
    ConsoleOutput::key_value("Total execution time", &format_duration_short(total_time));
    ConsoleOutput::success("Register data generation completed successfully");

    Ok(())
}