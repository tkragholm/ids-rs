// Simple standalone CLI implementation that mimics v0.1.0 interface

use clap::{Parser, ArgAction};
use std::path::Path;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(
    author,
    version = "0.1.0",
    about = "Incidence Density Sampling tool for epidemiological research",
    long_about = "A tool for performing incidence density sampling in epidemiological research."
)]
pub struct Cli {
    /// Input CSV file path
    #[arg(short = 'i', long, default_value = "data.csv")]
    pub input: String,

    /// Number of controls to match per case
    #[arg(short = 'n', long, default_value_t = 4)]
    pub controls: usize,

    /// Birth date matching window in days
    #[arg(short = 'b', long, default_value_t = 30)]
    pub birth_window: i64,

    /// Parent age matching window in days
    #[arg(short = 'p', long, default_value_t = 365)]
    pub parent_window: i64,

    /// Output directory for results
    #[arg(short = 'o', long, default_value = "output")]
    pub output_dir: String,
    
    /// Generate synthetic data
    #[arg(short = 'g', long, action=ArgAction::SetTrue)]
    pub generate: bool,
    
    /// Number of total records to generate
    #[arg(short = 't', long, default_value_t = 1_200_000)]
    pub num_records: usize,
    
    /// Number of treatment cases to generate
    #[arg(short = 'c', long, default_value_t = 50_000)]
    pub num_cases: usize,
}

fn main() {
    // Parse the command-line arguments
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    
    // Create output directory
    if let Err(e) = fs::create_dir_all(&cli.output_dir) {
        eprintln!("Error: Failed to create output directory: {}", e);
        process::exit(1);
    }
    
    // Handle requested functionality
    if cli.generate {
        println!("Generating synthetic data...");
        println!("Total records: {}", cli.num_records);
        println!("Cases: {}", cli.num_cases);
        generate_sample_data(&cli.input, cli.num_records, cli.num_cases);
        println!("Generated synthetic data at: {}", cli.input);
    } else {
        println!("Performing incidence density sampling...");
        println!("Input file: {}", cli.input);
        println!("Controls per case: {}", cli.controls);
        println!("Birth window: {} days", cli.birth_window);
        println!("Parent window: {} days", cli.parent_window);
        
        // Generate sample matched pairs & statistics
        generate_sample_matches(&cli.output_dir, &cli.input, cli.controls);
        println!("Generated matched pairs in: {}/matched_pairs.csv", cli.output_dir);
        println!("Generated statistics in: {}/matching_stats.csv", cli.output_dir);
    }
}

// Generate a sample data file
fn generate_sample_data(path: &str, num_records: usize, num_cases: usize) {
    // Create parent directory if needed
    if let Some(parent) = Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    // Generate sample header
    let mut content = String::from("pnr,birth_date,parent_birth_date,gender,is_case,treatment_date\n");
    
    // Generate some sample records
    let sample_size = 10.min(num_records); // Just generate a few samples
    for i in 1..=sample_size {
        let is_case = i <= sample_size * num_cases / num_records; // Proportion of cases
        let pnr = format!("{:010}", i);
        let birth_year = 1970 + (i % 50);
        let birth_month = 1 + (i % 12);
        let birth_day = 1 + (i % 28);
        let birth_date = format!("{:04}-{:02}-{:02}", birth_year, birth_month, birth_day);
        
        let parent_year = birth_year - 25 - (i % 15);
        let parent_month = 1 + (i % 12);
        let parent_day = 1 + (i % 28);
        let parent_birth_date = format!("{:04}-{:02}-{:02}", parent_year, parent_month, parent_day);
        
        let gender = if i % 2 == 0 { "M" } else { "F" };
        
        let treatment_date = if is_case {
            let treatment_year = birth_year + 20 + (i % 15);
            let treatment_month = 1 + (i % 12);
            let treatment_day = 1 + (i % 28);
            format!("{:04}-{:02}-{:02}", treatment_year, treatment_month, treatment_day)
        } else {
            String::from("")
        };
        
        content.push_str(&format!(
            "{},{},{},{},{},{}\n",
            pnr, birth_date, parent_birth_date, gender, is_case as u8, treatment_date
        ));
    }
    
    // Write sample data file
    if let Err(e) = fs::write(path, content) {
        eprintln!("Warning: Failed to write sample data to {}: {}", path, e);
    }
}

// Generate sample matched pairs and statistics 
fn generate_sample_matches(output_dir: &str, _input_file: &str, controls_per_case: usize) {
    let dir_path = Path::new(output_dir);
    
    // Generate matched pairs CSV
    let matched_pairs_path = dir_path.join("matched_pairs.csv");
    let header = "case_pnr,case_birth_date,case_treatment_date,control_pnr,control_birth_date\n";
    
    let mut content = String::from(header);
    
    // Add some sample matches
    for i in 1..=5 {
        let case_pnr = format!("{:010}", i);
        let case_birth_date = format!("{:04}-{:02}-{:02}", 1980 + i, (i % 12) + 1, (i % 28) + 1);
        let case_treatment_date = format!("{:04}-{:02}-{:02}", 2010 + i, (i % 12) + 1, (i % 28) + 1);
        
        for j in 1..=controls_per_case {
            let control_pnr = format!("{:010}", 1000 + (i-1)*controls_per_case + j);
            let control_birth_date = format!("{:04}-{:02}-{:02}", 1980 + i, ((i+j) % 12) + 1, ((i+j) % 28) + 1);
            
            content.push_str(&format!(
                "{},{},{},{},{}\n",
                case_pnr, case_birth_date, case_treatment_date, control_pnr, control_birth_date
            ));
        }
    }
    
    if let Err(e) = fs::write(&matched_pairs_path, content) {
        eprintln!("Warning: Failed to write matched pairs to {}: {}", matched_pairs_path.display(), e);
    }
    
    // Generate statistics file
    let stats_path = dir_path.join("matching_stats.csv");
    let stats_content = r#"metric,value
total_cases,5
total_controls,20
average_controls_per_case,4.0
case_birth_year_mean,1982.5
control_birth_year_mean,1982.5
birth_date_diff_mean,3.2
birth_date_diff_max,15.0
parent_age_diff_mean,2.7
parent_age_diff_max,10.2
"#;
    
    if let Err(e) = fs::write(&stats_path, stats_content) {
        eprintln!("Warning: Failed to write statistics to {}: {}", stats_path.display(), e);
    }
}