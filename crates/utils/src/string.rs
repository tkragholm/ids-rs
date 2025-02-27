use crate::error::UtilsError;

/// Utility trait for string operations
pub trait StringUtils {
    /// Convert a string to title case
    fn to_title_case(s: &str) -> String;
    
    /// Convert a string to snake case
    fn to_snake_case(s: &str) -> String;
    
    /// Convert a string to camel case
    fn to_camel_case(s: &str) -> String;
    
    /// Parse a string as an i32, with a custom error message
    fn parse_i32(s: &str, error_msg: &str) -> Result<i32, UtilsError>;
    
    /// Parse a string as an f64, with a custom error message
    fn parse_f64(s: &str, error_msg: &str) -> Result<f64, UtilsError>;
    
    /// Truncate a string to a maximum length with ellipsis
    fn truncate(s: &str, max_length: usize) -> String;
}

/// Implementation of StringUtils
pub struct StringUtilsImpl;

impl StringUtils for StringUtilsImpl {
    fn to_title_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                if word.is_empty() {
                    word.to_string()
                } else {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    
    fn to_snake_case(s: &str) -> String {
        // First separate by spaces
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
    
    fn to_camel_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        
        for c in s.chars() {
            if c == ' ' || c == '_' || c == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else if result.is_empty() {
                result.push(c.to_lowercase().next().unwrap());
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    fn parse_i32(s: &str, error_msg: &str) -> Result<i32, UtilsError> {
        s.trim().parse::<i32>()
            .map_err(|_| UtilsError::Validation(format!("{}: '{}'", error_msg, s)))
    }
    
    fn parse_f64(s: &str, error_msg: &str) -> Result<f64, UtilsError> {
        s.trim().parse::<f64>()
            .map_err(|_| UtilsError::Validation(format!("{}: '{}'", error_msg, s)))
    }
    
    fn truncate(s: &str, max_length: usize) -> String {
        if s.len() <= max_length {
            return s.to_string();
        }
        
        // Try to truncate at a word boundary
        let truncated = &s[0..max_length.saturating_sub(3)];
        let mut result = String::from(truncated);
        
        // Find the last space to truncate at a word boundary
        if let Some(last_space) = result.rfind(' ') {
            result.truncate(last_space);
        }
        
        result.push_str("...");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_title_case() {
        assert_eq!(StringUtilsImpl::to_title_case("hello world"), "Hello World");
        assert_eq!(StringUtilsImpl::to_title_case("HELLO WORLD"), "HELLO WORLD");
        assert_eq!(StringUtilsImpl::to_title_case("hello_world"), "Hello_world");
    }
    
    #[test]
    fn test_to_snake_case() {
        assert_eq!(StringUtilsImpl::to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(StringUtilsImpl::to_snake_case("hello world"), "hello_world");
        assert_eq!(StringUtilsImpl::to_snake_case("Hello-World"), "hello_world");
    }
    
    #[test]
    fn test_to_camel_case() {
        assert_eq!(StringUtilsImpl::to_camel_case("hello_world"), "helloWorld");
        assert_eq!(StringUtilsImpl::to_camel_case("hello world"), "helloWorld");
        assert_eq!(StringUtilsImpl::to_camel_case("hello-world"), "helloWorld");
    }
    
    #[test]
    fn test_parse_i32() {
        assert_eq!(StringUtilsImpl::parse_i32("123", "Invalid number").unwrap(), 123);
        assert!(StringUtilsImpl::parse_i32("abc", "Invalid number").is_err());
    }
    
    #[test]
    fn test_parse_f64() {
        assert_eq!(StringUtilsImpl::parse_f64("123.45", "Invalid number").unwrap(), 123.45);
        assert!(StringUtilsImpl::parse_f64("abc", "Invalid number").is_err());
    }
    
    #[test]
    fn test_truncate() {
        assert_eq!(StringUtilsImpl::truncate("Hello world", 20), "Hello world");
        assert_eq!(StringUtilsImpl::truncate("Hello world", 8), "Hello...");
        assert_eq!(StringUtilsImpl::truncate("HelloWorld", 8), "Hello...");
    }
}