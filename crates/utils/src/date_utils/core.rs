//! Core date utilities and traits.
//!
//! This module provides the core date utility traits and functions that form
//! the foundation of the date utility system.

use crate::error::{validation_error, Result};
use chrono::{Datelike, NaiveDate};

/// Date utilities trait providing common date operations
pub trait DateUtils {
    /// Calculate the age at a reference date
    ///
    /// # Arguments
    /// * `birth_date` - The birth date
    /// * `reference_date` - The reference date to calculate age at
    ///
    /// # Returns
    /// The age in years
    fn age_at(birth_date: NaiveDate, reference_date: NaiveDate) -> u32;

    /// Get the quarter (1-4) for a given date
    ///
    /// # Arguments
    /// * `date` - The date
    ///
    /// # Returns
    /// The quarter (1-4)
    fn quarter_from_date(date: NaiveDate) -> u32;

    /// Check if a year is a leap year
    ///
    /// # Arguments
    /// * `year` - The year to check
    ///
    /// # Returns
    /// `true` if the year is a leap year, `false` otherwise
    fn is_leap_year(year: i32) -> bool;

    /// Get the days in a month
    ///
    /// # Arguments
    /// * `year` - The year
    /// * `month` - The month (1-12)
    ///
    /// # Returns
    /// The number of days in the month
    fn days_in_month(year: i32, month: u32) -> u32;

    /// Create a date from year, month, and day
    ///
    /// # Arguments
    /// * `year` - The year
    /// * `month` - The month (1-12)
    /// * `day` - The day (1-31)
    ///
    /// # Returns
    /// A Result containing the date or an error if the date is invalid
    fn create_date(year: i32, month: u32, day: u32) -> Result<NaiveDate>;
}

/// Implementation of `DateUtils`
pub struct DateUtilsImpl;

impl DateUtils for DateUtilsImpl {
    fn age_at(birth_date: NaiveDate, reference_date: NaiveDate) -> u32 {
        let years = reference_date.year() - birth_date.year();

        if reference_date.month() < birth_date.month()
            || (reference_date.month() == birth_date.month()
                && reference_date.day() < birth_date.day())
        {
            (years - 1) as u32
        } else {
            years as u32
        }
    }

    fn quarter_from_date(date: NaiveDate) -> u32 {
        ((date.month() - 1) / 3) + 1
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => 0, // Invalid month
        }
    }

    fn create_date(year: i32, month: u32, day: u32) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| validation_error(format!("Invalid date: {year}-{month}-{day}")))
    }
}

/// Comprehensive trait for date-related helpers with consistent error handling
pub trait DateHelpers {
    /// Convert to `NaiveDate`
    ///
    /// # Returns
    /// * `Result<NaiveDate>` - The date as a `NaiveDate` or an error
    ///
    /// # Errors
    /// Returns an error if the conversion fails
    fn to_naive_date(&self) -> Result<NaiveDate>;

    /// Get year from date
    ///
    /// # Returns
    /// * `Result<i32>` - The year as an i32 or an error
    ///
    /// # Errors
    /// Returns an error if the conversion to `NaiveDate` fails
    fn year(&self) -> Result<i32>;

    /// Calculate age at a reference date
    ///
    /// # Arguments
    /// * `reference_date` - The date at which to calculate the age
    ///
    /// # Returns
    /// * `Result<u32>` - The age in years or an error
    ///
    /// # Errors
    /// Returns an error if:
    /// - The conversion to `NaiveDate` fails
    /// - The calculation yields an invalid age (e.g., negative)
    fn age_at(&self, reference_date: &NaiveDate) -> Result<u32>;

    /// Check if date is in a specific year
    ///
    /// # Arguments
    /// * `year` - The year to check against
    ///
    /// # Returns
    /// * `Result<bool>` - True if the date is in the specified year, false otherwise
    ///
    /// # Errors
    /// Returns an error if the conversion to `NaiveDate` fails
    fn is_in_year(&self, year: i32) -> Result<bool>;

    /// Get month from date
    ///
    /// # Returns
    /// * `Result<u32>` - The month as a u32 (1-12) or an error
    ///
    /// # Errors
    /// Returns an error if the conversion to `NaiveDate` fails
    fn month(&self) -> Result<u32>;

    /// Get day from date
    ///
    /// # Returns
    /// * `Result<u32>` - The day as a u32 (1-31) or an error
    ///
    /// # Errors
    /// Returns an error if the conversion to `NaiveDate` fails
    fn day(&self) -> Result<u32>;

    /// Get the quarter (1-4) for this date
    ///
    /// # Returns
    /// * `Result<u32>` - The quarter (1-4) or an error
    ///
    /// # Errors
    /// Returns an error if the conversion to `NaiveDate` fails
    fn quarter(&self) -> Result<u32> {
        Ok(((self.month()? - 1) / 3) + 1)
    }
}

// Implementation of DateHelpers is provided in the original files and will be adapted
// This file contains just the trait definition for now

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quarter_from_date() {
        let date1 = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2023, 4, 15).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2023, 7, 15).unwrap();
        let date4 = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();

        assert_eq!(DateUtilsImpl::quarter_from_date(date1), 1);
        assert_eq!(DateUtilsImpl::quarter_from_date(date2), 2);
        assert_eq!(DateUtilsImpl::quarter_from_date(date3), 3);
        assert_eq!(DateUtilsImpl::quarter_from_date(date4), 4);
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

    #[test]
    fn test_is_leap_year() {
        assert!(!DateUtilsImpl::is_leap_year(1900));
        assert!(DateUtilsImpl::is_leap_year(2000));
        assert!(DateUtilsImpl::is_leap_year(2004));
        assert!(!DateUtilsImpl::is_leap_year(2023));
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(DateUtilsImpl::days_in_month(2023, 1), 31);
        assert_eq!(DateUtilsImpl::days_in_month(2023, 2), 28);
        assert_eq!(DateUtilsImpl::days_in_month(2024, 2), 29);
        assert_eq!(DateUtilsImpl::days_in_month(2023, 4), 30);
    }

    #[test]
    fn test_create_date() {
        assert!(DateUtilsImpl::create_date(2023, 1, 31).is_ok());
        assert!(DateUtilsImpl::create_date(2023, 2, 31).is_err());
    }
}
