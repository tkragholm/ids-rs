//! Pruning functionality for optimized file reading
//!
//! This module provides statistical pruning functionality for efficient file access.
//! It allows for filtering files based on statistics about their contents without
//! reading the entire file, significantly improving performance for large datasets.

mod statistics;
mod predicate;
mod provider;

pub use statistics::*;
pub use predicate::*;
pub use provider::*;