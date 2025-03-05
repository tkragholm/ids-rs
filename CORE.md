# Rust Crate Refactoring Plan

Based on my analysis of the `ids-rs/crates/core` crate, I've identified several areas for improvement. Here's a comprehensive refactoring plan for each file:

## Overall Project Structure

1. **Rename the crate**: Change from `core` to `ids_core` to avoid confusion with Rust's standard `core` crate.
2. **Restructure modules**: Organize modules more logically with a clearer public API.
3. **Add prelude module**: Create a prelude for common imports.

## 1. Cargo.toml

```toml
[package]
name = "ids_core"
version.workspace = true
edition.workspace = true
description = "Core algorithms for incidence density sampling in epidemiological research"
documentation = "https://docs.rs/ids_core"
repository.workspace = true
license.workspace = true

[dependencies]
# Keep existing dependencies with explicit versions
```

## 2. File Structure Reorganization

```
src/
├── lib.rs             # Main entry point with public exports
├── error.rs           # Error types and handling
├── data/
│   ├── mod.rs         # Data structures module
│   ├── record.rs      # Record type and related utilities
│   └── date_utils.rs  # Date handling utilities
├── sampling/
│   ├── mod.rs         # Sampling module exports
│   ├── sampler.rs     # Core sampling algorithm
│   └── criteria.rs    # Matching criteria definitions
├── quality/
│   ├── mod.rs         # Quality assessment module
│   ├── metrics.rs     # Quality metrics calculation
│   └── reporting.rs   # Report generation
├── visualization/
│   ├── mod.rs         # Visualization module
│   ├── plotter.rs     # Base plotting trait and implementation
│   └── charts.rs      # Specific chart types
└── utils/
    ├── mod.rs         # Utility module
    ├── console.rs     # Console output utilities
    └── logging.rs     # Logging configuration
```

## 3. Specific File Improvements

### lib.rs

```rust
//! IDS Core: Incidence Density Sampling implementation for epidemiological research
//!
//! This crate provides algorithms and utilities for performing
//! incidence density sampling and analyzing the results.

pub mod data;
pub mod error;
pub mod quality;
pub mod sampling;
pub mod utils;
pub mod visualization;

// Public re-exports for convenience
pub use error::{Error, Result};
pub use data::Record;
pub use sampling::Sampler;

/// Prelude module containing most commonly used types
pub mod prelude {
    pub use crate::data::Record;
    pub use crate::error::{Error, Result};
    pub use crate::quality::MatchingQuality;
    pub use crate::sampling::{Criteria, Sampler};
    pub use crate::visualization::Plotter;
}
```

### error.rs

```rust
//! Error handling for the IDS Core library

use std::fmt;
use thiserror::Error;

// Define Error enum with thiserror
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid date: {0}")]
    InvalidDate(String),

    #[error("Sampling error: {0}")]
    Sampling(String),

    #[error("Invalid matching criteria: {0}")]
    InvalidCriteria(String),

    #[error("Plotting error: {0}")]
    Plotting(String),

    #[error("No eligible controls found")]
    NoEligibleControls,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

// Add context trait for error handling
pub trait Context<T, E> {
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: fmt::Display,
        F: FnOnce() -> C;
}

// Improve other error utility functions...
```

### data/record.rs

```rust
//! Record type definition and related functions

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// A record representing an individual in the dataset
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub pnr: String,
    #[serde(with = "crate::data::date_utils::date_format")]
    pub birth_date: NaiveDate,
    #[serde(with = "crate::data::date_utils::optional_date_format")]
    pub treatment_date: Option<NaiveDate>,
    #[serde(with = "crate::data::date_utils::optional_date_format")]
    pub mother_birth_date: Option<NaiveDate>,
    #[serde(with = "crate::data::date_utils::optional_date_format")]
    pub father_birth_date: Option<NaiveDate>,
}

impl Record {
    /// Check if the record represents a case (has treatment date)
    pub fn is_case(&self) -> bool {
        self.treatment_date.is_some()
    }

    /// Convert the record to DateData for efficient processing
    pub fn to_date_data(&self, epoch: &NaiveDate) -> DateData {
        // Implementation
    }
}

/// Internal representation of dates as days since epoch
#[derive(Debug, Clone, Copy)]
pub struct DateData {
    pub birth: i64,
    pub mother: Option<i64>,
    pub father: Option<i64>,
}

/// Functions for loading and validating records
pub fn load_records(filename: &str) -> Result<Vec<Record>> {
    // Implementation
}

pub fn validate_csv_format(filename: &str) -> Result<()> {
    // Implementation
}
```

