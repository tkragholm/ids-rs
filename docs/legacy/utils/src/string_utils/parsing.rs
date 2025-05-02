//! String parsing utilities.
//!
//! This module provides utilities for parsing strings into various types.

use crate::error::{validation_error, Result};

/// Trait for string parsing operations
pub trait StringParsingUtils {
    /// Parse a string as an i32, with a custom error message
    ///
    /// # Arguments
    /// * `s` - The string to parse
    /// * `error_msg` - The error message to use if parsing fails
    ///
    /// # Returns
    /// A Result containing the parsed i32 or an error
    fn parse_i32(s: &str, error_msg: &str) -> Result<i32>;

    /// Parse a string as an f64, with a custom error message
    ///
    /// # Arguments
    /// * `s` - The string to parse
    /// * `error_msg` - The error message to use if parsing fails
    ///
    /// # Returns
    /// A Result containing the parsed f64 or an error
    fn parse_f64(s: &str, error_msg: &str) -> Result<f64>;

    /// Parse a string as a boolean, with a custom error message
    ///
    /// # Arguments
    /// * `s` - The string to parse
    /// * `error_msg` - The error message to use if parsing fails
    ///
    /// # Returns
    /// A Result containing the parsed boolean or an error
    fn parse_bool(s: &str, error_msg: &str) -> Result<bool>;

    /// Parse a string as an Option<i32>, with a custom error message
    ///
    /// # Arguments
    /// * `s` - The string to parse
    /// * `error_msg` - The error message to use if parsing fails
    ///
    /// # Returns
    /// A Result containing the parsed Option<i32> or an error
    /// Returns Ok(None) if the string is empty or "null"
    fn parse_optional_i32(s: &str, error_msg: &str) -> Result<Option<i32>>;

    /// Parse a string as an Option<f64>, with a custom error message
    ///
    /// # Arguments
    /// * `s` - The string to parse
    /// * `error_msg` - The error message to use if parsing fails
    ///
    /// # Returns
    /// A Result containing the parsed Option<f64> or an error
    /// Returns Ok(None) if the string is empty or "null"
    fn parse_optional_f64(s: &str, error_msg: &str) -> Result<Option<f64>>;
}

/// Implementation of `StringParsingUtils`
pub struct StringParsingUtilsImpl;

impl StringParsingUtils for StringParsingUtilsImpl {
    fn parse_i32(s: &str, error_msg: &str) -> Result<i32> {
        s.trim()
            .parse::<i32>()
            .map_err(|_| validation_error(format!("{error_msg}: '{s}'")))
    }

    fn parse_f64(s: &str, error_msg: &str) -> Result<f64> {
        s.trim()
            .parse::<f64>()
            .map_err(|_| validation_error(format!("{error_msg}: '{s}'")))
    }

    fn parse_bool(s: &str, error_msg: &str) -> Result<bool> {
        match s.trim().to_lowercase().as_str() {
            "true" | "yes" | "1" => Ok(true),
            "false" | "no" | "0" => Ok(false),
            _ => Err(validation_error(format!("{error_msg}: '{s}'"))),
        }
    }

    fn parse_optional_i32(s: &str, error_msg: &str) -> Result<Option<i32>> {
        let trimmed = s.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("null") {
            Ok(None)
        } else {
            trimmed
                .parse::<i32>()
                .map(Some)
                .map_err(|_| validation_error(format!("{error_msg}: '{s}'")))
        }
    }

    fn parse_optional_f64(s: &str, error_msg: &str) -> Result<Option<f64>> {
        let trimmed = s.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("null") {
            Ok(None)
        } else {
            trimmed
                .parse::<f64>()
                .map(Some)
                .map_err(|_| validation_error(format!("{error_msg}: '{s}'")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_i32() {
        assert_eq!(
            StringParsingUtilsImpl::parse_i32("123", "Invalid number").unwrap(),
            123
        );
        assert!(StringParsingUtilsImpl::parse_i32("abc", "Invalid number").is_err());
    }

    #[test]
    fn test_parse_f64() {
        assert_eq!(
            StringParsingUtilsImpl::parse_f64("123.45", "Invalid number").unwrap(),
            123.45
        );
        assert!(StringParsingUtilsImpl::parse_f64("abc", "Invalid number").is_err());
    }

    #[test]
    fn test_parse_bool() {
        assert!(
            StringParsingUtilsImpl::parse_bool("true", "Invalid boolean").unwrap()
        );
        assert!(
            StringParsingUtilsImpl::parse_bool("yes", "Invalid boolean").unwrap()
        );
        assert!(
            StringParsingUtilsImpl::parse_bool("1", "Invalid boolean").unwrap()
        );
        assert!(
            !StringParsingUtilsImpl::parse_bool("false", "Invalid boolean").unwrap()
        );
        assert!(
            !StringParsingUtilsImpl::parse_bool("no", "Invalid boolean").unwrap()
        );
        assert!(
            !StringParsingUtilsImpl::parse_bool("0", "Invalid boolean").unwrap()
        );
        assert!(StringParsingUtilsImpl::parse_bool("abc", "Invalid boolean").is_err());
    }

    #[test]
    fn test_parse_optional_i32() {
        assert_eq!(
            StringParsingUtilsImpl::parse_optional_i32("123", "Invalid number").unwrap(),
            Some(123)
        );
        assert_eq!(
            StringParsingUtilsImpl::parse_optional_i32("", "Invalid number").unwrap(),
            None
        );
        assert_eq!(
            StringParsingUtilsImpl::parse_optional_i32("null", "Invalid number").unwrap(),
            None
        );
        assert!(StringParsingUtilsImpl::parse_optional_i32("abc", "Invalid number").is_err());
    }

    #[test]
    fn test_parse_optional_f64() {
        assert_eq!(
            StringParsingUtilsImpl::parse_optional_f64("123.45", "Invalid number").unwrap(),
            Some(123.45)
        );
        assert_eq!(
            StringParsingUtilsImpl::parse_optional_f64("", "Invalid number").unwrap(),
            None
        );
        assert_eq!(
            StringParsingUtilsImpl::parse_optional_f64("null", "Invalid number").unwrap(),
            None
        );
        assert!(StringParsingUtilsImpl::parse_optional_f64("abc", "Invalid number").is_err());
    }
}
