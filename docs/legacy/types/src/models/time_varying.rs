use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A value that varies over time, associated with a person
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeVaryingValue<T> {
    pub pnr: String,
    pub value: T,
    pub date: NaiveDate,
}

impl<T> TimeVaryingValue<T> {
    /// Create a new time-varying value
    pub fn new(pnr: impl Into<String>, value: T, date: NaiveDate) -> Self {
        Self {
            pnr: pnr.into(),
            value,
            date,
        }
    }
}
