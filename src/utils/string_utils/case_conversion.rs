//! String case conversion utilities.

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

/// Extension traits for string operations
pub trait StringCaseExtensions {
    /// Convert to `snake_case`
    fn to_snake_case(&self) -> String;
    
    /// Convert to camelCase
    fn to_camel_case(&self) -> String;
    
    /// Convert to Title Case
    fn to_title_case(&self) -> String;
}

impl StringCaseExtensions for str {
    fn to_snake_case(&self) -> String {
        to_snake_case(self)
    }
    
    fn to_camel_case(&self) -> String {
        to_camel_case(self)
    }
    
    fn to_title_case(&self) -> String {
        to_title_case(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("camelCase"), "camel_case");
        assert_eq!(to_snake_case("PascalCase"), "pascal_case");
        assert_eq!(to_snake_case("simple"), "simple");
        assert_eq!(to_snake_case("with space"), "with_space");
        assert_eq!(to_snake_case("with-dash"), "with_dash");
    }
    
    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("snake_case"), "snakeCase");
        assert_eq!(to_camel_case("simple"), "simple");
        assert_eq!(to_camel_case("with space"), "withSpace");
        assert_eq!(to_camel_case("with-dash"), "withDash");
    }
    
    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("snake_case"), "Snake Case");
        assert_eq!(to_title_case("camelCase"), "Camel Case");
        assert_eq!(to_title_case("simple"), "Simple");
    }
}
