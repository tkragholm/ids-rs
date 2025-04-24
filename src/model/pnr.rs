use chrono::NaiveDate;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use crate::core::pnr as pnr_utils;
use crate::error::Result;

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
        let birth_date = pnr_utils::extract_birth_date(&value).ok();
        
        Self { value, birth_date }
    }
    
    /// Get the PNR string value
    #[must_use] pub fn value(&self) -> &str {
        &self.value
    }
    
    /// Check if the PNR format is valid
    #[must_use] pub fn is_valid(&self) -> bool {
        pnr_utils::validate_pnr(&self.value)
    }
    
    /// Get the person's birth date from the PNR
    #[must_use] pub fn birth_date(&self) -> Option<NaiveDate> {
        self.birth_date
    }
    
    /// Extracts the birth date and updates the internal cache
    pub fn extract_birth_date(&mut self) -> Result<NaiveDate> {
        let date = pnr_utils::extract_birth_date(&self.value)?;
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