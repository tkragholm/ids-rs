use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input CSV file path
    #[arg(short, long, default_value = "data.csv")]
    pub input: String,

    /// Number of controls to match per case
    #[arg(short, long, default_value_t = 4)]
    pub controls: usize,

    /// Birth date matching window in days
    #[arg(short, long, default_value_t = 30)]
    pub birth_window: i64,

    /// Parent age matching window in days
    #[arg(short, long, default_value_t = 365)]
    pub parent_window: i64,

    /// Output directory for results
    #[arg(short, long, default_value = "output")]
    pub output_dir: String,

    /// Generate synthetic data
    #[arg(short, long)]
    pub generate: bool,

    /// Number of records to generate
    #[arg(short, long, default_value_t = 1_200_000)]
    pub num_records: usize,

    /// Number of treatment cases to generate
    #[arg(short, long, default_value_t = 50_000)]
    pub num_cases: usize,
}
