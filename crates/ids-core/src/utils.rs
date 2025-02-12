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

pub fn load_records(filename: &str) -> Result<Vec<crate::sampler::Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(filename)?;
    let records: Result<Vec<crate::sampler::Record>, Box<dyn Error>> = rdr
        .deserialize()
        .map(|result| {
            let record: crate::sampler::Record = result?;

            // Validate dates
            validate_date(&record.bday.to_string())?;
            validate_date(&record.mother_bday.to_string())?;
            validate_date(&record.father_bday.to_string())?;

            if let Some(treatment_date) = record.treatment_date {
                validate_date(&treatment_date.to_string())?;
            }

            Ok(record)
        })
        .collect();

    records
}

pub fn validate_date(date_str: &str) -> Result<NaiveDate, SamplingError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| SamplingError::InvalidDate)
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
    pub mother: i64,
    pub father: i64,
}
