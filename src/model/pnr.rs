use chrono::NaiveDate;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use crate::error::{IdsError, Result};

/// Extract birth date from a PNR string
/// 
/// Danish PNR format: DDMMYY-XXXX
pub fn extract_birth_date(pnr: &str) -> Result<NaiveDate> {
    if pnr.len() != 11 || (pnr.chars().nth(6) != Some('-')) {
        return Err(IdsError::Validation(format!("Invalid PNR format: {pnr}")));
    }

    let day = &pnr[0..2];
    let month = &pnr[2..4];
    let year_short = &pnr[4..6];
    
    // Determine century
    let control_digit = pnr.chars().nth(7).unwrap().to_digit(10).unwrap();
    let century = match control_digit {
        0..=3 => "19",
        4..=9 => "20",
        _ => "19", // Fallback, should never happen
    };
    
    let year = format!("{century}{year_short}");
    
    // Parse the date
    let date_str = format!("{year}-{month}-{day}");
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(IdsError::Validation(format!("Invalid date in PNR: {pnr}"))),
    }
}

/// Validate a Danish PNR (Personal Identification Number)
#[must_use] pub fn validate_pnr(pnr: &str) -> bool {
    // Basic format check
    if pnr.len() != 11 || (pnr.chars().nth(6) != Some('-')) {
        return false;
    }
    
    // Digit check
    if !pnr.chars().enumerate().all(|(i, c)| {
        if i == 6 {
            c == '-'
        } else {
            c.is_ascii_digit()
        }
    }) {
        return false;
    }
    
    // Valid date check
    extract_birth_date(pnr).is_ok()
}

/// Generate check digit for a Danish PNR
pub fn generate_check_digit(pnr_without_check: &str) -> Result<u8> {
    if pnr_without_check.len() != 10 {
        return Err(IdsError::Validation("PNR without check digit must be 10 characters".to_string()));
    }
    
    // Implementation depends on specific PNR rules
    // This is a placeholder - actual implementation would follow Danish PNR rules
    Ok(0)
}

/// Personal Identification Number (PNR)
#[derive(Debug, Clone, Eq)]
pub struct Pnr {
    value: String,
    birth_date: Option<NaiveDate>,
}

impl Pnr {
    /// Create a new PNR from a string
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        let birth_date = extract_birth_date(&value).ok();
        
        Self { value, birth_date }
    }
    
    /// Get the PNR string value
    #[must_use] pub fn value(&self) -> &str {
        self.value.as_str()
    }
    
    /// Check if the PNR format is valid
    #[must_use] pub fn is_valid(&self) -> bool {
        validate_pnr(&self.value)
    }
    
    /// Get the person's birth date from the PNR
    #[must_use] pub const fn birth_date(&self) -> Option<NaiveDate> {
        self.birth_date
    }
    
    /// Extracts the birth date and updates the internal cache
    pub fn extract_birth_date(&mut self) -> Result<NaiveDate> {
        let date = extract_birth_date(&self.value)?;
        self.birth_date = Some(date);
        Ok(date)
    }
}

impl Display for Pnr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for Pnr {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for Pnr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl From<String> for Pnr {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Pnr {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}