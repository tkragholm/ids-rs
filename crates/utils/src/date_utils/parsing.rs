//! Date parsing utilities.
//!
//! This module provides utilities for parsing dates in various formats.

use chrono::NaiveDate;
use crate::error::{Result, date_parse_error, validation_error};

/// Trait for date parsing operations
pub trait DateParsingUtils {
    /// Parse a date string in various formats (YMD, DMY, etc.)
    ///
    /// # Arguments
    /// * `date_str` - The date string to parse
    ///
    /// # Returns
    /// A Result containing the parsed date or an error
    fn parse_flexible(date_str: &str) -> Result<NaiveDate>;
    
    /// Parse a date string in ISO format (YYYY-MM-DD)
    ///
    /// # Arguments
    /// * `date_str` - The date string to parse
    ///
    /// # Returns
    /// A Result containing the parsed date or an error
    fn parse_iso(date_str: &str) -> Result<NaiveDate>;
    
    /// Parse a year string into an i32
    ///
    /// # Arguments
    /// * `year_str` - The year string to parse
    ///
    /// # Returns
    /// A Result containing the parsed year or an error
    fn parse_year(year_str: &str) -> Result<i32>;
    
    /// Parse a period string (YYYYMM) into a date
    ///
    /// # Arguments
    /// * `period_str` - The period string to parse
    ///
    /// # Returns
    /// A Result containing the parsed date (first day of the month) or an error
    fn parse_period(period_str: &str) -> Result<NaiveDate>;
}

/// Implementation of DateParsingUtils
pub struct DateParsingUtilsImpl;

impl DateParsingUtils for DateParsingUtilsImpl {
    fn parse_flexible(date_str: &str) -> Result<NaiveDate> {
        // Try different date formats
        let formats = [
            "%Y-%m-%d",  // ISO format (2023-01-31)
            "%d-%m-%Y",  // European format (31-01-2023)
            "%d/%m/%Y",  // European format with slash (31/01/2023)
            "%Y%m%d",    // Compact format (20230131)
            "%Y%m",      // Year-month (202301)
        ];
        
        for format in &formats {
            if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
                return Ok(date);
            }
        }
        
        Err(date_parse_error(format!("Could not parse date: {date_str}")))
    }
    
    fn parse_iso(date_str: &str) -> Result<NaiveDate> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| date_parse_error(
                format!("Failed to parse ISO date '{}': {}", date_str, e)
            ))
    }
    
    fn parse_year(year_str: &str) -> Result<i32> {
        year_str.parse::<i32>()
            .map_err(|e| validation_error(
                format!("Failed to parse year '{}': {}", year_str, e)
            ))
    }
    
    fn parse_period(period_str: &str) -> Result<NaiveDate> {
        if period_str.len() != 6 {
            return Err(validation_error(
                format!("Invalid period format (expected YYYYMM): {}", period_str)
            ));
        }
        
        let year = Self::parse_year(&period_str[0..4])?;
        let month = period_str[4..6].parse::<u32>()
            .map_err(|e| validation_error(
                format!("Failed to parse month in period '{}': {}", period_str, e)
            ))?;
        
        if month < 1 || month > 12 {
            return Err(validation_error(
                format!("Invalid month in period '{}': {}", period_str, month)
            ));
        }
        
        NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| validation_error(
                format!("Invalid date for period '{}': {}-{}-01", period_str, year, month)
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_flexible() {
        assert!(DateParsingUtilsImpl::parse_flexible("2023-01-31").is_ok());
        assert!(DateParsingUtilsImpl::parse_flexible("31-01-2023").is_ok());
        assert!(DateParsingUtilsImpl::parse_flexible("31/01/2023").is_ok());
        assert!(DateParsingUtilsImpl::parse_flexible("20230131").is_ok());
        assert!(DateParsingUtilsImpl::parse_flexible("202301").is_ok());
        assert!(DateParsingUtilsImpl::parse_flexible("invalid date").is_err());
    }
    
    #[test]
    fn test_parse_iso() {
        assert!(DateParsingUtilsImpl::parse_iso("2023-01-31").is_ok());
        assert!(DateParsingUtilsImpl::parse_iso("31-01-2023").is_err());
    }
    
    #[test]
    fn test_parse_year() {
        assert_eq!(DateParsingUtilsImpl::parse_year("2023").unwrap(), 2023);
        assert!(DateParsingUtilsImpl::parse_year("abc").is_err());
    }
    
    #[test]
    fn test_parse_period() {
        let date = DateParsingUtilsImpl::parse_period("202301").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        
        assert!(DateParsingUtilsImpl::parse_period("20231").is_err());
        assert!(DateParsingUtilsImpl::parse_period("202313").is_err());
    }
}