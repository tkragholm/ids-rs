//! Date formatting utilities.
//!
//! This module provides utilities for formatting dates in various formats.

use chrono::NaiveDate;

/// Trait for date formatting operations
pub trait DateFormattingUtils {
    /// Format a date as a string
    ///
    /// # Arguments
    /// * `date` - The date to format
    /// * `format` - The format string to use (defaults to "%Y-%m-%d")
    ///
    /// # Returns
    /// A string containing the formatted date
    fn format_date(date: NaiveDate, format: Option<&str>) -> String;

    /// Format an optional date as a string
    ///
    /// # Arguments
    /// * `date` - The optional date to format
    /// * `format` - The format string to use (defaults to "%Y-%m-%d")
    /// * `null_placeholder` - The placeholder to use for null dates (defaults to "N/A")
    ///
    /// # Returns
    /// A string containing the formatted date or the null placeholder
    fn format_optional_date(
        date: Option<NaiveDate>,
        format: Option<&str>,
        null_placeholder: Option<&str>,
    ) -> String;
}

/// Implementation of `DateFormattingUtils`
pub struct DateFormattingUtilsImpl;

impl DateFormattingUtils for DateFormattingUtilsImpl {
    fn format_date(date: NaiveDate, format: Option<&str>) -> String {
        let format_str = format.unwrap_or("%Y-%m-%d");
        date.format(format_str).to_string()
    }

    fn format_optional_date(
        date: Option<NaiveDate>,
        format: Option<&str>,
        null_placeholder: Option<&str>,
    ) -> String {
        let format_str = format.unwrap_or("%Y-%m-%d");
        let placeholder = null_placeholder.unwrap_or("N/A");

        match date {
            Some(d) => d.format(format_str).to_string(),
            None => placeholder.to_string(),
        }
    }
}

/// Formats a date as a string in the specified format.
///
/// # Parameters
///
/// * `date` - The date to format
/// * `format` - The format string to use (defaults to "%Y-%m-%d")
///
/// # Returns
///
/// A string containing the formatted date.
///
/// # Errors
///
/// This function will not error, but will return a placeholder string
/// if the date is None.
#[must_use] pub fn format_date(date: Option<NaiveDate>, format: Option<&str>) -> String {
    let format_str = format.unwrap_or("%Y-%m-%d");
    match date {
        Some(d) => d.format(format_str).to_string(),
        None => "N/A".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();

        assert_eq!(
            DateFormattingUtilsImpl::format_date(date, None),
            "2023-01-31"
        );
        assert_eq!(
            DateFormattingUtilsImpl::format_date(date, Some("%d/%m/%Y")),
            "31/01/2023"
        );
        assert_eq!(
            DateFormattingUtilsImpl::format_date(date, Some("%Y%m%d")),
            "20230131"
        );
    }

    #[test]
    fn test_format_optional_date() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 31);

        assert_eq!(
            DateFormattingUtilsImpl::format_optional_date(date, None, None),
            "2023-01-31"
        );
        assert_eq!(
            DateFormattingUtilsImpl::format_optional_date(None, None, None),
            "N/A"
        );
        assert_eq!(
            DateFormattingUtilsImpl::format_optional_date(None, None, Some("Unknown")),
            "Unknown"
        );
    }

    #[test]
    fn test_format_date_function() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 31);

        assert_eq!(format_date(date, None), "2023-01-31");
        assert_eq!(format_date(None, None), "N/A");
        assert_eq!(format_date(date, Some("%d/%m/%Y")), "31/01/2023");
    }
}
