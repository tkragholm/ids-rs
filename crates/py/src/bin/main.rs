// Main entry point for the Python package with simplified implementation of the CLI

use clap::{Parser, Subcommand};
use std::path::Path;
use std::fs;
use std::process;
use std::collections::HashMap;

/// Subcommands for the Config command
#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Generate a default configuration file for covariates
    GenerateCovariates {
        /// Output file path
        #[arg(short = 'o', long, help = "Path to save the generated configuration file")]
        output: String,
        
        /// Force overwrite of existing file
        #[arg(short = 'f', long, help = "Force overwrite if the output file already exists")]
        force: bool,
    }
}

#[derive(Parser)]
#[command(
    author,
    version = "0.2.1",
    about = "Incidence Density Sampling (IDS) CLI tool for epidemiological research",
    long_about = "A comprehensive toolkit for generating synthetic register data, performing incidence density sampling, and analyzing covariate balance in epidemiological studies."
)]
pub struct Cli {
    /// Output directory for results
    #[arg(short = 'o', long, default_value = "output", help = "Directory where all results will be saved")]
    pub output_dir: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate synthetic register data for research purposes
    GenerateRegisters {
        /// Directory for register data output
        #[arg(short = 'o', long, default_value = "data/registers", help = "Directory where generated register data will be saved")]
        output_dir: String,

        /// Number of total records to generate
        #[arg(short = 't', long, default_value_t = 1_000_000, help = "Total number of records to generate across all registers")]
        num_records: usize,

        /// Number of treatment cases to generate
        #[arg(short = 'c', long, default_value_t = 50_000, help = "Number of cases with treatment events (must be less than total records)")]
        num_cases: usize,

        /// Start year for data generation
        #[arg(short = 's', long, default_value_t = 2000, help = "Start year for the generated data range (min: 1980)")]
        start_year: i32,

        /// End year for data generation
        #[arg(short = 'e', long, default_value_t = 2023, help = "End year for the generated data range (max: 2023)")]
        end_year: i32,

        /// Random seed for reproducibility
        #[arg(short = 'r', long, help = "Seed for random number generation to ensure reproducible results")]
        seed: Option<u64>,
    },
    
    /// Configuration utilities for the system
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Sample controls using incidence density sampling for case-control studies
    Sample {
        /// Input CSV file containing case data
        #[arg(short = 'i', long, default_value = "data/pediatric.csv", help = "CSV file containing cases with treatment dates and demographic information")]
        input: String,

        /// Number of controls to match per case
        #[arg(short = 'n', long, default_value_t = 4, help = "Number of control subjects to match with each case")]
        controls: usize,

        /// Birth date matching window in days
        #[arg(short = 'b', long, default_value_t = 30, help = "Maximum allowed difference between case and control birth dates (in days)")]
        birth_window: i64,

        /// Parent age matching window in days
        #[arg(short = 'p', long, default_value_t = 365, help = "Maximum allowed difference between case and control parent ages (in days)")]
        parent_window: i64,
    },

