//! Population SCD command implementation
//!
//! This module provides the implementation for the Population SCD command, which
//! identifies children in a population who have been diagnosed with severe chronic diseases.

pub mod config;
pub mod handler;

pub use config::PopulationScdCommandConfig;
pub use handler::handle_population_scd_command;