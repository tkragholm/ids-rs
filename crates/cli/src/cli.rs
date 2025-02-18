use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Incidence Density Sampling (IDS) CLI tool",
    long_about = None
)]
pub struct Cli {
    /// Output directory for results
    #[arg(short = 'o', long, default_value = "output")]
    pub output_dir: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate synthetic pediatric test data
    GeneratePediatric {
        /// Output file path for synthetic data
        #[arg(short = 'o', long, default_value = "pediatric_data.csv")]
        output: String,

        /// Number of total records to generate
        #[arg(short = 't', long, default_value_t = 1_200_000)]
        num_records: usize,

        /// Number of treatment cases to generate
        #[arg(short = 'c', long, default_value_t = 50_000)]
        num_cases: usize,

        /// Random seed for reproducibility
        #[arg(short = 'r', long)]
        seed: Option<u64>,
    },

    /// Generate synthetic register data
    GenerateRegisters {
        /// Directory for register data output
        #[arg(short = 'o', long, default_value = "data/registers")]
        output_dir: String,

        /// Number of total records to generate
        #[arg(short = 't', long, default_value_t = 1_000_000)]
        num_records: usize,

        /// Number of treatment cases to generate
        #[arg(short = 'c', long, default_value_t = 50_000)]
        num_cases: usize,

        /// Start year for data generation
        #[arg(short = 's', long, default_value_t = 2000)]
        start_year: i32,

        /// End year for data generation
        #[arg(short = 'e', long, default_value_t = 2022)]
        end_year: i32,

        /// Random seed for reproducibility
        #[arg(short = 'r', long)]
        seed: Option<u64>,
    },

    /// Sample controls using incidence density sampling
    Sample {
        /// Input CSV file path
        #[arg(short = 'i', long, default_value = "data.csv")]
        input: String,

        /// Number of controls to match per case
        #[arg(short = 'n', long, default_value_t = 4)]
        controls: usize,

        /// Birth date matching window in days
        #[arg(short = 'b', long, default_value_t = 30)]
        birth_window: i64,

        /// Parent age matching window in days
        #[arg(short = 'p', long, default_value_t = 365)]
        parent_window: i64,
    },

    /// Check covariate balance between cases and controls
    CheckBalance {
        /// Path to the matched pairs CSV file
        #[arg(short = 'm', long)]
        matches_file: String,

        /// Directory containing the register data files
        #[arg(short = 'c', long)]
        covariate_dir: String,
    },
}
