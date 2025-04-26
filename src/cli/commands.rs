use crate::cli::console::Console;
use crate::error::Result;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;

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
        Console::print_info(&format!(
            "Loaded {} case record batches",
            case_records.len()
        ));

        Console::print_info("Loading control data...");
        let control_records = crate::algorithm::balance::load_records(&self.control_path)?;
        Console::print_info(&format!(
            "Loaded {} control record batches",
            control_records.len()
        ));

        // Calculate balance metrics
        Console::print_info("Calculating balance metrics...");
        let balance_report =
            crate::algorithm::balance::calculate_balance(&case_records, &control_records)?;

        // Generate report
        Console::print_info(&format!("Generating report at {}", self.report_path));
        crate::algorithm::balance::generate_balance_report(&self.report_path, &balance_report)?;

        // Print summary
        Console::print_info("Balance Check Summary:");
        Console::print_key_value(
            "Total Covariates",
            &balance_report.summary.total_covariates.to_string(),
        );
        Console::print_key_value(
            "Imbalanced Covariates",
            &balance_report.summary.imbalanced_covariates.to_string(),
        );
        Console::print_key_value(
            "Max Standardized Difference",
            &format!("{:.4}", balance_report.summary.max_standardized_difference),
        );
        Console::print_key_value(
            "Mean Absolute Standardized Difference",
            &format!(
                "{:.4}",
                balance_report.summary.mean_absolute_standardized_difference
            ),
        );

        Console::print_success("Balance check completed");
        Ok(())
    }
}

/// Population command handler
pub struct PopulationCommand {
    /// BEF data path
    pub bef_path: PathBuf,

    /// MFR data path
    pub mfr_path: PathBuf,

    /// Output directory
    pub output_dir: PathBuf,

    /// Start year for birth inclusion
    pub birth_start_year: i32,

    /// End year for birth inclusion
    pub birth_end_year: i32,
}

impl CommandHandler for PopulationCommand {
    fn execute(&self) -> Result<()> {
        Console::print_header("Generating Population Data");
        Console::print_key_value("BEF Data", &self.bef_path.display().to_string());
        Console::print_key_value("MFR Data", &self.mfr_path.display().to_string());
        Console::print_key_value("Output Directory", &self.output_dir.display().to_string());
        Console::print_key_value(
            "Birth Year Range",
            &format!("{} - {}", self.birth_start_year, self.birth_end_year),
        );

        // Create config from CLI arguments
        let config = crate::commands::population::PopulationCommandConfig {
            bef_path: self.bef_path.clone(),
            mfr_path: self.mfr_path.clone(),
            output_dir: self.output_dir.clone(),
            birth_inclusion_start_year: self.birth_start_year,
            birth_inclusion_end_year: self.birth_end_year,
        };

        // Execute the population generation
        crate::commands::population::handle_population_command(&config)?;

        Console::print_success("Population generation completed");
        Ok(())
    }
}

/// SCD command handler
pub struct ScdCommand {
    /// LPR data path
    pub lpr_path: PathBuf,

    /// Output directory
    pub output_path: PathBuf,

    /// Include LPR2 data
    pub include_lpr2: bool,

    /// Include LPR3 data
    pub include_lpr3: bool,

    /// Start date for filtering
    pub start_date: Option<chrono::NaiveDate>,

    /// End date for filtering
    pub end_date: Option<chrono::NaiveDate>,
}

