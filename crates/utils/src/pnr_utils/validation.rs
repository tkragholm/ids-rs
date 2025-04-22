//! PNR validation utilities.
//!
//! This module provides utilities for validating PNRs and extracting
//! information from them.

use chrono::NaiveDate;
use crate::date_utils::parsing::DateParsingUtilsImpl;
use crate::date_utils::core::DateUtils;
use crate::error::{Result, validation_error};
use super::types::Gender;

/// Trait for PNR validation operations
pub trait PnrValidationUtils {
    /// Validate a PNR string
    ///
    /// # Arguments
    /// * `pnr` - The PNR string to validate
    ///
    /// # Returns
    /// A Result indicating whether the PNR is valid
    fn validate_pnr(pnr: &str) -> Result<()>;
    
    /// Extract the birth date from a PNR
    ///
    /// # Arguments
    /// * `pnr` - The PNR string
    ///
    /// # Returns
    /// A Result containing the birth date or an error
    fn extract_birth_date(pnr: &str) -> Result<NaiveDate>;
    
    /// Extract the gender from a PNR
    ///
    /// # Arguments
    /// * `pnr` - The PNR string
    ///
    /// # Returns
    /// A Result containing the gender or an error
    fn extract_gender(pnr: &str) -> Result<Gender>;
    
    /// Calculate the age of a person based on their PNR at a reference date
    ///
    /// # Arguments
    /// * `pnr` - The PNR string
    /// * `reference_date` - The reference date
    ///
    /// # Returns
    /// A Result containing the age in years or an error
    fn age_from_pnr(pnr: &str, reference_date: &NaiveDate) -> Result<u32>;
}

/// Implementation of PnrValidationUtils
pub struct PnrValidationUtilsImpl;

impl PnrValidationUtils for PnrValidationUtilsImpl {
    fn validate_pnr(pnr: &str) -> Result<()> {
        // Check format: DDMMYY-XXXX
        let re = regex::Regex::new(r"^(\d{2})(\d{2})(\d{2})-(\d{4})$").map_err(|e| {
            validation_error(format!("Regex error: {}", e))
        })?;
        
        let captures = re.captures(pnr).ok_or_else(|| {
            validation_error(format!("Invalid PNR format: {}", pnr))
        })?;
        
        let day = captures.get(1).unwrap().as_str().parse::<u32>().map_err(|_| {
            validation_error(format!("Invalid day in PNR: {}", pnr))
        })?;
        
        let month = captures.get(2).unwrap().as_str().parse::<u32>().map_err(|_| {
            validation_error(format!("Invalid month in PNR: {}", pnr))
        })?;
        
        let year = captures.get(3).unwrap().as_str().parse::<u32>().map_err(|_| {
            validation_error(format!("Invalid year in PNR: {}", pnr))
        })?;
        
        // Basic validation
        if month < 1 || month > 12 {
            return Err(validation_error(format!("Invalid month in PNR: {}", pnr)));
        }
        
        if day < 1 || day > 31 {
            return Err(validation_error(format!("Invalid day in PNR: {}", pnr)));
        }
        
        // Convert to full year (assuming 1900s for now, but could be extended)
        let full_year = 1900 + year;
        
        // Check if the date is valid
        if NaiveDate::from_ymd_opt(full_year as i32, month, day).is_none() {
            return Err(validation_error(format!("Invalid date in PNR: {}", pnr)));
        }
        
        Ok(())
    }
    
    fn extract_birth_date(pnr: &str) -> Result<NaiveDate> {
        // Validate first
        Self::validate_pnr(pnr)?;
        
        // Extract date components
        let re = regex::Regex::new(r"^(\d{2})(\d{2})(\d{2})-(\d{4})$").map_err(|e| {
            validation_error(format!("Regex error: {}", e))
        })?;
        
        let captures = re.captures(pnr).unwrap(); // Safe due to validation
        let day = captures.get(1).unwrap().as_str();
        let month = captures.get(2).unwrap().as_str();
        let year = captures.get(3).unwrap().as_str();
        
        // Convert to full year (assuming 1900s for now)
        let full_year = format!("19{}", year);
        
        // Parse date
        let date_str = format!("{}-{}-{}", full_year, month, day);
        DateParsingUtilsImpl::parse_iso(&date_str)
    }
    
