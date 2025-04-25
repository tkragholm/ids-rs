//! Date parsing utilities.

use crate::error::{IdsError, Result};
use chrono::NaiveDate;

/// Parse a date from various formats
pub fn parse_flexible(date_str: &str) -> Result<NaiveDate> {
    // Try various formats
    let formats = [
        "%Y-%m-%d", 
        "%d-%m-%Y", 
        "%d/%m/%Y", 
        "%Y%m%d",   
    ];

    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(date);
        }
    }

    Err(IdsError::Validation(format!("Invalid date format: {date_str}")))
}
