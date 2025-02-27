use chrono::NaiveDate;
use log::LevelFilter;
use std::{error::Error, sync::Once};

use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

use crate::errors::SamplingError;

// Export the console submodule
pub mod console;

static INIT: Once = Once::new();

/// Configures logging with optional file output.
///
/// # Arguments
/// * `log_file` - Optional path to a log file. If provided, logs will be written to both console and file.
///
/// # Errors
/// Returns an error if:
/// * The log file cannot be created or written to
/// * The logging configuration is invalid
///
/// # Panics
/// This function may panic if the logging configuration cannot be built due to invalid parameters
pub fn configure_logging(log_file: Option<&str>) -> Result<(), Box<dyn Error>> {
    INIT.call_once(|| {
        // Helper function to create a console appender
        let create_console_appender = || {
            ConsoleAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
                .build()
        };

        let mut config_builder = Config::builder();

        // Add console appender
        config_builder = config_builder
            .appender(Appender::builder().build("console", Box::new(create_console_appender())));

        // Add file appender if log file is specified
        if let Some(file_path) = log_file {
            if let Ok(file_appender) = log4rs::append::file::FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
                .build(file_path)
            {
                config_builder = config_builder
                    .appender(Appender::builder().build("file", Box::new(file_appender)));
            }
        }

        // Build root logger
        let mut root = Root::builder().appender("console");
        if log_file.is_some() {
            root = root.appender("file");
        }

        let config = config_builder
            .build(root.build(LevelFilter::Debug))
            .unwrap_or_else(|_| {
                // Create a new console appender for the fallback configuration
                Config::builder()
                    .appender(
                        Appender::builder().build("console", Box::new(create_console_appender())),
                    )
                    .build(
                        Root::builder()
                            .appender("console")
                            .build(LevelFilter::Debug),
                    )
                    .unwrap()
            });

        let _ = log4rs::init_config(config);
    });

    Ok(())
}

/// Validates an optional date.
///
/// # Arguments
/// * `date` - Optional NaiveDate to validate
///
/// # Errors
/// Returns `SamplingError::InvalidDate` if the date is present but invalid
pub fn validate_optional_date(date: &Option<NaiveDate>) -> Result<(), SamplingError> {
    match date {
        Some(d) => validate_date(&d.to_string()),
        None => Ok(()),
    }
}

/// Loads and validates records from a CSV file.
///
/// # Arguments
/// * `filename` - Path to the CSV file
///
/// # Errors
/// Returns an error if:
/// * The file cannot be opened or read
/// * The CSV format is invalid
/// * Any date fields contain invalid dates
/// * Record parsing fails
pub fn load_records(filename: &str) -> Result<Vec<crate::sampler::Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut records = Vec::new();

    for (idx, result) in rdr.deserialize().enumerate() {
        match result {
            Ok(record) => {
                let record: crate::sampler::Record = record;

                // Validate dates with detailed error messages
                if let Err(e) = validate_date(&record.bday.to_string()) {
                    log::error!("Invalid birth date at record {}: {}", idx + 1, e);
                    return Err(Box::new(SamplingError::invalid_date("Invalid birth date".to_string())));
                }

                if let Err(e) = validate_optional_date(&record.mother_bday) {
                    log::error!("Invalid mother birth date at record {}: {}", idx + 1, e);
                    return Err(Box::new(SamplingError::invalid_date("Invalid mother birth date".to_string())));
                }

                if let Err(e) = validate_optional_date(&record.father_bday) {
                    log::error!("Invalid father birth date at record {}: {}", idx + 1, e);
                    return Err(Box::new(SamplingError::invalid_date("Invalid father birth date".to_string())));
                }

                if let Some(treatment_date) = record.treatment_date {
                    if let Err(e) = validate_date(&treatment_date.to_string()) {
                        log::error!("Invalid treatment date at record {}: {}", idx + 1, e);
                        return Err(Box::new(SamplingError::invalid_date("Invalid treatment date".to_string())));
                    }
                }

                records.push(record);
            }
            Err(e) => {
                log::error!("Failed to parse record {} with error: {}", idx + 1, e);
                return Err(Box::new(e));
            }
        }
    }

    Ok(records)
}

/// Validates the format of a CSV file.
///
/// # Arguments
/// * `filename` - Path to the CSV file to validate
///
/// # Errors
/// Returns an error if:
/// * The file cannot be opened or read
/// * Required headers are missing
/// * The number of fields in any row is incorrect
/// * The CSV format is invalid
pub fn validate_csv_format(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?;

    // Check required headers
    let required_headers = [
        "pnr",
        "bday",
        "treatment_date",
        "mother_bday",
        "father_bday",
    ];
    for &header in &required_headers {
        if !headers.iter().any(|h| h == header) {
            return Err(format!("Missing required header: {header}").into());
        }
    }

    // Validate each row
    for (idx, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                if record.len() != required_headers.len() {
                    return Err(format!(
                        "Invalid number of fields at line {}: expected {}, got {}",
                        idx + 2, // +2 because idx starts at 0 and we need to account for header
                        required_headers.len(),
                        record.len()
                    )
                    .into());
                }
            }
            Err(e) => {
                return Err(format!("Error at line {}: {}", idx + 2, e).into());
            }
        }
    }

    Ok(())
}

/// Validates a date string.
///
/// # Arguments
/// * `date_str` - Date string in "YYYY-MM-DD" format
///
/// # Errors
/// Returns `SamplingError::invalid_date` if the date string cannot be parsed
pub fn validate_date(date_str: &str) -> Result<(), SamplingError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map(|_| ()) // Convert success to () instead of NaiveDate
        .map_err(|_| SamplingError::invalid_date("Invalid date format".to_string()))
}

pub mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    /// Deserializes a date string into a NaiveDate.
    ///
    /// # Errors
    /// Returns a deserialization error if the date string cannot be parsed
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

pub mod optional_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    /// Deserializes a date string into an Option<NaiveDate>.
    ///
    /// # Errors
    /// Returns a deserialization error if the date string is neither "NA" nor a valid date
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "NA" {
            Ok(None)
        } else {
            NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

pub struct MatchingCriteria {
    pub birth_date_window: i64,
    pub parent_date_window: i64,
}

impl MatchingCriteria {
    /// Validates the matching criteria values.
    ///
    /// # Errors
    /// Returns `SamplingError::invalid_criteria` if either window value is not positive
    pub fn validate(&self) -> Result<(), crate::errors::SamplingError> {
        if self.birth_date_window <= 0 || self.parent_date_window <= 0 {
            return Err(crate::errors::SamplingError::invalid_criteria("Birth or parent date window must be positive"));
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct DateData {
    pub birth: i64,
    pub mother: Option<i64>,
    pub father: Option<i64>,
}