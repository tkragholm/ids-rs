use chrono::{Datelike, NaiveDate};
use crate::error::{IdsError, Result};

/// Returns the quarter (1-4) for a given date
#[must_use] pub fn quarter_from_date(date: NaiveDate) -> u32 {
    ((date.month() - 1) / 3) + 1
}

/// Returns the first day of the quarter for a given date
#[must_use] pub fn start_of_quarter(date: NaiveDate) -> NaiveDate {
    let quarter = quarter_from_date(date);
    let month = ((quarter - 1) * 3) + 1;
    
    NaiveDate::from_ymd_opt(date.year(), month, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap())
}

/// Returns the last day of the quarter for a given date
#[must_use] pub fn end_of_quarter(date: NaiveDate) -> NaiveDate {
    let quarter = quarter_from_date(date);
    let month = quarter * 3;
    let year = date.year();
    let days = days_in_month(year, month);
    
    NaiveDate::from_ymd_opt(year, month, days)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, 12, 31).unwrap())
}

/// Calculates age at a reference date
#[must_use] pub fn age_at(birth_date: NaiveDate, reference_date: NaiveDate) -> u32 {
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
#[must_use] pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 => if is_leap_year(year) { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 0, // Invalid month
    }
}

/// Checks if a year is a leap year
#[must_use] pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

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