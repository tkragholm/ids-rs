use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
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

        /// Directory containing the register data files with covariates
        #[arg(short = 'c', long, help = "Directory containing parquet files with covariate data for analysis", required = true)]
        covariate_dir: String,
    },
}
