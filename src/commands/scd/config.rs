//! Configuration for the SCD command
//!
//! This module defines the configuration options for the SCD command.

use chrono::NaiveDate;
use std::path::PathBuf;

/// Configuration for the SCD command
pub struct ScdCommandConfig {
    /// Base directory for LPR data
    pub lpr_data_path: PathBuf,
    /// Output path for processed data
    pub output_path: PathBuf,
    /// Whether to include LPR2 data
    pub include_lpr2: bool,
    /// Whether to include LPR3 data
    pub include_lpr3: bool,
    /// Start date for filtering health data (inclusive)
    pub start_date: Option<NaiveDate>,
    /// End date for filtering health data (inclusive)
    pub end_date: Option<NaiveDate>,
    /// Diagnosis columns to check for SCD codes
    pub diagnosis_columns: Vec<String>,
    /// Patient ID column
    pub patient_id_column: String,
    /// Date column
    pub date_column: String,
}

impl Default for ScdCommandConfig {
    fn default() -> Self {
        Self {
            lpr_data_path: PathBuf::from("./data/lpr"),
            output_path: PathBuf::from("./output"),
            include_lpr2: true,
            include_lpr3: true,
            start_date: None,
            end_date: None,
            diagnosis_columns: vec![
                "primary_diagnosis".to_string(),
                "secondary_diagnosis".to_string(),
            ],
            patient_id_column: "patient_id".to_string(),
            date_column: "admission_date".to_string(),
        }
    }
}