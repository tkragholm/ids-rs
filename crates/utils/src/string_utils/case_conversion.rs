//! String case conversion utilities.
//!
//! This module provides utilities for converting strings between different cases
//! (camel case, snake case, title case, etc.).

/// Trait for string case conversion operations
pub trait StringCaseUtils {
    /// Convert a string to title case
    ///
    /// # Arguments
    /// * `s` - The string to convert
    ///
    /// # Returns
    /// The string converted to title case
    fn to_title_case(s: &str) -> String;

    /// Convert a string to snake case
    ///
    /// # Arguments
    /// * `s` - The string to convert
    ///
    /// # Returns
    /// The string converted to snake case
    fn to_snake_case(s: &str) -> String;

    /// Convert a string to camel case
    ///
    /// # Arguments
    /// * `s` - The string to convert
    ///
    /// # Returns
    /// The string converted to camel case
    fn to_camel_case(s: &str) -> String;

    /// Convert a string to pascal case
    ///
    /// # Arguments
    /// * `s` - The string to convert
    ///
    /// # Returns
    /// The string converted to pascal case
    fn to_pascal_case(s: &str) -> String;

    /// Convert a string to kebab case
    ///
    /// # Arguments
    /// * `s` - The string to convert
    ///
    /// # Returns
    /// The string converted to kebab case
    fn to_kebab_case(s: &str) -> String;
}

/// Implementation of `StringCaseUtils`
pub struct StringCaseUtilsImpl;

impl StringCaseUtils for StringCaseUtilsImpl {
    fn to_title_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                if word.is_empty() {
                    word.to_string()
                } else {
                    let mut chars = word.chars();
                    chars.next().map_or_else(String::new, |first| {
                        first.to_uppercase().collect::<String>() + chars.as_str()
                    })
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

    fn to_pascal_case(s: &str) -> String {
        let camel = Self::to_camel_case(s);

        if camel.is_empty() {
            return camel;
        }

        let mut chars = camel.chars();
        let first = chars.next().unwrap().to_uppercase().next().unwrap();
        first.to_string() + chars.as_str()
    }

    fn to_kebab_case(s: &str) -> String {
        Self::to_snake_case(s).replace('_', "-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_title_case() {
        assert_eq!(
            StringCaseUtilsImpl::to_title_case("hello world"),
            "Hello World"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_title_case("HELLO WORLD"),
            "HELLO WORLD"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_title_case("hello_world"),
            "Hello_world"
        );
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(
            StringCaseUtilsImpl::to_snake_case("HelloWorld"),
            "hello_world"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_snake_case("hello world"),
            "hello_world"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_snake_case("Hello-World"),
            "hello_world"
        );
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(
            StringCaseUtilsImpl::to_camel_case("hello_world"),
            "helloWorld"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_camel_case("hello world"),
            "helloWorld"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_camel_case("hello-world"),
            "helloWorld"
        );
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(
            StringCaseUtilsImpl::to_pascal_case("hello_world"),
            "HelloWorld"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_pascal_case("hello world"),
            "HelloWorld"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_pascal_case("hello-world"),
            "HelloWorld"
        );
    }

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(
            StringCaseUtilsImpl::to_kebab_case("HelloWorld"),
            "hello-world"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_kebab_case("hello world"),
            "hello-world"
        );
        assert_eq!(
            StringCaseUtilsImpl::to_kebab_case("hello_world"),
            "hello-world"
        );
    }
}
