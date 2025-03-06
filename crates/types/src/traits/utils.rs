use chrono::{Datelike, NaiveDate};

use crate::error::{IdsError, Result};

/// Trait for date-related helper functions
///
/// This trait provides common operations for working with dates,
/// with consistent error handling.
pub trait DateHelpers {
    /// Convert to NaiveDate
    ///
    /// # Returns
    /// * `Result<NaiveDate>` - The date as a NaiveDate or an error
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
    /// Returns an error if the conversion to NaiveDate fails
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
    /// - The conversion to NaiveDate fails
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
    /// Returns an error if the conversion to NaiveDate fails
    fn is_in_year(&self, year: i32) -> Result<bool>;
    
    /// Get month from date
    ///
    /// # Returns
    /// * `Result<u32>` - The month as a u32 (1-12) or an error
    ///
    /// # Errors
    /// Returns an error if the conversion to NaiveDate fails
    fn month(&self) -> Result<u32>;
    
    /// Get day from date
    ///
    /// # Returns
    /// * `Result<u32>` - The day as a u32 (1-31) or an error
    ///
    /// # Errors
    /// Returns an error if the conversion to NaiveDate fails
    fn day(&self) -> Result<u32>;
    
    /// Get the quarter (1-4) for this date
    ///
    /// # Returns
    /// * `Result<u32>` - The quarter (1-4) or an error
    ///
    /// # Errors
    /// Returns an error if the conversion to NaiveDate fails
    fn quarter(&self) -> Result<u32> {
        Ok(((self.month()? - 1) / 3) + 1)
    }
}

/// Implementation of DateHelpers for i32 (days since epoch)
impl DateHelpers for i32 {
    fn to_naive_date(&self) -> Result<NaiveDate> {
        NaiveDate::from_num_days_from_ce_opt(*self)
            .ok_or_else(|| IdsError::date_conversion(format!("Invalid days since epoch: {}", self)))
    }
    
    fn year(&self) -> Result<i32> {
        let date = self.to_naive_date()?;
        Ok(Datelike::year(&date))
    }
    
    fn age_at(&self, reference_date: &NaiveDate) -> Result<u32> {
        let birth_date = self.to_naive_date()?;
        
        if birth_date > *reference_date {
            return Err(IdsError::invalid_value(format!(
                "Birth date ({}) is after reference date ({})",
                birth_date, reference_date
            )));
        }
        
        let mut age = Datelike::year(reference_date) - Datelike::year(&birth_date);
        
        // Adjust if birthday hasn't occurred yet in the reference year
        if Datelike::month(reference_date) < Datelike::month(&birth_date) ||
           (Datelike::month(reference_date) == Datelike::month(&birth_date) && 
            Datelike::day(reference_date) < Datelike::day(&birth_date)) {
            age -= 1;
        }
        
        Ok(age as u32)
    }
    
    fn is_in_year(&self, year: i32) -> Result<bool> {
        Ok(self.year()? == year)
    }
    
    fn month(&self) -> Result<u32> {
        let date = self.to_naive_date()?;
        Ok(Datelike::month(&date))
    }
    
    fn day(&self) -> Result<u32> {
        let date = self.to_naive_date()?;
        Ok(Datelike::day(&date))
    }
}

/// Implementation of DateHelpers for NaiveDate directly
impl DateHelpers for NaiveDate {
    fn to_naive_date(&self) -> Result<NaiveDate> {
        Ok(*self)
    }
    
    fn year(&self) -> Result<i32> {
        Ok(Datelike::year(self))
    }
    
    fn age_at(&self, reference_date: &NaiveDate) -> Result<u32> {
        if self > reference_date {
            return Err(IdsError::invalid_value(format!(
                "Birth date ({}) is after reference date ({})",
                self, reference_date
            )));
        }
        
        let mut age = Datelike::year(reference_date) - Datelike::year(self);
        
        // Adjust if birthday hasn't occurred yet in the reference year
        if Datelike::month(reference_date) < Datelike::month(self) ||
           (Datelike::month(reference_date) == Datelike::month(self) && 
            Datelike::day(reference_date) < Datelike::day(self)) {
            age -= 1;
        }
        
        Ok(age as u32)
    }
    
    fn is_in_year(&self, year: i32) -> Result<bool> {
        Ok(Datelike::year(self) == year)
    }
    
    fn month(&self) -> Result<u32> {
        Ok(Datelike::month(self))
    }
    
    fn day(&self) -> Result<u32> {
        Ok(Datelike::day(self))
    }
}

/// Implementation of DateHelpers for Option<i32>
impl DateHelpers for Option<i32> {
    fn to_naive_date(&self) -> Result<NaiveDate> {
        match self {
            Some(days) => days.to_naive_date(),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn year(&self) -> Result<i32> {
        match self {
            Some(days) => days.year(),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn age_at(&self, reference_date: &NaiveDate) -> Result<u32> {
        match self {
            Some(days) => days.age_at(reference_date),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn is_in_year(&self, year: i32) -> Result<bool> {
        match self {
            Some(days) => days.is_in_year(year),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn month(&self) -> Result<u32> {
        match self {
            Some(days) => days.month(),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn day(&self) -> Result<u32> {
        match self {
            Some(days) => days.day(),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
}

/// Implementation of DateHelpers for Option<NaiveDate>
impl DateHelpers for Option<NaiveDate> {
    fn to_naive_date(&self) -> Result<NaiveDate> {
        match self {
            Some(date) => Ok(*date),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn year(&self) -> Result<i32> {
        match self {
            Some(date) => Ok(Datelike::year(date)),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn age_at(&self, reference_date: &NaiveDate) -> Result<u32> {
        match self {
            Some(date) => DateHelpers::age_at(date, reference_date),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn is_in_year(&self, year: i32) -> Result<bool> {
        match self {
            Some(date) => Ok(Datelike::year(date) == year),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn month(&self) -> Result<u32> {
        match self {
            Some(date) => Ok(Datelike::month(date)),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
    
    fn day(&self) -> Result<u32> {
        match self {
            Some(date) => Ok(Datelike::day(date)),
            None => Err(IdsError::missing_value("Date is null".to_string())),
        }
    }
}