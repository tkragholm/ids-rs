//! String formatting utilities.
//!
//! This module provides utilities for formatting and manipulating strings.

/// Trait for string formatting operations
pub trait StringFormattingUtils {
    /// Truncate a string to a maximum length, adding an ellipsis if truncated
    ///
    /// # Arguments
    /// * `s` - The string to truncate
    /// * `max_length` - The maximum length
    ///
    /// # Returns
    /// The truncated string, with an ellipsis if truncated
    fn truncate(s: &str, max_length: usize) -> String;
    
    /// Sanitize a string for use as an identifier
    ///
    /// # Arguments
    /// * `input` - The string to sanitize
    ///
    /// # Returns
    /// A sanitized string that can be used as an identifier
    fn sanitize_identifier(input: &str) -> String;
    
    /// Ensure a string starts with a given prefix
    ///
    /// # Arguments
    /// * `s` - The string to check
    /// * `prefix` - The prefix to ensure
    ///
    /// # Returns
    /// The string with the prefix added if it doesn't already start with it
    fn ensure_prefix(s: &str, prefix: &str) -> String;
    
    /// Ensure a string ends with a given suffix
    ///
    /// # Arguments
    /// * `s` - The string to check
    /// * `suffix` - The suffix to ensure
    ///
    /// # Returns
    /// The string with the suffix added if it doesn't already end with it
    fn ensure_suffix(s: &str, suffix: &str) -> String;
    
    /// Pad a string to a fixed width with a given character
    ///
    /// # Arguments
    /// * `s` - The string to pad
    /// * `width` - The desired width
    /// * `pad_char` - The character to use for padding (defaults to space)
    /// * `right_align` - Whether to right-align the string (defaults to false)
    ///
    /// # Returns
    /// The padded string
    fn pad(s: &str, width: usize, pad_char: Option<char>, right_align: Option<bool>) -> String;
}

/// Implementation of StringFormattingUtils
pub struct StringFormattingUtilsImpl;

impl StringFormattingUtils for StringFormattingUtilsImpl {
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
    
    fn sanitize_identifier(input: &str) -> String {
        input.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect()
    }
    
    fn ensure_prefix(s: &str, prefix: &str) -> String {
        if s.starts_with(prefix) {
            s.to_string()
        } else {
            format!("{}{}", prefix, s)
        }
    }
    
    fn ensure_suffix(s: &str, suffix: &str) -> String {
        if s.ends_with(suffix) {
            s.to_string()
        } else {
            format!("{}{}", s, suffix)
        }
    }
    
    fn pad(s: &str, width: usize, pad_char: Option<char>, right_align: Option<bool>) -> String {
        let char_to_pad = pad_char.unwrap_or(' ');
        let align_right = right_align.unwrap_or(false);
        
        if s.len() >= width {
            return s.to_string();
        }
        
        let padding = char_to_pad.to_string().repeat(width - s.len());
        
        if align_right {
            format!("{}{}", padding, s)
        } else {
            format!("{}{}", s, padding)
        }
    }
}

/// Truncates a string to the specified length, adding an ellipsis if truncated.
///
/// # Parameters
///
/// * `input` - The string to truncate
/// * `max_length` - The maximum length
///
/// # Returns
///
/// A truncated string, with an ellipsis if truncated.
pub fn truncate(input: &str, max_length: usize) -> String {
    if input.len() <= max_length {
        input.to_string()
    } else {
        format!("{}...", &input[0..max_length.saturating_sub(3)])
    }
}

/// Sanitizes a string for use as an identifier, replacing invalid characters with underscores.
///
/// # Parameters
///
/// * `input` - The string to sanitize
///
/// # Returns
///
/// A sanitized string that can be used as an identifier.
pub fn sanitize_identifier(input: &str) -> String {
    input.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_truncate() {
        assert_eq!(StringFormattingUtilsImpl::truncate("Hello world", 20), "Hello world");
        assert_eq!(StringFormattingUtilsImpl::truncate("Hello world", 8), "Hello...");
        assert_eq!(StringFormattingUtilsImpl::truncate("HelloWorld", 8), "Hello...");
    }
    
    #[test]
    fn test_sanitize_identifier() {
        assert_eq!(StringFormattingUtilsImpl::sanitize_identifier("Hello World"), "Hello_World");
        assert_eq!(StringFormattingUtilsImpl::sanitize_identifier("Hello-World"), "Hello_World");
        assert_eq!(StringFormattingUtilsImpl::sanitize_identifier("Hello_World"), "Hello_World");
    }
    
    #[test]
    fn test_ensure_prefix() {
        assert_eq!(StringFormattingUtilsImpl::ensure_prefix("World", "Hello "), "Hello World");
        assert_eq!(StringFormattingUtilsImpl::ensure_prefix("Hello World", "Hello "), "Hello World");
    }
    
    #[test]
    fn test_ensure_suffix() {
        assert_eq!(StringFormattingUtilsImpl::ensure_suffix("Hello", " World"), "Hello World");
        assert_eq!(StringFormattingUtilsImpl::ensure_suffix("Hello World", " World"), "Hello World");
    }
    
    #[test]
    fn test_pad() {
        assert_eq!(StringFormattingUtilsImpl::pad("Hello", 10, None, None), "Hello     ");
        assert_eq!(StringFormattingUtilsImpl::pad("Hello", 10, Some('-'), None), "Hello-----");
        assert_eq!(StringFormattingUtilsImpl::pad("Hello", 10, None, Some(true)), "     Hello");
        assert_eq!(StringFormattingUtilsImpl::pad("Hello", 10, Some('-'), Some(true)), "-----Hello");
        assert_eq!(StringFormattingUtilsImpl::pad("Hello World", 5, None, None), "Hello World");
    }
    
    #[test]
    fn test_truncate_function() {
        assert_eq!(truncate("Hello world", 20), "Hello world");
        assert_eq!(truncate("Hello world", 8), "Hello...");
    }
    
    #[test]
    fn test_sanitize_identifier_function() {
        assert_eq!(sanitize_identifier("Hello World"), "Hello_World");
        assert_eq!(sanitize_identifier("Hello-World"), "Hello_World");
    }
}