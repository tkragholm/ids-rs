//! Date formatting utilities.

use chrono::NaiveDate;

/// Format a date as YYYY-MM-DD
#[must_use]
pub fn format_iso(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Format a date as DD/MM/YYYY
#[must_use]
pub fn format_dmy(date: NaiveDate) -> String {
    date.format("%d/%m/%Y").to_string()
}

/// Format a date as YYYYMMDD
#[must_use]
pub fn format_compact(date: NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}