### sampling/criteria.rs

```rust
//! Matching criteria for incidence density sampling

use crate::error::{Error, Result};

/// Criteria for matching cases and controls
#[derive(Debug, Clone, Copy)]
pub struct MatchingCriteria {
    /// Maximum allowed difference in birth dates (in days)
    pub birth_date_window: i64,
    /// Maximum allowed difference in parent ages (in days)
    pub parent_date_window: i64,
}

impl MatchingCriteria {
    /// Create a new set of matching criteria
    ///
    /// # Arguments
    /// * `birth_date_window` - Maximum allowed difference in birth dates (in days)
    /// * `parent_date_window` - Maximum allowed difference in parent ages (in days)
    ///
    /// # Errors
    /// Returns an error if either window is not positive
    pub fn new(birth_date_window: i64, parent_date_window: i64) -> Result<Self> {
        // Implementation with validation
    }

    /// Validate the current criteria
    pub fn validate(&self) -> Result<()> {
        // Implementation
    }
}

impl Default for MatchingCriteria {
    fn default() -> Self {
        Self {
            birth_date_window: 30,   // 30 days default
            parent_date_window: 365, // 1 year default
        }
    }
}
```

### sampling/sampler.rs

```rust
//! Core incidence density sampling implementation

use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxBuildHasher};
use smallvec::SmallVec;
use std::sync::Arc;

use crate::data::{DateData, Record};
use crate::error::{Error, Result};
use crate::sampling::MatchingCriteria;

/// Type alias for a group of control indices
pub type ControlGroup = SmallVec<[usize; 8]>;

/// Type alias for a case-control pair
pub type CaseControlPair = (usize, ControlGroup);

/// Sampler for incidence density sampling
pub struct Sampler {
    // Fields
}

impl Sampler {
    /// Create a new sampler with the given records and criteria
    pub fn new(records: Vec<Record>, criteria: MatchingCriteria) -> Result<Self> {
        // Implementation with optimized data preparation
    }

    /// Sample controls for each case according to the criteria
    pub fn sample(&self, n_controls: usize) -> Result<Vec<CaseControlPair>> {
        // Implementation with improved progress reporting
    }

    /// Get statistics about the dataset
    pub fn dataset_statistics(&self) -> String {
        // Implementation
    }

    /// Evaluate the quality of the matching
    pub fn evaluate_matching_quality(&self, pairs: &[CaseControlPair]) -> crate::quality::MatchingQuality {
        // Implementation
    }

    /// Save matched pairs to a CSV file
    pub fn save_matches_to_csv(&self, pairs: &[CaseControlPair], filename: &str) -> Result<()> {
        // Implementation
    }

    /// Save matching statistics to a CSV file
    pub fn save_matching_statistics(&self, pairs: &[CaseControlPair], filename: &str) -> Result<()> {
        // Implementation
    }

    // Private helper methods
    fn build_birth_date_index(&self) -> Arc<FxHashMap<i64, SmallVec<[usize; 16]>>> {
        // Implementation
    }

    fn is_parent_match(case_parent: Option<i64>, control_parent: Option<i64>, window: i64) -> bool {
        // Implementation
    }

    fn select_random_controls(eligible: &[usize], n_controls: usize) -> ControlGroup {
        // Implementation
    }
}
```

### quality/metrics.rs

