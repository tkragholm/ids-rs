use chrono::{Datelike, NaiveDate};
use crate::error::{date_parse_error, Result};

/// Get the quarter (1-4) from a date
#[must_use]
pub fn quarter_from_date(date: NaiveDate) -> u32 {
    ((date.month() - 1) / 3) + 1
}

/// Utility trait for date operations
pub trait DateUtils {
    /// Parse a date string in various formats (YMD, DMY, etc.)
    fn parse_flexible(date_str: &str) -> Result<NaiveDate>;
    
    /// Convert a date to a period string (YYYYMM format)
    fn to_period_string(date: NaiveDate) -> String;
    
    /// Get start date of quarter for a given date
    fn start_of_quarter(date: NaiveDate) -> NaiveDate;
    
    /// Get end date of quarter for a given date
    fn end_of_quarter(date: NaiveDate) -> NaiveDate;
    
    /// Get start date of year for a given date
    fn start_of_year(date: NaiveDate) -> NaiveDate;
    
    /// Get end date of year for a given date
    fn end_of_year(date: NaiveDate) -> NaiveDate;
    
    /// Get age in years at a given reference date
    fn age_at(birth_date: NaiveDate, reference_date: NaiveDate) -> u32;
}

/// Implementation of DateUtils
pub struct DateUtilsImpl;

impl DateUtils for DateUtilsImpl {
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
    
    fn to_period_string(date: NaiveDate) -> String {
        format!("{}{:02}", date.year(), date.month())
    }
    
    fn start_of_quarter(date: NaiveDate) -> NaiveDate {
        let quarter = quarter_from_date(date);
        let month = ((quarter - 1) * 3) + 1;
        
        NaiveDate::from_ymd_opt(date.year(), month, 1)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap())
    }
    
    fn end_of_quarter(date: NaiveDate) -> NaiveDate {
        let quarter = quarter_from_date(date);
        let month = quarter * 3;
        
        let year = date.year();
        let days_in_month = if month == 2 {
            if is_leap_year(year) { 29 } else { 28 }
        } else if [4, 6, 9, 11].contains(&month) {
            30
        } else {
            31
        };
        
        NaiveDate::from_ymd_opt(year, month, days_in_month)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, 12, 31).unwrap())
    }
    
    fn start_of_year(date: NaiveDate) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.year(), 1, 1)
            .unwrap_or(date)
    }
    
    fn end_of_year(date: NaiveDate) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.year(), 12, 31)
            .unwrap_or(date)
    }
    
    fn age_at(birth_date: NaiveDate, reference_date: NaiveDate) -> u32 {
        let years = reference_date.year() - birth_date.year();
        
        if reference_date.month() < birth_date.month() || 
           (reference_date.month() == birth_date.month() && reference_date.day() < birth_date.day()) {
            (years - 1) as u32
        } else {
            years as u32
        }
    }
}

/// Check if a year is a leap year
const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quarter_from_date() {
        let date1 = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2023, 4, 15).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2023, 7, 15).unwrap();
        let date4 = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();
        
        assert_eq!(quarter_from_date(date1), 1);
        assert_eq!(quarter_from_date(date2), 2);
        assert_eq!(quarter_from_date(date3), 3);
        assert_eq!(quarter_from_date(date4), 4);
    }
    
    #[test]
    fn test_parse_flexible() {
        assert!(DateUtilsImpl::parse_flexible("2023-01-31").is_ok());
        assert!(DateUtilsImpl::parse_flexible("31-01-2023").is_ok());
        assert!(DateUtilsImpl::parse_flexible("31/01/2023").is_ok());
        assert!(DateUtilsImpl::parse_flexible("20230131").is_ok());
        assert!(DateUtilsImpl::parse_flexible("invalid date").is_err());
    }
    
    #[test]
    fn test_to_period_string() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();
        assert_eq!(DateUtilsImpl::to_period_string(date), "202301");
    }
    
    #[test]
    fn test_start_and_end_of_quarter() {
        let date = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        
        let start = DateUtilsImpl::start_of_quarter(date);
        let end = DateUtilsImpl::end_of_quarter(date);
        
        assert_eq!(start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2023, 3, 31).unwrap());
    }
    
    #[test]
    fn test_age_at() {
        let birth_date = NaiveDate::from_ymd_opt(1990, 6, 15).unwrap();
        let before_birthday = NaiveDate::from_ymd_opt(2023, 6, 14).unwrap();
        let on_birthday = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        let after_birthday = NaiveDate::from_ymd_opt(2023, 6, 16).unwrap();
        
        assert_eq!(DateUtilsImpl::age_at(birth_date, before_birthday), 32);
        assert_eq!(DateUtilsImpl::age_at(birth_date, on_birthday), 33);
        assert_eq!(DateUtilsImpl::age_at(birth_date, after_birthday), 33);
    }
}