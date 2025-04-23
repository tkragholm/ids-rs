//! Consolidated date handling utilities.
//!
//! This module provides various functions and traits for working with dates,
//! including parsing, formatting, manipulation, and calculation of various date
//! properties like age, quarter, etc.
//!
//! It consolidates functionality previously scattered across multiple modules
//! into a single, comprehensive implementation that can be used throughout the
//! codebase.

pub mod core;
pub mod formatting;
pub mod parsing;
pub mod periods;

pub use core::*;
pub use formatting::*;
pub use parsing::*;
pub use periods::*;

// Re-export core types and traits for convenience
pub use crate::error::Result;
pub use chrono::{Datelike, Duration, NaiveDate};
