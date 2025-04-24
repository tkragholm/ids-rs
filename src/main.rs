use ids_rs::cli::commands::Cli;
use ids_rs::core::logging;

fn main() {
    // Initialize logging
    if let Err(e) = logging::init_default_logging() {
        eprintln!("Failed to initialize logging: {e}");
    }
    
    // Run the CLI application
    if let Err(e) = Cli::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}