//! Population generation command
//!
//! This module provides functionality for generating population data by combining
//! BEF and MFR register data.

pub mod config;
pub mod handler;

pub use config::PopulationCommandConfig;
pub use handler::handle_population_command;