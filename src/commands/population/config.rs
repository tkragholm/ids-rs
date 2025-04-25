//! Configuration for the population generation command

use std::path::PathBuf;

/// Configuration for the population generation command
#[derive(Debug, Clone)]
pub struct PopulationCommandConfig {
    /// Path to the BEF data files (supports glob patterns like "*.parquet")
    pub bef_path: PathBuf,

    /// Path to the MFR data files (supports glob patterns like "*.parquet")
    pub mfr_path: PathBuf,

    /// Output directory for population data and reports
    pub output_dir: PathBuf,

    /// Start year for filtering births (inclusive)
    pub birth_inclusion_start_year: i32,

    /// End year for filtering births (inclusive)
    pub birth_inclusion_end_year: i32,
}

impl Default for PopulationCommandConfig {
    fn default() -> Self {
        Self {
            bef_path: PathBuf::from("data/registers/bef/*.parquet"),
            mfr_path: PathBuf::from("data/registers/mfr/*.parquet"),
            output_dir: PathBuf::from("data/population"),
            birth_inclusion_start_year: 1995,
            birth_inclusion_end_year: 2018,
        }
    }
}
