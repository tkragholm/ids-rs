//! Consolidated string manipulation utilities.
//!
//! This module provides various functions and traits for working with strings,
//! including case conversion, parsing, formatting, truncation, and sanitization.
//!
//! It consolidates functionality previously scattered across multiple modules
//! into a single, comprehensive implementation that can be used throughout the
//! codebase.

mod case_conversion;
mod parsing;
mod formatting;

pub use case_conversion::*;
pub use parsing::*;
pub use formatting::*;

// Re-export core types and traits for convenience
pub use crate::error::Result;