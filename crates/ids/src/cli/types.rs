use clap::{Parser, Subcommand};

/// Subcommands for the Config command
///
/// These commands handle configuration-related tasks like generating
/// default configuration files for covariates.
#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    /// Generate a default configuration file for covariates
    GenerateCovariates {
        /// Output file path
        #[arg(
            short = 'o',
            long,
            help = "Path to save the generated configuration file"
        )]
        output: String,

        /// Force overwrite of existing file
        #[arg(
            short = 'f',
            long,
            help = "Force overwrite if the output file already exists"
        )]
        force: bool,
    },
}

/// Main CLI configuration for the IDS tool
///
/// This struct defines the top-level command-line interface for the IDS (Incidence Density Sampling)
/// tool. It handles global options that apply to all subcommands, such as the output directory.
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Incidence Density Sampling (IDS) CLI tool for epidemiological research",
    long_about = "A comprehensive toolkit for generating synthetic register data, performing incidence density sampling, and analyzing covariate balance in epidemiological studies."
)]
pub struct Cli {
    /// Output directory for results
    #[arg(
        short = 'o',
        long,
        default_value = "output",
        help = "Directory where all results will be saved"
    )]
    pub output_dir: String,

    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands for the IDS tool
///
/// These commands represent the main functionality of the IDS toolkit,
/// including data generation, sampling, and balance analysis.
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Generate synthetic register data for research purposes
    GenerateRegisters {
        /// Directory for register data output
        #[arg(
            short = 'o',
            long,
            default_value = "data/registers",
            help = "Directory where generated register data will be saved"
        )]
        output_dir: String,

        /// Number of total records to generate
        #[arg(
            short = 't',
            long,
            default_value_t = 1_000_000,
            help = "Total number of records to generate across all registers"
        )]
        num_records: usize,

        /// Number of treatment cases to generate
        #[arg(
            short = 'c',
            long,
            default_value_t = 50_000,
            help = "Number of cases with treatment events (must be less than total records)"
        )]
        num_cases: usize,

        /// Start year for data generation
        #[arg(
            short = 's',
            long,
            default_value_t = 2000,
            help = "Start year for the generated data range (min: 1980)"
        )]
        start_year: i32,

        /// End year for data generation
        #[arg(
            short = 'e',
            long,
            default_value_t = 2023,
            help = "End year for the generated data range (max: 2023)"
        )]
        end_year: i32,

        /// Random seed for reproducibility
        #[arg(
            short = 'r',
            long,
            help = "Seed for random number generation to ensure reproducible results"
        )]
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
        #[arg(
            short = 'i',
            long,
            default_value = "data/pediatric.csv",
            help = "CSV file containing cases with treatment dates and demographic information"
        )]
        input: String,

        /// Number of controls to match per case
        #[arg(
            short = 'n',
            long,
            default_value_t = 4,
            help = "Number of control subjects to match with each case"
        )]
        controls: usize,

        /// Birth date matching window in days
        #[arg(
            short = 'b',
            long,
            default_value_t = 30,
            help = "Maximum allowed difference between case and control birth dates (in days)"
        )]
        birth_window: i64,

        /// Parent age matching window in days
        #[arg(
            short = 'p',
            long,
            default_value_t = 365,
            help = "Maximum allowed difference between case and control parent ages (in days)"
        )]
        parent_window: i64,
    },

    /// Analyze covariate balance between matched cases and controls
    CheckBalance {
        /// Path to the matched pairs CSV file
        #[arg(
            short = 'm',
            long,
            help = "CSV file containing the matched case-control pairs from sampling",
            required = true
        )]
        matches_file: String,

        /// Base directory containing the register data files with covariates
        #[arg(
            short = 'c',
            long,
            help = "Base directory containing register data. Can be omitted if all custom paths are specified. Expected structure is a directory containing 'family.parquet' and/or subdirectories 'akm', 'bef', 'ind', 'uddf'"
        )]
        covariate_dir: Option<String>,

        /// Path to the family.parquet file
        #[arg(
            long,
            value_name = "FILE",
            help = "Path to the family relationships file (family.parquet). This should be a direct path to the parquet file, NOT a directory. This path is handled independently from the covariate directory."
        )]
        family_file: Option<String>,

        /// Path to the AKM register directory
        #[arg(
            long,
            value_name = "DIR",
            help = "Path to the directory containing AKM register files (named like '2000.parquet', '2001.parquet', etc.). Either absolute or relative paths are supported."
        )]
        akm_dir: Option<String>,

        /// Path to the BEF register directory
        #[arg(
            long,
            value_name = "DIR",
            help = "Path to the directory containing BEF register files (named like '200012.parquet', '201903.parquet', etc.). Either absolute or relative paths are supported."
        )]
        bef_dir: Option<String>,

        /// Path to the IND register directory
        #[arg(
            long,
            value_name = "DIR",
            help = "Path to the directory containing IND register files (named like '2000.parquet', '2001.parquet', etc.). Either absolute or relative paths are supported."
        )]
        ind_dir: Option<String>,

        /// Path to the UDDF register directory
        #[arg(
            long,
            value_name = "DIR",
            help = "Path to the directory containing UDDF register files (named like '202009.parquet', '202209.parquet', etc.). Either absolute or relative paths are supported."
        )]
        uddf_dir: Option<String>,

        /// Generate structured HTML reports and organized outputs
        #[arg(
            long,
            help = "Generate structured HTML reports and organized output files"
        )]
        structured: bool,
    },
}
