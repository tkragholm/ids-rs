use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    init_config,
};

use std::error::Error;

pub fn configure_logging() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("logfile", Box::new(logfile)),
        )
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    init_config(config).unwrap();
}

pub fn load_records(filename: &str) -> Result<Vec<crate::sampler::Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(filename)?;
    let records: Vec<crate::sampler::Record> = rdr.deserialize().collect::<Result<_, _>>()?;
    Ok(records)
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
    pub fn validate(&self) -> Result<(), crate::errors::SamplingError> {
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
