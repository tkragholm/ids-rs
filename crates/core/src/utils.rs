use chrono::NaiveDate;
use log::LevelFilter;
use std::{error::Error, sync::Once};

use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

use crate::errors::SamplingError;

static INIT: Once = Once::new();

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

pub fn validate_optional_date(date: &Option<NaiveDate>) -> Result<(), SamplingError> {
    match date {
        Some(d) => validate_date(&d.to_string()),
        None => Ok(()),
    }
}

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
                    return Err(Box::new(SamplingError::InvalidDate));
                }

                if let Err(e) = validate_optional_date(&record.mother_bday) {
                    log::error!("Invalid mother birth date at record {}: {}", idx + 1, e);
                    return Err(Box::new(SamplingError::InvalidDate));
                }

                if let Err(e) = validate_optional_date(&record.father_bday) {
                    log::error!("Invalid father birth date at record {}: {}", idx + 1, e);
                    return Err(Box::new(SamplingError::InvalidDate));
                }

                if let Some(treatment_date) = record.treatment_date {
                    if let Err(e) = validate_date(&treatment_date.to_string()) {
                        log::error!("Invalid treatment date at record {}: {}", idx + 1, e);
                        return Err(Box::new(SamplingError::InvalidDate));
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

pub fn validate_date(date_str: &str) -> Result<(), SamplingError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map(|_| ()) // Convert success to () instead of NaiveDate
        .map_err(|_| SamplingError::InvalidDate)
}

pub mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

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
    pub const fn validate(&self) -> Result<(), crate::errors::SamplingError> {
        if self.birth_date_window <= 0 || self.parent_date_window <= 0 {
            return Err(crate::errors::SamplingError::InvalidCriteria);
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
