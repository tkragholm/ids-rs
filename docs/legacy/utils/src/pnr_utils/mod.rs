//! Consolidated Personal Identification Number (PNR) utilities.
//!
//! This module provides various functions and types for working with PNRs,
//! including generation, validation, and lookup operations.
//!
//! It consolidates functionality previously scattered across multiple modules
//! into a single, comprehensive implementation that can be used throughout the
//! codebase.

pub mod generation;
pub mod types;
pub mod validation;

pub use generation::*;
pub use types::*;
pub use validation::*;

// Re-export core types and traits for convenience
pub use crate::error::Result;
pub use chrono::{Datelike, Duration, NaiveDate};
