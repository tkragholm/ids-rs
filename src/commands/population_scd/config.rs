//! Configuration for the Population SCD command
//!
//! This module defines the configuration options for the Population SCD command.

use chrono::NaiveDate;
use std::path::PathBuf;

/// Configuration for the Population SCD command
pub struct PopulationScdCommandConfig {
    /// Path to the population data Parquet file
    pub population_path: PathBuf,
    /// Base directory for LPR data
    pub lpr_data_path: PathBuf,
    /// Output path for processed data
    pub output_dir: PathBuf,
    /// Whether to include LPR2 data
    pub include_lpr2: bool,
    /// Whether to include LPR3 data
    pub include_lpr3: bool,
    /// Start date for filtering health data (inclusive)
    pub start_date: Option<NaiveDate>,
    /// End date for filtering health data (inclusive)
    pub end_date: Option<NaiveDate>,
}

impl Default for PopulationScdCommandConfig {
    fn default() -> Self {
        Self {
            population_path: PathBuf::from("./output/population.parquet"),
            lpr_data_path: PathBuf::from("./data/lpr"),
            output_dir: PathBuf::from("./output/population_scd"),
            include_lpr2: true,
            include_lpr3: true,
            start_date: None,
            end_date: None,
        }
    }
}