use ids_rs::cli::commands::Cli;
use ids_rs::utils::logging::{Component, init_logging};
use ids_rs::info_log;
use log::LevelFilter;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() {
    // Initialize our structured logging system
    if let Err(e) = init_logging(LevelFilter::Debug) {
        eprintln!("Failed to initialize logging: {e}");
        std::process::exit(1);
    }
    
    // Log application startup
    info_log!(Component::Core, "startup", "IDS-RS application starting");
    
    // Run the CLI application
    if let Err(e) = Cli::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
