use crate::cli::console::Console;
use crate::error::Result;
use clap::{Parser, Subcommand, Args};

/// Command handler trait
pub trait CommandHandler {
    /// Execute the command
    fn execute(&self) -> Result<()>;
}

/// Sample command handler
pub struct SampleCommand {
    /// Input file path
    pub input_path: String,

    /// Output file path
    pub output_path: String,

    /// Number of samples
    pub sample_count: usize,
}

impl CommandHandler for SampleCommand {
    fn execute(&self) -> Result<()> {
        Console::print_header("Sampling Data");
        Console::print_key_value("Input", &self.input_path);
        Console::print_key_value("Output", &self.output_path);
        Console::print_key_value("Samples", &self.sample_count.to_string());

        // Load data from the specified registry
        Console::print_info("Loading registry data...");
        let registry = crate::registry::registry_from_path(&self.input_path)?;
        let records = registry.load(&self.input_path, None)?;
        
        Console::print_info(&format!("Loaded {} record batches", records.len()));
        
        // Sample from the loaded data
        Console::print_info(&format!("Sampling {} records...", self.sample_count));
        let sampled = crate::core::sampler::sample_records(&records, self.sample_count, None)?;
        
        // Save to the output path
        Console::print_info(&format!("Writing sampled data to {}", self.output_path));
        crate::core::sampler::write_parquet(&self.output_path, &sampled)?;
        
        Console::print_success("Sampling completed");
        Ok(())
    }
}

/// Balance command handler
pub struct BalanceCommand {
    /// Case file path
    pub case_path: String,

    /// Control file path
    pub control_path: String,

    /// Report file path
    pub report_path: String,
}

impl CommandHandler for BalanceCommand {
    fn execute(&self) -> Result<()> {
        Console::print_header("Checking Balance");
        Console::print_key_value("Cases", &self.case_path);
        Console::print_key_value("Controls", &self.control_path);
        Console::print_key_value("Report", &self.report_path);

        // Load case and control data
        Console::print_info("Loading case data...");
        let case_records = crate::algorithm::balance::load_records(&self.case_path)?;
        Console::print_info(&format!("Loaded {} case record batches", case_records.len()));
        
        Console::print_info("Loading control data...");
        let control_records = crate::algorithm::balance::load_records(&self.control_path)?;
        Console::print_info(&format!("Loaded {} control record batches", control_records.len()));
        
        // Calculate balance metrics
        Console::print_info("Calculating balance metrics...");
        let balance_report = crate::algorithm::balance::calculate_balance(&case_records, &control_records)?;
        
        // Generate report
        Console::print_info(&format!("Generating report at {}", self.report_path));
        crate::algorithm::balance::generate_balance_report(&self.report_path, &balance_report)?;
        
        // Print summary
        Console::print_info("Balance Check Summary:");
        Console::print_key_value("Total Covariates", &balance_report.summary.total_covariates.to_string());
        Console::print_key_value("Imbalanced Covariates", &balance_report.summary.imbalanced_covariates.to_string());
        Console::print_key_value("Max Standardized Difference", &format!("{:.4}", balance_report.summary.max_standardized_difference));
        Console::print_key_value("Mean Absolute Standardized Difference", &format!("{:.4}", balance_report.summary.mean_absolute_standardized_difference));
        
        Console::print_success("Balance check completed");
        Ok(())
    }
}

/// CLI Parser for the IDS-RS application
#[derive(Parser)]
#[clap(version, about = "Integrated Data System for Research in Rust")]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Available commands
#[derive(Subcommand)]
enum Commands {
    /// Sample data from a registry
    Sample(SampleArgs),
    
    /// Check balance between case and control groups
    Balance(BalanceArgs),
}

/// Arguments for the sample command
#[derive(Args)]
struct SampleArgs {
    /// Input file path
    #[clap(short, long)]
    input: String,
    
    /// Output file path
    #[clap(short, long)]
    output: String,
    
    /// Number of samples
    #[clap(short, long)]
    count: usize,
}

/// Arguments for the balance command
#[derive(Args)]
struct BalanceArgs {
    /// Case file path
    #[clap(short = 'c', long)]
    cases: String,
    
    /// Control file path
    #[clap(short = 't', long)]
    controls: String,
    
    /// Report file path
    #[clap(short, long)]
    report: String,
}

impl Cli {
    /// Parse command-line arguments and execute the appropriate command
    pub fn run() -> Result<()> {
        let cli = Self::parse();

        match cli.command {
            Commands::Sample(args) => {
                let command = SampleCommand {
                    input_path: args.input,
                    output_path: args.output,
                    sample_count: args.count,
                };
                command.execute()
            }
            Commands::Balance(args) => {
                let command = BalanceCommand {
                    case_path: args.cases,
                    control_path: args.controls,
                    report_path: args.report,
                };
                command.execute()
            }
        }
    }
}