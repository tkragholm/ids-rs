//! Utility functions and helpers for common operations.
//!
//! This module provides various utility functions and helpers that are used
//! throughout the codebase, including:
//!
//! - Date handling utilities
//! - String manipulation functions
//! - Translation maps and utilities
//! - Logging utilities
//! - Common constants and default values
//!
//! Many of these utilities were previously scattered across different modules
//! and have been consolidated here for better organization.

// Re-export utilities from other modules
pub use crate::traits::utils::DateHelpers;
pub use crate::translation::{TranslationMaps, TranslationType};

#[cfg(feature = "logging")]
pub mod logging;

/// Date-related utilities for formatting and parsing.
pub mod date {
    use crate::error::{IdsError, Result};
    use chrono::NaiveDate;

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
    pub fn format_date(date: Option<NaiveDate>, format: Option<&str>) -> String {
        let format_str = format.unwrap_or("%Y-%m-%d");
        match date {
            Some(d) => d.format(format_str).to_string(),
            None => "N/A".to_string(),
        }
    }

    /// Parses a date string using the DateHelpers trait.
    ///
    /// This is a convenience wrapper around the DateHelpers trait that
    /// provides a more ergonomic API for date parsing.
    ///
    /// # Parameters
    ///
    /// * `date_str` - The date string to parse
    ///
    /// # Returns
    ///
    /// A Result containing the parsed date, or an error if parsing failed.
    pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
        // Try to parse date in format 'YYYY-MM-DD'
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
            IdsError::invalid_date(format!("Failed to parse date '{}': {}", date_str, e))
        })
    }

    /// Parses a year string into an i32.
    ///
    /// # Parameters
    ///
    /// * `year_str` - The year string to parse
    ///
    /// # Returns
    ///
    /// A Result containing the parsed year, or an error if parsing failed.
    pub fn parse_year(year_str: &str) -> Result<i32> {
        year_str.parse::<i32>().map_err(|e| {
            IdsError::invalid_format(format!("Failed to parse year '{}': {}", year_str, e))
        })
    }
}

/// String utilities for common string manipulations.
pub mod string {
    /// Sanitizes a string for use as an identifier, replacing invalid characters with underscores.
    ///
    /// # Parameters
    ///
    /// * `input` - The string to sanitize
    ///
    /// # Returns
    ///
    /// A sanitized string that can be used as an identifier.
    pub fn sanitize_identifier(input: &str) -> String {
        input
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }

    /// Truncates a string to the specified length, adding an ellipsis if truncated.
    ///
    /// # Parameters
    ///
    /// * `input` - The string to truncate
    /// * `max_length` - The maximum length
    ///
    /// # Returns
    ///
    /// A truncated string, with an ellipsis if truncated.
    pub fn truncate(input: &str, max_length: usize) -> String {
        if input.len() <= max_length {
            input.to_string()
        } else {
            format!("{}...", &input[0..max_length.saturating_sub(3)])
        }
    }
}
