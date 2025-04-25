use ids_rs::cli::commands::Cli;

fn main() {
    // Run the CLI application (logging is initialized in Cli::run)
    if let Err(e) = Cli::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}