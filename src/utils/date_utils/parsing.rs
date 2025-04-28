//! Date parsing utilities.

use crate::error::{IdsError, Result};
use chrono::NaiveDate;

/// Parse a date from various formats
pub fn parse_flexible(date_str: &str) -> Result<NaiveDate> {
    // Try to clean up the input string
    let cleaned_str = date_str.trim();

    // Try parsing as integer YYYYMMDD first
    if let Ok(date_int) = cleaned_str.parse::<i32>() {
        if (19000101..=21000101).contains(&date_int) {
            let year = date_int / 10000;
            let month = ((date_int % 10000) / 100) as u32;
            let day = (date_int % 100) as u32;

            if (1..=12).contains(&month) && (1..=31).contains(&day) {
                if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                    return Ok(date);
                }
            }
        }
    }

    // Try various string formats
    let formats = [
        // ISO formats
        "%Y-%m-%d",
        "%Y/%m/%d",
        // Danish/European formats
        "%d-%m-%Y",
        "%d/%m/%Y",
        "%d.%m.%Y",
        // Compact formats
        "%Y%m%d",
        // With time components (ignoring time)
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%d-%m-%Y %H:%M:%S",
        "%d/%m/%Y %H:%M:%S",
        // Month name formats
        "%d %b %Y",  // 01 Jan 2020
        "%d %B %Y",  // 01 January 2020
        "%b %d, %Y", // Jan 01, 2020
        "%B %d, %Y", // January 01, 2020
        // Two-digit year formats (assuming 2000s for years 00-69, 1900s for 70-99)
        "%d-%m-%y",
        "%d/%m/%y",
        "%y-%m-%d",
        "%y/%m/%d",
    ];

    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(cleaned_str, format) {
            return Ok(date);
        }
    }

    // Log the failed attempt for debugging
    log::debug!("Failed to parse date: {cleaned_str}");

    Err(IdsError::Validation(format!(
        "Invalid date format: {cleaned_str}"
    )))
}
