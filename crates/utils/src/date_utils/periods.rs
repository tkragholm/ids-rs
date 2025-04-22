//! Date period utilities.
//!
//! This module provides utilities for working with date periods, including
//! functions to find the start/end of quarters, years, etc.

use chrono::NaiveDate;
use crate::date_utils::core::{DateUtils, DateUtilsImpl};

/// Trait for date period operations
pub trait DatePeriodUtils {
    /// Get start date of quarter for a given date
    ///
    /// # Arguments
    /// * `date` - The date
    ///
    /// # Returns
    /// The start date of the quarter
    fn start_of_quarter(date: NaiveDate) -> NaiveDate;
    
    /// Get end date of quarter for a given date
    ///
    /// # Arguments
    /// * `date` - The date
    ///
    /// # Returns
    /// The end date of the quarter
    fn end_of_quarter(date: NaiveDate) -> NaiveDate;
    
    /// Get start date of year for a given date
    ///
    /// # Arguments
    /// * `date` - The date
    ///
    /// # Returns
    /// The start date of the year
    fn start_of_year(date: NaiveDate) -> NaiveDate;
    
    /// Get end date of year for a given date
    ///
    /// # Arguments
    /// * `date` - The date
    ///
    /// # Returns
    /// The end date of the year
    fn end_of_year(date: NaiveDate) -> NaiveDate;
    
    /// Convert a date to a period string (YYYYMM format)
    ///
    /// # Arguments
    /// * `date` - The date
    ///
    /// # Returns
    /// The period string in YYYYMM format
    fn to_period_string(date: NaiveDate) -> String;
    
    /// Convert a date to a year string (YYYY format)
    ///
    /// # Arguments
    /// * `date` - The date
    ///
    /// # Returns
    /// The year string in YYYY format
    fn to_year_string(date: NaiveDate) -> String;
}

/// Implementation of DatePeriodUtils
pub struct DatePeriodUtilsImpl;

impl DatePeriodUtils for DatePeriodUtilsImpl {
    fn start_of_quarter(date: NaiveDate) -> NaiveDate {
        let quarter = DateUtilsImpl::quarter_from_date(date);
        let month = ((quarter - 1) * 3) + 1;
        
        NaiveDate::from_ymd_opt(date.year(), month, 1)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap())
    }
    
    fn end_of_quarter(date: NaiveDate) -> NaiveDate {
        let quarter = DateUtilsImpl::quarter_from_date(date);
        let month = quarter * 3;
        let days = DateUtilsImpl::days_in_month(date.year(), month);
        
        NaiveDate::from_ymd_opt(date.year(), month, days)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(date.year(), 12, 31).unwrap())
    }
    
    fn start_of_year(date: NaiveDate) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.year(), 1, 1)
            .unwrap_or(date)
    }
    
    fn end_of_year(date: NaiveDate) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.year(), 12, 31)
            .unwrap_or(date)
    }
    
    fn to_period_string(date: NaiveDate) -> String {
        format!("{}{:02}", date.year(), date.month())
    }
    
    fn to_year_string(date: NaiveDate) -> String {
        date.year().to_string()
    }
}

/// Extract a period (YYYYMM or YYYY) from a filename
///
/// # Arguments
/// * `filename` - The filename to extract the period from
///
/// # Returns
/// An Option containing the period string, or None if no period was found
pub fn extract_period_from_filename(filename: &str) -> Option<String> {
    // Match YYYYMM pattern (e.g., 202301)
    let re_period = regex::Regex::new(r"(?:^|[^\d])(\d{6})(?:[^\d]|$)").ok()?;
    if let Some(cap) = re_period.captures(filename) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }
    
    // Match YYYY pattern (e.g., 2023)
    let re_year = regex::Regex::new(r"(?:^|[^\d])(\d{4})(?:[^\d]|$)").ok()?;
    if let Some(cap) = re_year.captures(filename) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }
    
    None
}

/// Extract a year from a filename
///
/// # Arguments
/// * `filename` - The filename to extract the year from
///
/// # Returns
/// An Option containing the year as an i32, or None if no year was found
pub fn extract_year_from_filename(filename: &str) -> Option<i32> {
    let period = extract_period_from_filename(filename)?;
    
    // If we have a 6-digit period (YYYYMM), extract the year part
    if period.len() == 6 {
        return period[0..4].parse::<i32>().ok();
    }
    
    // Otherwise, assume it's already a year
    period.parse::<i32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_start_and_end_of_quarter() {
        let date = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        
        let start = DatePeriodUtilsImpl::start_of_quarter(date);
        let end = DatePeriodUtilsImpl::end_of_quarter(date);
        
        assert_eq!(start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2023, 3, 31).unwrap());
    }
    
    #[test]
    fn test_start_and_end_of_year() {
        let date = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        
        let start = DatePeriodUtilsImpl::start_of_year(date);
        let end = DatePeriodUtilsImpl::end_of_year(date);
        
        assert_eq!(start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }
    
    #[test]
    fn test_to_period_string() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();
        assert_eq!(DatePeriodUtilsImpl::to_period_string(date), "202301");
        
        let date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        assert_eq!(DatePeriodUtilsImpl::to_period_string(date), "202312");
    }
    
    #[test]
    fn test_extract_period_from_filename() {
        assert_eq!(extract_period_from_filename("data_202301.csv"), Some("202301".to_string()));
        assert_eq!(extract_period_from_filename("data_2023.csv"), Some("2023".to_string()));
        assert_eq!(extract_period_from_filename("data.csv"), None);
    }
    
    #[test]
    fn test_extract_year_from_filename() {
        assert_eq!(extract_year_from_filename("data_202301.csv"), Some(2023));
        assert_eq!(extract_year_from_filename("data_2023.csv"), Some(2023));
        assert_eq!(extract_year_from_filename("data.csv"), None);
    }
}