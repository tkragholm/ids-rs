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
    /// Generate synthetic test data
    Generate {
        /// Output file path for synthetic data
        #[arg(short = 'o', long, default_value = "generated_data.csv")]
        output: String,

        /// Number of total records to generate
        #[arg(short = 't', long, default_value_t = 1_200_000)]
        num_records: usize,

        /// Number of treatment cases to generate
        #[arg(short = 'c', long, default_value_t = 50_000)]
        num_cases: usize,
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
}