impl CommandHandler for ScdCommand {
    fn execute(&self) -> Result<()> {
        Console::print_header("Analyzing LPR Data for Severe Chronic Disease");
        Console::print_key_value("LPR Data", &self.lpr_path.display().to_string());
        Console::print_key_value("Output Directory", &self.output_path.display().to_string());
        Console::print_key_value("Include LPR2", &self.include_lpr2.to_string());
        Console::print_key_value("Include LPR3", &self.include_lpr3.to_string());
        
        if let Some(date) = self.start_date {
            Console::print_key_value("Start Date", &date.to_string());
        }
        
        if let Some(date) = self.end_date {
            Console::print_key_value("End Date", &date.to_string());
        }

        // Create config from CLI arguments
        let config = crate::commands::scd::ScdCommandConfig {
            lpr_data_path: self.lpr_path.clone(),
            output_path: self.output_path.clone(),
            include_lpr2: self.include_lpr2,
            include_lpr3: self.include_lpr3,
            start_date: self.start_date,
            end_date: self.end_date,
            diagnosis_columns: vec![
                "primary_diagnosis".to_string(),
                "secondary_diagnosis".to_string(),
            ],
            patient_id_column: "patient_id".to_string(),
            date_column: "admission_date".to_string(),
        };

        // Execute the SCD analysis
        crate::commands::scd::handle_scd_command(&config)?;

        Console::print_success("SCD analysis completed");
        Ok(())
    }
}

/// Population SCD command handler
pub struct PopulationScdCommand {
    /// Population data path
    pub population_path: PathBuf,
    
    /// LPR data path
    pub lpr_path: PathBuf,

    /// Output directory
    pub output_dir: PathBuf,

    /// Include LPR2 data
    pub include_lpr2: bool,

    /// Include LPR3 data
    pub include_lpr3: bool,

    /// Start date for filtering
    pub start_date: Option<chrono::NaiveDate>,

    /// End date for filtering
    pub end_date: Option<chrono::NaiveDate>,
}

impl CommandHandler for PopulationScdCommand {
    fn execute(&self) -> Result<()> {
        Console::print_header("Identifying Children with Severe Chronic Disease in Population");
        Console::print_key_value("Population Data", &self.population_path.display().to_string());
        Console::print_key_value("LPR Data", &self.lpr_path.display().to_string());
        Console::print_key_value("Output Directory", &self.output_dir.display().to_string());
        Console::print_key_value("Include LPR2", &self.include_lpr2.to_string());
        Console::print_key_value("Include LPR3", &self.include_lpr3.to_string());
        
        if let Some(date) = self.start_date {
            Console::print_key_value("Start Date", &date.to_string());
        }
        
        if let Some(date) = self.end_date {
            Console::print_key_value("End Date", &date.to_string());
        }

        // Create config from CLI arguments
        let config = crate::commands::population_scd::PopulationScdCommandConfig {
            population_path: self.population_path.clone(),
            lpr_data_path: self.lpr_path.clone(),
            output_dir: self.output_dir.clone(),
            include_lpr2: self.include_lpr2,
            include_lpr3: self.include_lpr3,
            start_date: self.start_date,
            end_date: self.end_date,
        };

        // Execute the Population SCD analysis
        crate::commands::population_scd::handle_population_scd_command(&config)?;

        Console::print_success("Population SCD analysis completed");
        Ok(())
    }
}

/// CLI Parser for the IDS-RS application
#[derive(Parser)]
#[clap(version, about = "Integrated Data System for Research in Rust")]
pub struct Cli {
    /// Control verbosity of the output
    #[clap(flatten)]
    pub verbose: Verbosity,

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

    /// Generate population data by combining BEF and MFR registers
    Population(PopulationArgs),
    
    /// Analyze LPR data for Severe Chronic Disease (SCD)
    Scd(ScdArgs),
    
    /// Identify children in a population with Severe Chronic Disease (SCD)
    PopulationScd(PopulationScdArgs),
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

/// Arguments for the population command
#[derive(Args)]
struct PopulationArgs {
    /// BEF data path (supports glob patterns like "*.parquet")
    #[clap(short, long)]
    bef: PathBuf,

    /// MFR data path (supports glob patterns like "*.parquet")
    #[clap(short, long)]
    mfr: PathBuf,

    /// Output directory for population data and reports
    #[clap(short, long)]
    output: PathBuf,

    /// Start year for filtering births (inclusive)
    #[clap(long, default_value = "1995")]
    start_year: i32,

