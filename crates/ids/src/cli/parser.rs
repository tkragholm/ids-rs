use clap::Parser;
use std::process;

use crate::cli::types::Cli;

/// Parse command line arguments and handle common errors
///
/// This function handles parsing CLI arguments with special
/// error handling for common mistakes like missing spaces between
/// options and values.
///
/// # Returns
/// * `Cli` - The parsed command line arguments
///
/// # Panics
/// * If command line parsing fails (with helpful error messages)
#[must_use] pub fn parse_cli_args() -> Cli {
    // Check for the most common command line mistake - missing space after --family-file
    for arg in std::env::args() {
        if arg.starts_with("--family-file") && arg != "--family-file" {
            eprintln!(
                "ERROR: Detected possible command line issue. You provided '{arg}' without a space."
            );
            eprintln!("       Did you mean to write: --family-file {}", &arg[13..]);
            eprintln!(
                "       Check other parameters too. Put a space between each flag and its value."
            );
            process::exit(1);
        }
    }

    // Parse command line arguments
    match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{e}");
            eprintln!("\nNOTE: Make sure there is a space between each flag and its value!");
            eprintln!("Example: --family-file data/registers/family.parquet");
            process::exit(1);
        }
    }
}
