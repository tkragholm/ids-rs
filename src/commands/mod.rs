//! Command handlers for the IDS-RS CLI
//!
//! This module contains handlers for various CLI commands

pub mod population;

// Re-export common command handlers
pub use population::handle_population_command;