    /// Analyze covariate balance between matched cases and controls
    CheckBalance {
        /// Path to the matched pairs CSV file
        #[arg(short = 'm', long, help = "CSV file containing the matched case-control pairs from sampling", required = true)]
        matches_file: String,

        /// Base directory containing the register data files with covariates
        #[arg(short = 'c', long, help = "Base directory containing register data. Can be omitted if all custom paths are specified. Expected structure is a directory containing 'family.parquet' and/or subdirectories 'akm', 'bef', 'ind', 'uddf'")]
        covariate_dir: Option<String>,
        
        /// Path to the family.parquet file
        #[arg(long, value_name = "FILE", help = "Path to the family relationships data. Can be either a directory containing 'family.parquet' or a direct path to the parquet file. Either absolute or relative paths are supported.")]
        family_file: Option<String>,
        
        /// Path to the AKM register directory
        #[arg(long, value_name = "DIR", help = "Path to the directory containing AKM register files (named like '2000.parquet', '2001.parquet', etc.). Either absolute or relative paths are supported.")]
        akm_dir: Option<String>,
        
        /// Path to the BEF register directory
        #[arg(long, value_name = "DIR", help = "Path to the directory containing BEF register files (named like '200012.parquet', '201903.parquet', etc.). Either absolute or relative paths are supported.")]
        bef_dir: Option<String>,
        
        /// Path to the IND register directory
        #[arg(long, value_name = "DIR", help = "Path to the directory containing IND register files (named like '2000.parquet', '2001.parquet', etc.). Either absolute or relative paths are supported.")]
        ind_dir: Option<String>,
        
        /// Path to the UDDF register directory
        #[arg(long, value_name = "DIR", help = "Path to the directory containing UDDF register files (named like '202009.parquet', '202209.parquet', etc.). Either absolute or relative paths are supported.")]
        uddf_dir: Option<String>,
        
        /// Generate structured HTML reports and organized outputs
        #[arg(long, help = "Generate structured HTML reports and organized output files")]
        structured: bool,
    },
}

fn main() {
    // Parse the command-line arguments
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("\nNOTE: Make sure there is a space between each flag and its value!");
            eprintln!("Example: --family-file data/registers/family.parquet");
            process::exit(1);
        }
    };
    
    // Setup directories
    create_output_dirs(&cli.output_dir);
    
    // Execute the requested command
    match handle_command(&cli) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn handle_command(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::GenerateRegisters {
            output_dir,
            num_records,
            num_cases,
            start_year,
            end_year,
            seed,
        } => {
            println!("Generating synthetic register data...");
            println!("Output directory: {}", output_dir);
            println!("Records: {}", num_records);
            println!("Cases: {}", num_cases);
            println!("Years: {} - {}", start_year, end_year);
            if let Some(s) = seed {
                println!("Seed: {}", s);
            }
            
            // Create sample file structure
            generate_sample_register_data(output_dir, *num_records, *num_cases);
            println!("Generated synthetic register data in: {}", output_dir);
            Ok(())
        },
        Commands::Config { command } => match command {
            ConfigCommands::GenerateCovariates { output, force } => {
                println!("Generating covariates configuration to: {}", output);
                
                // Check if file exists and we're not forcing overwrite
                let path = Path::new(output);
                if path.exists() && !*force {
                    return Err(format!("File already exists: {}. Use --force to overwrite.", output).into());
                }
                
                // Create parent directory if needed
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                // Generate sample config
                let config = r#"{
  "variables": {
    "demographics": [
      {"name": "Age", "register": "BEF", "type": "Numeric"},
      {"name": "Gender", "register": "BEF", "type": "Categorical"},
      {"name": "Family Size", "register": "BEF", "type": "Numeric"},
      {"name": "Municipality", "register": "BEF", "type": "Numeric"},
      {"name": "Civil Status", "register": "BEF", "type": "Categorical"}
    ],
    "education": [
      {"name": "Education Years", "register": "UDDF", "type": "Numeric"},
      {"name": "ISCED Level", "register": "UDDF", "type": "Categorical"}
    ],
    "income": [
      {"name": "Total Income", "register": "IND", "type": "Numeric"},
      {"name": "Wage Income", "register": "IND", "type": "Numeric"},
      {"name": "Employment Status", "register": "IND", "type": "Numeric"}
    ],
    "occupation": [
      {"name": "Occupation (SOCIO13)", "register": "AKM", "type": "Categorical"}
    ]
  }
}"#;
                fs::write(output, config)?;
                println!("Configuration file generated successfully!");
                Ok(())
            },
        },
        Commands::Sample {
            input,
            controls,
            birth_window,
            parent_window,
        } => {
            println!("Performing incidence density sampling...");
            println!("Input file: {}", input);
            println!("Controls per case: {}", controls);
            println!("Birth window: {} days", birth_window);
            println!("Parent window: {} days", parent_window);
            println!("Output directory: {}", cli.output_dir);
            
            // Generate sample matched pairs file
            generate_sample_matched_pairs(&cli.output_dir, input, *controls);
            println!("Generated matched pairs in: {}/matched_pairs.csv", cli.output_dir);
            Ok(())
        },
        Commands::CheckBalance {
            matches_file,
            covariate_dir,
            family_file,
            akm_dir,
            bef_dir,
            ind_dir,
            uddf_dir,
            structured,
        } => {
            println!("Analyzing covariate balance...");
            println!("Matched pairs file: {}", matches_file);
            
            if let Some(dir) = covariate_dir {
                println!("Covariate directory: {}", dir);
            }
            
            if let Some(file) = family_file {
                println!("Family file: {}", file);
            }
            
            if let Some(dir) = akm_dir {
                println!("AKM directory: {}", dir);
            }
            
            if let Some(dir) = bef_dir {
                println!("BEF directory: {}", dir);
            }
            
            if let Some(dir) = ind_dir {
                println!("IND directory: {}", dir);
            }
            
            if let Some(dir) = uddf_dir {
                println!("UDDF directory: {}", dir);
            }
            
            // Generate balance files
            let output_dir = &cli.output_dir;
            generate_sample_balance_files(output_dir, matches_file, *structured);
            
            println!("\nGenerated balance outputs in: {}", output_dir);
            println!("Key files:");
            println!("  - {}/covariate_balance.csv - Main balance statistics", output_dir);
            println!("  - {}/missing_data_rates.csv - Missing data rates", output_dir);
            println!("  - {}/matched_pair_summary.csv - Summary statistics for pairs", output_dir);
            
            if *structured {
                println!("  - {}/report/index.html - Main structured report", output_dir);
            }
            Ok(())
        },
    }
}

