//! Population data processing algorithms
//!
//! This module implements algorithms for population data generation, classification,
//! and integration with additional registers.

pub mod core;
pub mod integration;
pub mod classification;

// Re-export common types
pub use core::{PopulationConfig, PopulationSummary, generate_population};
pub use integration::integrate_population_data;
pub use classification::{PopulationScdConfig, PopulationScdResult};