```rust
//! Quality metrics for incidence density sampling

/// Statistics about the quality of matching
#[derive(Debug, Clone)]
pub struct MatchingStats {
    pub total_cases: usize,
    pub matched_cases: usize,
    pub total_controls: usize,
    pub avg_controls_per_case: f64,
    pub differences: MatchingDifferences,
    pub percentiles: MatchingPercentiles,
    pub balance: BalanceMetrics,
}

/// Differences between cases and controls
#[derive(Debug, Clone)]
pub struct MatchingDifferences {
    pub birth_date: Vec<i64>,
    pub mother_age: Vec<i64>,
    pub father_age: Vec<i64>,
}

/// Statistical percentiles of matching differences
#[derive(Debug, Clone)]
pub struct MatchingPercentiles {
    pub birth_date: Vec<i64>,
    pub mother_age: Vec<i64>,
    pub father_age: Vec<i64>,
}

/// Balance metrics for evaluation
#[derive(Debug, Clone)]
pub struct BalanceMetrics {
    pub birth_date: f64,
    pub parent_age: f64,
}

/// Functions to calculate matching quality metrics
pub fn calculate_percentiles(values: &[i64], percentiles: &[f64]) -> Vec<i64> {
    // Implementation
}

pub fn calculate_balance_metric(diffs: &[i64]) -> f64 {
    // Implementation
}
```

### visualization/plotter.rs

```rust
//! Plotting utilities for data visualization

use crate::error::Result;
use plotters::prelude::*;

/// Trait for plotting functionality
pub trait Plotter: std::fmt::Debug {
    fn plot_distribution(
        &self,
        data: &[i64],
        filename: &str,
        title: &str,
        x_label: &str,
    ) -> Result<()>;

    fn plot_utilization_summary(
        &self,
        output_file: &str,
        utilization_rate: f64,
        average_reuse: f64,
    ) -> Result<()>;

    fn plot_matching_stats(
        &self,
        output_file: &str,
        matched_count: usize,
        unmatched_count: usize,
        avg_controls: f64,
    ) -> Result<()>;

    fn plot_matched_pairs_summary(
        &self,
        output_file: &str,
        birth_differences: &[i64],
        mother_age_differences: &[i64],
        father_age_differences: &[i64],
    ) -> Result<()>;
}

/// Default implementation of the Plotter trait
#[derive(Debug, Default)]
pub struct DefaultPlotter;

impl DefaultPlotter {
    pub fn new() -> Self {
        Self
    }
}

impl Plotter for DefaultPlotter {
    // Implementation of all required methods
}
```

### utils/console.rs

```rust
//! Console output utilities

use colored::{ColoredString, Colorize};

/// Console output utilities for structured information display
#[derive(Debug)]
pub struct ConsoleOutput;

impl ConsoleOutput {
    // Existing methods with improvements

    /// Format a duration in a human-readable format
    pub fn format_duration(duration: std::time::Duration) -> String {
        // Implementation
    }

    /// Create a progress bar with standard styling
    pub fn create_progress_bar(total: u64) -> indicatif::ProgressBar {
        // Implementation
    }
}
```

### utils/logging.rs

```rust
//! Logging utilities

use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::{error::Error, sync::Once};

/// Configure logging with the specified level and optional file output
pub fn configure_logging(log_file: Option<&str>, level: LevelFilter) -> Result<(), Box<dyn Error>> {
    // Implementation
}
```

## 4. Key Code Improvements

1. **Error Handling**:
   - Replace Box<dyn Error> with strongly-typed Error enum
   - Make better use of the ? operator for error propagation
   - Add context to errors for better diagnostics

2. **Sampler Implementation**:
   - Redesign Record struct with better field naming
   - Optimize the parallel processing approach
   - Improve memory usage with more efficient data structures
   - Better progress reporting

3. **Quality Assessment**:
   - Split MatchingQuality into separate metrics calculation and reporting
   - Make metrics more reusable and easier to test

4. **Visualization**:
   - Improve the Plotter trait design
   - Add better chart customization options

5. **Documentation**:
   - Add comprehensive doc comments with examples
   - Document public API fully

This refactoring plan improves:

- Code organization by separating concerns more clearly
- API usability with better naming and more logical structure
- Performance through optimized data structures
- Error handling with more specific error types
- Documentation for better usability
  Add
