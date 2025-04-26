//! Report generation utilities
//!
//! This module provides functions for generating various kinds of reports.

pub mod csv;
mod population;

pub use csv::generate_balance_report;
pub use csv::write_csv_report;
pub use population::save_population_summary;