    fn extract_gender(pnr: &str) -> Result<Gender> {
        // Validate first
        Self::validate_pnr(pnr)?;
        
        // Get the last digit
        let re = regex::Regex::new(r"^(\d{2})(\d{2})(\d{2})-(\d{4})$").map_err(|e| {
            validation_error(format!("Regex error: {}", e))
        })?;
        
        let captures = re.captures(pnr).unwrap(); // Safe due to validation
        let sequence = captures.get(4).unwrap().as_str();
        let last_digit = sequence.chars().last().unwrap().to_digit(10).unwrap() as u8;
        
        Ok(Gender::from_pnr_digit(last_digit))
    }
    
    fn age_from_pnr(pnr: &str, reference_date: &NaiveDate) -> Result<u32> {
        let birth_date = Self::extract_birth_date(pnr)?;
        Ok(DateUtils::age_at(birth_date, *reference_date))
    }
}

/// Parse a PNR string and extract its components
///
/// # Arguments
/// * `pnr` - The PNR string to parse
///
/// # Returns
/// A Result containing a tuple of (day, month, year, sequence) or an error
pub fn parse_pnr(pnr: &str) -> Result<(u32, u32, u32, u16)> {
    // Check format: DDMMYY-XXXX
    let re = regex::Regex::new(r"^(\d{2})(\d{2})(\d{2})-(\d{4})$").map_err(|e| {
        validation_error(format!("Regex error: {}", e))
    })?;
    
    let captures = re.captures(pnr).ok_or_else(|| {
        validation_error(format!("Invalid PNR format: {}", pnr))
    })?;
    
    let day = captures.get(1).unwrap().as_str().parse::<u32>().map_err(|_| {
        validation_error(format!("Invalid day in PNR: {}", pnr))
    })?;
    
    let month = captures.get(2).unwrap().as_str().parse::<u32>().map_err(|_| {
        validation_error(format!("Invalid month in PNR: {}", pnr))
    })?;
    
    let year = captures.get(3).unwrap().as_str().parse::<u32>().map_err(|_| {
        validation_error(format!("Invalid year in PNR: {}", pnr))
    })?;
    
    let sequence = captures.get(4).unwrap().as_str().parse::<u16>().map_err(|_| {
        validation_error(format!("Invalid sequence in PNR: {}", pnr))
    })?;
    
    Ok((day, month, year, sequence))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_pnr() {
        assert!(PnrValidationUtilsImpl::validate_pnr("010100-1234").is_ok());
        assert!(PnrValidationUtilsImpl::validate_pnr("311295-5678").is_ok());
        
        // Invalid format
        assert!(PnrValidationUtilsImpl::validate_pnr("0101001234").is_err());
        assert!(PnrValidationUtilsImpl::validate_pnr("01-01-00-1234").is_err());
        
        // Invalid date components
        assert!(PnrValidationUtilsImpl::validate_pnr("320100-1234").is_err()); // Invalid day
        assert!(PnrValidationUtilsImpl::validate_pnr("011300-1234").is_err()); // Invalid month
        assert!(PnrValidationUtilsImpl::validate_pnr("290200-1234").is_err()); // Invalid date (non leap year)
    }
    
    #[test]
    fn test_extract_birth_date() {
        let date = PnrValidationUtilsImpl::extract_birth_date("010100-1234").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
        
        let date = PnrValidationUtilsImpl::extract_birth_date("311295-5678").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(1995, 12, 31).unwrap());
    }
    
    #[test]
    fn test_extract_gender() {
        let gender = PnrValidationUtilsImpl::extract_gender("010100-1231").unwrap();
        assert_eq!(gender, Gender::Male);
        
        let gender = PnrValidationUtilsImpl::extract_gender("010100-1232").unwrap();
        assert_eq!(gender, Gender::Female);
    }
    
    #[test]
    fn test_age_from_pnr() {
        let reference_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        
        let age = PnrValidationUtilsImpl::age_from_pnr("010190-1234", &reference_date).unwrap();
        assert_eq!(age, 33);
        
        let age = PnrValidationUtilsImpl::age_from_pnr("311295-5678", &reference_date).unwrap();
        assert_eq!(age, 27);
    }
    
    #[test]
    fn test_parse_pnr() {
        let (day, month, year, sequence) = parse_pnr("010100-1234").unwrap();
        assert_eq!(day, 1);
        assert_eq!(month, 1);
        assert_eq!(year, 0);
        assert_eq!(sequence, 1234);
        
        let (day, month, year, sequence) = parse_pnr("311295-5678").unwrap();
        assert_eq!(day, 31);
        assert_eq!(month, 12);
        assert_eq!(year, 95);
        assert_eq!(sequence, 5678);
    }
}