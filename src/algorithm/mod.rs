//! Algorithms for the IDS-RS library
//!
//! This module contains the algorithms for sampling, matching, balance checking,
//! population data generation, and health data processing.

// Core algorithms that are not specific to population or health data
pub mod matching;
pub mod balance;
pub mod statistics;
pub mod sampler;

// Population submodule
pub mod population;

// Health data submodule
pub mod health;

// Re-export for backward compatibility
// These will be deprecated in a future version
#[doc(hidden)]
pub use population::core as population_deprecated;
#[doc(hidden)]
pub use population::integration as population_integration;
#[doc(hidden)]
pub use population::classification as population_scd;
#[doc(hidden)]
pub use health::lpr as lpr;
#[doc(hidden)]
pub use health::diagnosis::scd as scd;
#[doc(hidden)]
pub use health::diagnosis::secondary as secondary_diagnosis;