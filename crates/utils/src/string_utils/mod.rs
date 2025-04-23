//! Consolidated string manipulation utilities.
//!
//! This module provides various functions and traits for working with strings,
//! including case conversion, parsing, formatting, truncation, and sanitization.
//!
//! It consolidates functionality previously scattered across multiple modules
//! into a single, comprehensive implementation that can be used throughout the
//! codebase.

pub mod case_conversion;
pub mod formatting;
pub mod parsing;

pub use case_conversion::*;
pub use formatting::*;
pub use parsing::*;

// Re-export core types and traits for convenience
pub use crate::error::Result;
