use crate::error::{Result, validation_error};

/// Convert a string to `snake_case`
#[must_use] pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_uppercase = false;
    
    for c in s.chars() {
        if c.is_uppercase() {
            if !prev_is_uppercase && !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_is_uppercase = true;
        } else if c == ' ' || c == '-' {
            result.push('_');
            prev_is_uppercase = false;
        } else {
            result.push(c);
            prev_is_uppercase = false;
        }
    }
    
    result
}

/// Convert a string to camelCase
#[must_use] pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for c in s.chars() {
        if c == '_' || c == ' ' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert a string to Title Case
#[must_use] pub fn to_title_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c == ' ' || c == '_' || c == '-' {
            result.push(' ');
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

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

/// Extension traits for string operations
pub trait StringExtensions {
    /// Convert to `snake_case`
    fn to_snake_case(&self) -> String;
    
    /// Convert to camelCase
    fn to_camel_case(&self) -> String;
    
    /// Convert to Title Case
    fn to_title_case(&self) -> String;
    
    /// Parse as i32
    fn parse_i32(&self) -> Result<i32>;
    
    /// Parse as f64
    fn parse_f64(&self) -> Result<f64>;
    
    /// Parse as Option<i32>
    fn parse_optional_i32(&self) -> Result<Option<i32>>;
    
    /// Parse as Option<f64>
    fn parse_optional_f64(&self) -> Result<Option<f64>>;
}

impl StringExtensions for str {
    fn to_snake_case(&self) -> String {
        to_snake_case(self)
    }
    
    fn to_camel_case(&self) -> String {
        to_camel_case(self)
    }
    
    fn to_title_case(&self) -> String {
        to_title_case(self)
    }
    
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