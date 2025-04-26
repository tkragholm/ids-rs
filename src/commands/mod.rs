//! Command handlers for the IDS-RS CLI
//!
//! This module contains handlers for various CLI commands

pub mod population;
pub mod scd;
pub mod population_scd;

// Re-export common command handlers
pub use population::handle_population_command;
pub use scd::handle_scd_command;
pub use population_scd::handle_population_scd_command;