mod errors;
mod matching_quality;
mod sampler;
mod utils;

use log::info;
use sampler::IncidenceDensitySampler;
use std::time::Instant;
use utils::{configure_logging, load_records};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_logging();
    let start = Instant::now();

    info!("Reading data...");
    let records = load_records("data.csv")?;
    info!("Data loaded in {:?}", start.elapsed());

    let criteria = utils::MatchingCriteria {
        birth_date_window: 30,
        parent_date_window: 365,
    };

    info!("Initializing sampler...");
    let sampler_start = Instant::now();
    let sampler = IncidenceDensitySampler::new(records, criteria)?;
    info!("{}", sampler.get_statistics());
    info!("Sampler initialized in {:?}", sampler_start.elapsed());

    info!("Sampling controls...");
    let sampling_start = Instant::now();
    match sampler.sample_controls(4) {
        Ok(case_control_pairs) => {
            info!(
                "Sampling completed in {:?}. Found {} matches",
                sampling_start.elapsed(),
                case_control_pairs.len()
            );

            let quality = sampler.evaluate_matching_quality(&case_control_pairs);
            info!("{}", quality.format_report());

            if let Err(e) = quality.plot_all_distributions("matching_distributions") {
                log::error!("Error plotting distributions: {}", e);
            }

            info!("\nSaving results...");

            if let Err(e) = sampler.save_matches_to_csv(&case_control_pairs, "matched_pairs.csv") {
                log::error!("Error saving matches to CSV: {}", e);
            }

            if let Err(e) =
                sampler.save_matching_statistics(&case_control_pairs, "matching_stats.csv")
            {
                log::error!("Error saving matching statistics: {}", e);
            }
        }
        Err(e) => log::error!("Error sampling controls: {}", e),
    }

    info!("Total execution time: {:?}", start.elapsed());
    Ok(())
}
