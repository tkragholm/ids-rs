mod cli;
mod errors;
mod generate_data;
mod matching_quality;
mod plotting;
mod sampler;
mod utils;

use clap::Parser;
use cli::Cli;
use log::info;
use sampler::IncidenceDensitySampler;
use std::{fs, path::Path, time::Instant};
use utils::{configure_logging, load_records, MatchingCriteria};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Create output directory if it doesn't exist
    fs::create_dir_all(&cli.output_dir)?;

    // Configure logging
    configure_logging();
    let start = Instant::now();

    // Generate synthetic data if requested
    if cli.generate {
        info!("Generating synthetic data...");
        generate_data::generate_pediatric_data(&cli.input, cli.num_records, cli.num_cases)?;
    }

    info!("Reading data from {}...", cli.input);
    let records = load_records(&cli.input)?;
    info!("Data loaded in {:?}", start.elapsed());

    let criteria = MatchingCriteria {
        birth_date_window: cli.birth_window,
        parent_date_window: cli.parent_window,
    };

    info!("Initializing sampler...");
    let sampler_start = Instant::now();
    let sampler = IncidenceDensitySampler::new(records, criteria)?;
    info!("{}", sampler.get_statistics());
    info!("Sampler initialized in {:?}", sampler_start.elapsed());

    info!("Sampling controls...");
    let sampling_start = Instant::now();
    match sampler.sample_controls(cli.controls) {
        Ok(case_control_pairs) => {
            info!(
                "Sampling completed in {:?}. Found {} matches",
                sampling_start.elapsed(),
                case_control_pairs.len()
            );

            let quality = sampler.evaluate_matching_quality(&case_control_pairs);
            info!("{}", quality.format_report());

            // Generate all plots
            let plots_dir = Path::new(&cli.output_dir).join("plots");
            fs::create_dir_all(&plots_dir)?;

            if let Err(e) = quality.generate_summary_plots(&plots_dir.to_string_lossy()) {
                log::error!("Error generating plots: {}", e);
            }

            let matches_path = Path::new(&cli.output_dir).join("matched_pairs.csv");
            if let Err(e) =
                sampler.save_matches_to_csv(&case_control_pairs, &matches_path.to_string_lossy())
            {
                log::error!("Error saving matches to CSV: {}", e);
            }

            let stats_path = Path::new(&cli.output_dir).join("matching_stats.csv");
            if let Err(e) =
                sampler.save_matching_statistics(&case_control_pairs, &stats_path.to_string_lossy())
            {
                log::error!("Error saving matching statistics: {}", e);
            }
        }
        Err(e) => log::error!("Error sampling controls: {}", e),
    }

    info!("Total execution time: {:?}", start.elapsed());
    Ok(())
}
