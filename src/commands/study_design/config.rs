//! Configuration for the study design command

use chrono::NaiveDate;
use std::path::PathBuf;

/// Configuration for the integrated study design command
pub struct StudyDesignCommandConfig {
    /// BEF data path
    pub bef_path: PathBuf,

    /// MFR data path
    pub mfr_path: PathBuf,

    /// LPR data path
    pub lpr_data_path: PathBuf,

    /// Include LPR2 data
    pub include_lpr2: bool,

    /// Include LPR3 data
    pub include_lpr3: bool,

    /// Start date for filtering health data (inclusive)
    pub start_date: Option<NaiveDate>,

    /// End date for filtering health data (inclusive)
    pub end_date: Option<NaiveDate>,

    /// Matching ratio (e.g., 1:4 matching would be 4)
    pub matching_ratio: usize,

    /// Maximum difference in days between birth dates
    pub birth_date_window_days: i32,

    /// Maximum difference in days between parent birth dates
    pub parent_birth_date_window_days: i32,

    /// Whether both parents are required
    pub require_both_parents: bool,

    /// Whether the same gender is required
    pub require_same_gender: bool,

    /// Output directory
    pub output_dir: PathBuf,

    /// Start year for filtering births (inclusive)
    pub birth_inclusion_start_year: i32,

    /// End year for filtering births (inclusive)
    pub birth_inclusion_end_year: i32,

    /// Whether to use async IO for better performance with slow storage
    pub use_async_io: bool,

    /// Batch size for parquet reading (None means use default)
    pub batch_size: Option<usize>,
}

impl Default for StudyDesignCommandConfig {
    fn default() -> Self {
        Self {
            bef_path: PathBuf::new(),
            mfr_path: PathBuf::new(),
            lpr_data_path: PathBuf::new(),
            include_lpr2: true,
            include_lpr3: true,
            start_date: None,
            end_date: None,
            matching_ratio: 4,
            birth_date_window_days: 30,
            parent_birth_date_window_days: 365,
            require_both_parents: false,
            require_same_gender: true,
            output_dir: PathBuf::new(),
            birth_inclusion_start_year: 1995,
            birth_inclusion_end_year: 2018,
            use_async_io: false,
            batch_size: None,
        }
    }
}
