//! Date utilities module for handling date operations.
//!
//! This module provides functions and traits for working with dates including
//! parsing, formatting, manipulation, and calculations such as age, quarter, etc.

mod core;
mod formatting;
mod parsing;
mod arrow;

pub use self::core::*;
pub use self::formatting::*;
pub use self::parsing::*;
pub use self::arrow::*;

// Re-export core types and traits for convenience
pub use chrono::{Datelike, Duration, NaiveDate};
