//! Core date utilities and extensions.

use chrono::{Datelike, NaiveDate};

/// The UNIX epoch date (1970-01-01)
pub const EPOCH_DATE: NaiveDate = unsafe { NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_unchecked() };

/// Convert a `NaiveDate` to days since UNIX epoch (1970-01-01)
/// 
/// This function calculates the number of days between the given date
/// and the UNIX epoch (January 1, 1970).
/// 
/// # Arguments
/// 
/// * `date` - The date to convert
/// 
/// # Returns
/// 
/// The number of days since epoch. May be negative for dates before 1970.
#[must_use]
pub fn date_to_days_since_epoch(date: NaiveDate) -> i32 {
    (date - EPOCH_DATE).num_days() as i32
}

/// Convert days since UNIX epoch (1970-01-01) to a `NaiveDate`
/// 
/// # Arguments
/// 
/// * `days` - The number of days since epoch
/// 
/// # Returns
/// 
/// The corresponding `NaiveDate`
#[must_use]
pub fn days_since_epoch_to_date(days: i32) -> NaiveDate {
    EPOCH_DATE + chrono::Duration::days(i64::from(days))
}

/// Parse a date from a Danish format string
///
/// This function attempts to parse a date string in common Danish formats:
/// - DD-MM-YYYY
/// - DD/MM/YYYY
/// - DD.MM.YYYY
/// 
/// # Arguments
/// 
/// * `date_str` - The date string to parse
/// 
/// # Returns
/// 
/// The parsed date, or None if parsing failed
#[must_use]
pub fn parse_danish_date(date_str: &str) -> Option<NaiveDate> {
    let formats = [
        "%d-%m-%Y",
        "%d/%m/%Y",
        "%d.%m.%Y",
    ];
    
    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Some(date);
        }
    }
    
    // If Danish formats failed, try other common formats
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Some(date);
    }
    
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y%m%d") {
        return Some(date);
    }
    
    None
}

/// Returns the quarter (1-4) for a given date
#[must_use] 
pub fn quarter_from_date(date: NaiveDate) -> u32 {
    ((date.month() - 1) / 3) + 1
}

/// Returns the first day of the quarter for a given date
#[must_use] 
pub fn start_of_quarter(date: NaiveDate) -> NaiveDate {
    let quarter = quarter_from_date(date);
    let month = ((quarter - 1) * 3) + 1;
    
    NaiveDate::from_ymd_opt(date.year(), month, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap())
}

/// Returns the last day of the quarter for a given date
#[must_use] 
pub fn end_of_quarter(date: NaiveDate) -> NaiveDate {
    let quarter = quarter_from_date(date);
    let month = quarter * 3;
    let year = date.year();
    let days = days_in_month(year, month);
    
    NaiveDate::from_ymd_opt(year, month, days)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, 12, 31).unwrap())
}

/// Calculates age at a reference date
#[must_use] 
pub fn age_at(birth_date: NaiveDate, reference_date: NaiveDate) -> u32 {
    let years = reference_date.year() - birth_date.year();
    
    if reference_date.month() < birth_date.month() 
        || (reference_date.month() == birth_date.month() 
            && reference_date.day() < birth_date.day()) {
        (years - 1) as u32
    } else {
        years as u32
    }
}

/// Returns the number of days in a month
#[must_use] 
pub const fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 => if is_leap_year(year) { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 0, // Invalid month
    }
}

/// Checks if a year is a leap year
#[must_use] 
pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Extension trait for `NaiveDate`
pub trait DateExtensions {
    /// Get the quarter (1-4)
    fn quarter(&self) -> u32;
    
    /// Get the first day of the quarter
    fn start_of_quarter(&self) -> NaiveDate;
    
    /// Get the last day of the quarter
    fn end_of_quarter(&self) -> NaiveDate;
    
    /// Calculate age at reference date
    fn age_at(&self, reference_date: NaiveDate) -> u32;
}

impl DateExtensions for NaiveDate {
    fn quarter(&self) -> u32 {
        quarter_from_date(*self)
    }
    
    fn start_of_quarter(&self) -> NaiveDate {
        start_of_quarter(*self)
    }
    
    fn end_of_quarter(&self) -> NaiveDate {
        end_of_quarter(*self)
    }
    
    fn age_at(&self, reference_date: NaiveDate) -> u32 {
        age_at(*self, reference_date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_days_since_epoch() {
        // Test conversion from date to days
        let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        assert_eq!(date_to_days_since_epoch(date), 18262);
        
        // Test conversion from days to date
        let days = 18262;
        assert_eq!(days_since_epoch_to_date(days), date);
        
        // Test epoch date itself
        assert_eq!(date_to_days_since_epoch(EPOCH_DATE), 0);
        
        // Test date before epoch
        let before_epoch = NaiveDate::from_ymd_opt(1969, 12, 31).unwrap();
        assert_eq!(date_to_days_since_epoch(before_epoch), -1);
    }
    
    #[test]
    fn test_parse_danish_date() {
        // Test Danish formats
        assert_eq!(
            parse_danish_date("01-01-2020"),
            Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        );
        
        assert_eq!(
            parse_danish_date("01/01/2020"),
            Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        );
        
        assert_eq!(
            parse_danish_date("01.01.2020"),
            Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        );
        
        // Test international formats as fallback
        assert_eq!(
            parse_danish_date("2020-01-01"),
            Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        );
        
        assert_eq!(
            parse_danish_date("20200101"),
            Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        );
        
        // Test invalid format
        assert_eq!(parse_danish_date("not a date"), None);
    }
}