    /// End year for filtering births (inclusive)
    #[clap(long, default_value = "2018")]
    end_year: i32,
}

/// Arguments for the SCD command
#[derive(Args)]
struct ScdArgs {
    /// LPR data directory (should contain LPR2 and/or LPR3 data)
    #[clap(short, long)]
    lpr: PathBuf,

    /// Output directory for SCD results and reports
    #[clap(short, long)]
    output: PathBuf,

    /// Include LPR2 data
    #[clap(long, default_value = "true")]
    include_lpr2: bool,

    /// Include LPR3 data
    #[clap(long, default_value = "true")]
    include_lpr3: bool,

    /// Start date for filtering LPR data (format: YYYY-MM-DD)
    #[clap(long)]
    start_date: Option<String>,

    /// End date for filtering LPR data (format: YYYY-MM-DD)
    #[clap(long)]
    end_date: Option<String>,
}

/// Arguments for the Population SCD command
#[derive(Args)]
struct PopulationScdArgs {
    /// Path to the population data file (generated with the 'population' command)
    #[clap(short, long)]
    population: PathBuf,
    
    /// LPR data directory (should contain LPR2 and/or LPR3 data)
    #[clap(short, long)]
    lpr: PathBuf,

    /// Output directory for population SCD results and reports
    #[clap(short, long)]
    output: PathBuf,

    /// Include LPR2 data
    #[clap(long, default_value = "true")]
    include_lpr2: bool,

    /// Include LPR3 data
    #[clap(long, default_value = "true")]
    include_lpr3: bool,

    /// Start date for filtering LPR data (format: YYYY-MM-DD)
    #[clap(long)]
    start_date: Option<String>,

    /// End date for filtering LPR data (format: YYYY-MM-DD)
    #[clap(long)]
    end_date: Option<String>,
}

impl Cli {
    /// Parse command-line arguments and execute the appropriate command
    pub fn run() -> Result<()> {
        let cli = Self::parse();

        // Initialize logger with verbosity from CLI
        let log_level = cli.verbose.log_level_filter();
        env_logger::Builder::new()
            .filter_level(log_level)
            .init();

        log::debug!("Log level set to: {log_level}");

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
            Commands::Population(args) => {
                let command = PopulationCommand {
                    bef_path: args.bef,
                    mfr_path: args.mfr,
                    output_dir: args.output,
                    birth_start_year: args.start_year,
                    birth_end_year: args.end_year,
                };
                command.execute()
            }
            Commands::Scd(args) => {
                // Parse start and end dates if provided
                let start_date = args.start_date.map(|date_str| {
                    chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                        .unwrap_or_else(|_| panic!("Invalid start date format. Expected YYYY-MM-DD, got {date_str}"))
                });
                
                let end_date = args.end_date.map(|date_str| {
                    chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                        .unwrap_or_else(|_| panic!("Invalid end date format. Expected YYYY-MM-DD, got {date_str}"))
                });
                
                let command = ScdCommand {
                    lpr_path: args.lpr,
                    output_path: args.output,
                    include_lpr2: args.include_lpr2,
                    include_lpr3: args.include_lpr3,
                    start_date,
                    end_date,
                };
                command.execute()
            }
            Commands::PopulationScd(args) => {
                // Parse start and end dates if provided
                let start_date = args.start_date.map(|date_str| {
                    chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                        .unwrap_or_else(|_| panic!("Invalid start date format. Expected YYYY-MM-DD, got {date_str}"))
                });
                
                let end_date = args.end_date.map(|date_str| {
                    chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                        .unwrap_or_else(|_| panic!("Invalid end date format. Expected YYYY-MM-DD, got {date_str}"))
                });
                
                let command = PopulationScdCommand {
                    population_path: args.population,
                    lpr_path: args.lpr,
                    output_dir: args.output,
                    include_lpr2: args.include_lpr2,
                    include_lpr3: args.include_lpr3,
                    start_date,
                    end_date,
                };
                command.execute()
            }
        }
    }
}