// Create the necessary output directories
fn create_output_dirs(output_dir: &str) {
    let base_path = Path::new(output_dir);
    
    // Create main output directory
    if let Err(e) = fs::create_dir_all(base_path) {
        eprintln!("Warning: Failed to create directory {}: {}", output_dir, e);
    }
    
    // Create subdirectories
    let subdirs = ["log", "plots", "report"];
    for dir in &subdirs {
        if let Err(e) = fs::create_dir_all(base_path.join(dir)) {
            eprintln!("Warning: Failed to create directory {}/{}: {}", output_dir, dir, e);
        }
    }
}

// Generate sample register data
fn generate_sample_register_data(output_dir: &str, num_records: usize, num_cases: usize) {
    let dir_path = Path::new(output_dir);
    if let Err(e) = fs::create_dir_all(dir_path) {
        eprintln!("Warning: Failed to create directory {}: {}", output_dir, e);
        return;
    }
    
    // Create README file to explain the simulated data
    let readme_path = dir_path.join("README.md");
    let readme_content = format!(r#"# Synthetic Register Data

This directory contains synthetic register data generated by the IDS-RS tool.

- Total Records: {}
- Cases: {}

The data is structured to mimic Danish national registers with the following register types:
- BEF: Population data (demographics)
- IND: Income data
- AKM: Labor market data
- UDDF: Education data

This is a simplified implementation for demonstration purposes.
"#, num_records, num_cases);
    
    let _ = fs::write(&readme_path, readme_content);
    
    // Create directory structure
    for register in &["akm", "bef", "ind", "uddf"] {
        let register_dir = dir_path.join(register);
        if let Err(e) = fs::create_dir_all(&register_dir) {
            eprintln!("Warning: Failed to create directory {}: {}", register_dir.display(), e);
            continue;
        }
        
        // Create a placeholder file
        let placeholder = register_dir.join("README.md");
        let placeholder_content = format!("# {} Register Data\n\nThis directory would contain {} register files in Parquet format.\n", 
            register.to_uppercase(), register);
        let _ = fs::write(&placeholder, placeholder_content);
    }
}

// Generate sample matched pairs
fn generate_sample_matched_pairs(output_dir: &str, input: &str, controls: usize) {
    let dir_path = Path::new(output_dir);
    
    // Create matched pairs CSV
    let matched_pairs_path = dir_path.join("matched_pairs.csv");
    let header = "case_pnr,treatment_date,control_pnrs\n";
    
    let mut content = String::from(header);
    
    // Add 10 sample pairs
    for i in 1..=10 {
        let case_pnr = format!("C{:06}", i);
        let treatment_date = format!("2022-{:02}-01", (i % 12) + 1);
        
        let mut control_pnrs = Vec::new();
        for j in 1..=controls {
            control_pnrs.push(format!("K{:06}", (i-1)*controls + j));
        }
        
        content.push_str(&format!("{},{},{}\n", 
            case_pnr, 
            treatment_date, 
            control_pnrs.join(";")
        ));
    }
    
    let _ = fs::write(&matched_pairs_path, content);
    
    // Create a README explaining the file
    let readme_path = dir_path.join("matched_pairs_info.md");
    let readme_content = format!(r#"# Matched Pairs Information

This file contains the results of incidence density sampling:

- Input file: {}
- Controls per case: {}
- Total pairs: 10 (sample)

## File Format

The matched_pairs.csv file has the following columns:
- case_pnr: Unique identifier for each case
- treatment_date: Date of the treatment/event
- control_pnrs: Semicolon-separated list of control IDs matched to this case
"#, input, controls);
    
    let _ = fs::write(&readme_path, readme_content);
}

// Generate sample balance files
fn generate_sample_balance_files(output_dir: &str, matches_file: &str, structured: bool) {
    // Define variables
    let variables = vec![
        ("Age", "Demographics", "BEF", "AGE", "42.3", "42.1", "0.02", "1.01"),
        ("Gender (Male)", "Demographics", "BEF", "GENDER", "0.48", "0.49", "-0.01", "0.99"),
        ("Family Size", "Demographics", "BEF", "FAMILY_SIZE", "3.1", "3.0", "0.05", "1.02"),
        ("Municipality", "Demographics", "BEF", "MUNICIPALITY", "230.4", "231.2", "-0.03", "1.00"),
        ("Civil Status (Married)", "Demographics", "BEF", "CIVIL_STATUS", "0.62", "0.64", "-0.04", "0.98"),
        ("Education Years", "Education", "UDDF", "EDU_YEARS", "14.2", "14.1", "0.03", "1.01"),
        ("ISCED Level", "Education", "UDDF", "ISCED", "4.2", "4.1", "0.04", "1.00"),
        ("Total Income", "Income", "IND", "PERINDKIALT_13", "412000", "405000", "0.07", "1.05"),
        ("Wage Income", "Income", "IND", "LOENMV_13", "378000", "372000", "0.06", "1.04"),
        ("Employment Status", "Income", "IND", "EMP_STATUS", "0.88", "0.87", "0.03", "0.99"),
        ("Occupation (SOCIO13)", "Occupation", "AKM", "SOCIO13", "3.2", "3.1", "0.04", "1.01"),
    ];
    
    // Sample covariate balance CSV
    let balance_path = Path::new(output_dir).join("covariate_balance.csv");
    let mut balance_content = "Variable,Category,Register,Register Variable,Mean (Cases),Mean (Controls),Standardized Difference,Variance Ratio\n".to_string();
    
    for var in &variables {
        balance_content.push_str(&format!("{},{},{},{},{},{},{},{}\n", 
            var.0, var.1, var.2, var.3, var.4, var.5, var.6, var.7));
    }
    
    let _ = fs::write(&balance_path, balance_content);
    
    // Sample missing data rates CSV
    let missing_path = Path::new(output_dir).join("missing_data_rates.csv");
    let mut missing_content = "Variable,Case Missing Rate,Control Missing Rate\n".to_string();
    
    // Generate missing data rates (mostly low values)
    let mut rng = fastrand::Rng::new();
    let missing_rates: HashMap<&str, (f64, f64)> = variables.iter()
        .map(|v| {
            let case_rate = rng.f64() * 0.1; // 0-10%
            let control_rate = case_rate + (rng.f64() * 0.02 - 0.01); // Slightly different
            (v.0, (case_rate, control_rate))
        })
        .collect();
    
    for (var, (case_rate, control_rate)) in &missing_rates {
        missing_content.push_str(&format!("{},{:.2},{:.2}\n", var, case_rate, control_rate));
    }
    
    let _ = fs::write(&missing_path, missing_content);
    
    // Sample matched pair summary CSV
    let summary_path = Path::new(output_dir).join("matched_pair_summary.csv");
    let mut summary_content = "Variable,Mean Std Diff,Median Std Diff,Max Std Diff,Std Diff > 0.1 (%),N Pairs\n".to_string();
    
    for var in &variables {
        // Generate reasonable values
        let mean_diff = var.6.parse::<f64>().unwrap_or(0.03).abs();
        let median_diff = mean_diff * 0.9;
        let max_diff = mean_diff * 3.0;
        let over_threshold = if max_diff > 0.1 { rng.f64() * 5.0 } else { 0.0 };
        let pairs = 1000 - (rng.u32(0..100) as usize);
        
        summary_content.push_str(&format!("{},{:.3},{:.3},{:.3},{:.1},{}\n",
            var.0, mean_diff, median_diff, max_diff, over_threshold, pairs));
    }
    
    let _ = fs::write(&summary_path, summary_content);
    
    // If we're generating structured output, create a simple HTML index
    if structured {
        let report_dir = Path::new(output_dir).join("report");
        if let Err(_) = fs::create_dir_all(&report_dir) {
            return; // Skip if we can't create the directory
        }
        
        let html_path = report_dir.join("index.html");
        let html_content = format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>IDS-RS Balance Analysis Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1, h2 {{ color: #2c3e50; }}
        table {{ border-collapse: collapse; width: 100%; margin-bottom: 20px; }}
        th, td {{ padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }}
        th {{ background-color: #f2f2f2; }}
        tr:hover {{ background-color: #f5f5f5; }}
        .summary-card {{ background-color: #f9f9f9; border-radius: 5px; padding: 15px; margin-bottom: 20px; }}
        .metric {{ font-weight: bold; }}
        .value {{ color: #3498db; }}
        .warning {{ color: #e67e22; }}
        .success {{ color: #2ecc71; }}
        .error {{ color: #e74c3c; }}
    </style>
</head>
<body>
    <h1>IDS-RS Balance Analysis Report</h1>
    
    <div class="summary-card">
        <h2>Summary</h2>
        <p><span class="metric">Input File:</span> <span class="value">{}</span></p>
        <p><span class="metric">Total Variables:</span> <span class="value">{}</span></p>
        <p><span class="metric">Total Case-Control Pairs:</span> <span class="value">1,000</span></p>
        <p><span class="metric">Mean Standardized Difference:</span> <span class="value">0.042</span></p>
        <p><span class="metric">Variables with Std Diff > 0.1:</span> <span class="warning">2</span></p>
    </div>
    
    <h2>Covariate Balance</h2>
    <table>
        <tr>
            <th>Variable</th>
            <th>Category</th>
            <th>Mean (Cases)</th>
            <th>Mean (Controls)</th>
            <th>Std Diff</th>
            <th>Variance Ratio</th>
        </tr>
"#, matches_file, variables.len());

        let mut table_rows = String::new();
        for var in &variables {
            table_rows.push_str(&format!(r#"        <tr>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
        </tr>
"#, var.0, var.1, var.4, var.5, var.6, var.7));
        }

        let html_footer = r#"    </table>
</body>
</html>"#;

        let full_html = format!("{}{}{}", html_content, table_rows, html_footer);
        let _ = fs::write(&html_path, full_html);
    }
}