//! String parsing utilities.

use crate::error::{Result, validation_error};

/// Parse a string as i32
pub fn parse_i32(s: &str) -> Result<i32> {
    s.trim()
        .parse::<i32>()
        .map_err(|_| validation_error(format!("Cannot parse as i32: '{s}'")))
}

/// Parse a string as f64
pub fn parse_f64(s: &str) -> Result<f64> {
    s.trim()
        .parse::<f64>()
        .map_err(|_| validation_error(format!("Cannot parse as f64: '{s}'")))
}

/// Parse a string as an optional i32
pub fn parse_optional_i32(s: &str) -> Result<Option<i32>> {
    let trimmed = s.trim();
    if trimmed.is_empty() || trimmed == "NA" || trimmed == "NULL" {
        Ok(None)
    } else {
        Ok(Some(parse_i32(trimmed)?))
    }
}

/// Parse a string as an optional f64
pub fn parse_optional_f64(s: &str) -> Result<Option<f64>> {
    let trimmed = s.trim();
    if trimmed.is_empty() || trimmed == "NA" || trimmed == "NULL" {
        Ok(None)
    } else {
        Ok(Some(parse_f64(trimmed)?))
    }
}

/// Extension traits for string parsing operations
pub trait StringParsingExtensions {
    /// Parse as i32
    fn parse_i32(&self) -> Result<i32>;
    
    /// Parse as f64
    fn parse_f64(&self) -> Result<f64>;
    
    /// Parse as Option<i32>
    fn parse_optional_i32(&self) -> Result<Option<i32>>;
    
    /// Parse as Option<f64>
    fn parse_optional_f64(&self) -> Result<Option<f64>>;
}

impl StringParsingExtensions for str {
    fn parse_i32(&self) -> Result<i32> {
        parse_i32(self)
    }
    
    fn parse_f64(&self) -> Result<f64> {
        parse_f64(self)
    }
    
    fn parse_optional_i32(&self) -> Result<Option<i32>> {
        parse_optional_i32(self)
    }
    
    fn parse_optional_f64(&self) -> Result<Option<f64>> {
        parse_optional_f64(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32("123").unwrap(), 123);
        assert_eq!(parse_i32(" 456 ").unwrap(), 456);
        assert!(parse_i32("abc").is_err());
    }
    
    #[test]
    fn test_parse_f64() {
        #[allow(clippy::float_cmp)]
        {
            assert_eq!(parse_f64("123.45").unwrap(), 123.45);
            assert_eq!(parse_f64(" 456.78 ").unwrap(), 456.78);
        }
        assert!(parse_f64("abc").is_err());
    }
    
    #[test]
    fn test_parse_optional_i32() {
        assert_eq!(parse_optional_i32("123").unwrap(), Some(123));
        assert_eq!(parse_optional_i32("NA").unwrap(), None);
        assert_eq!(parse_optional_i32("").unwrap(), None);
    }
    
    #[test]
    fn test_parse_optional_f64() {
        #[allow(clippy::float_cmp)]
        {
            assert_eq!(parse_optional_f64("123.45").unwrap(), Some(123.45));
        }
        assert_eq!(parse_optional_f64("NULL").unwrap(), None);
        assert_eq!(parse_optional_f64("").unwrap(), None);
    }
}
