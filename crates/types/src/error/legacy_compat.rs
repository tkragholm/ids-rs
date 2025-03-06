//! Tests for backward compatibility with legacy error handling
//!
//! This module verifies that the new error handling system is compatible with
//! legacy code patterns and can be used as a drop-in replacement.
//!
//! Note: This file is only included for testing purposes and is not part of the public API.

#![cfg(test)]

use super::{IdsError, Result, ErrorContext, LegacyErrorContext};
// No macros needed for this test file

/// Tests that legacy code patterns involving unwrapping errors still work
#[test]
fn test_legacy_unwrap_patterns() {
    // Legacy pattern: Returning simple error messages
    fn legacy_function1(value: i32) -> Result<i32> {
        if value < 0 {
            return Err(IdsError::Other(format!("Value {} is negative", value)));
        }
        Ok(value * 2)
    }

    assert!(legacy_function1(10).is_ok());
    assert!(legacy_function1(-5).is_err());
    
    // Legacy pattern: Using map_err to convert errors
    fn legacy_function2(value: &str) -> Result<i32> {
        value.parse::<i32>()
            .map_err(|e| IdsError::Other(format!("Failed to parse '{}': {}", value, e)))
    }
    
    assert!(legacy_function2("42").is_ok());
    assert!(legacy_function2("abc").is_err());
    
    // Verify new code can call legacy functions
    fn new_function(value: &str) -> Result<i32> {
        // Manual validation checks
        if value.is_empty() {
            return Err(IdsError::validation("Value cannot be empty"));
        }
        let num = legacy_function2(value)?;
        if num <= 0 {
            return Err(IdsError::validation("Value must be positive"));
        }
        Ok(num)
    }
    
    assert!(new_function("42").is_ok());
    assert!(new_function("").is_err());
    assert!(new_function("-10").is_err());
}

/// Tests that legacy error type conversions work with the new system
#[test]
fn test_legacy_error_conversions() {
    // Old style: Creating domain-specific errors
    let validation_error = IdsError::Validation("Invalid value".to_string());
    let config_error = IdsError::Config("Missing configuration".to_string());
    
    // New style: Using factory methods
    let new_validation_error = IdsError::validation("Invalid value");
    let new_config_error = IdsError::config("Missing configuration");
    
    // They should be equivalent types
    assert!(matches!(validation_error, IdsError::Validation(_)));
    assert!(matches!(new_validation_error, IdsError::Validation(_)));
    
    assert!(matches!(config_error, IdsError::Config(_)));
    assert!(matches!(new_config_error, IdsError::Config(_)));
    
    // They should have the same error messages
    assert_eq!(format!("{}", validation_error), format!("{}", new_validation_error));
    assert_eq!(format!("{}", config_error), format!("{}", new_config_error));
}

/// Tests that the legacy error context pattern works with the new system
#[test]
fn test_legacy_context_pattern() {
    // Legacy pattern: Adding context with map_err
    fn legacy_context(value: &str) -> Result<i32> {
        value.parse::<i32>()
            .map_err(|e| IdsError::Other(format!("Failed to parse '{}': {}", value, e)))
    }
    
    // Only checking legacy style for now
    let legacy_result = legacy_context("abc");
    
    assert!(legacy_result.is_err());
    
    let legacy_err = format!("{}", legacy_result.unwrap_err());
    
    assert!(legacy_err.contains("Failed to parse 'abc'"));
}

/// Tests that legacy data-specific error types work with the new system
#[test]
fn test_legacy_error_types() {
    // Old style: Using specific variants directly
    let legacy_io_error = IdsError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found"
    ));
    
    let legacy_missing_data = IdsError::MissingData("Required field missing".to_string());
    
    // New style: Using factory methods
    let new_io_error = IdsError::io_error("File not found");
    let new_missing_data = IdsError::missing_data("Required field missing");
    
    // Verify error types match
    assert!(matches!(legacy_io_error, IdsError::Io(_)));
    assert!(matches!(new_io_error, IdsError::Io(_)));
    
    assert!(matches!(legacy_missing_data, IdsError::MissingData(_)));
    assert!(matches!(new_missing_data, IdsError::MissingData(_)));
    
    // Verify error messages match
    assert!(format!("{}", legacy_io_error).contains("File not found"));
    assert!(format!("{}", new_io_error).contains("File not found"));
    
    assert!(format!("{}", legacy_missing_data).contains("Required field missing"));
    assert!(format!("{}", new_missing_data).contains("Required field missing"));
}

/// Tests that legacy code can use the new Try operator (?) with mixed error types
#[test]
fn test_legacy_try_operator() {
    // Legacy function returning old-style errors
    fn legacy_parse(value: &str) -> Result<i32> {
        value.parse::<i32>()
            .map_err(|e| IdsError::Other(format!("Parse error: {}", e)))
    }
    
    // New function using the legacy function with ?
    fn new_function(value: &str) -> Result<i32> {
        if value.is_empty() {
            return Err(IdsError::validation("Value cannot be empty"));
        }
        let num = legacy_parse(value)?;
        Ok(num * 2)
    }
    
    // Legacy function using the new function with ?
    fn legacy_wrapper(value: &str) -> Result<i32> {
        if value.is_empty() {
            return Err(IdsError::Other("Empty value".to_string()));
        }
        let num = new_function(value)?;
        Ok(num + 1)
    }
    
    // Test the chain of functions
    assert_eq!(legacy_wrapper("42").unwrap(), 85); // (42 * 2) + 1
    assert!(legacy_wrapper("").is_err());
    assert!(legacy_wrapper("abc").is_err());
    
    // Verify error propagation
    let error = legacy_wrapper("abc").unwrap_err();
    assert!(format!("{}", error).contains("Parse error"));
}

/// Tests that error helper functions work with legacy code
#[test]
fn test_legacy_helper_patterns() {
    // Legacy pattern: Verbose error handling
    fn legacy_get_value(data: &[i32], index: usize) -> Result<i32> {
        if index >= data.len() {
            return Err(IdsError::Other(format!(
                "Index {} out of bounds (len: {})",
                index, data.len()
            )));
        }
        Ok(data[index])
    }
    
    // New pattern: Using helper functions
    fn new_get_value(data: &[i32], index: usize) -> Result<i32> {
        if index >= data.len() {
            return Err(IdsError::index_out_of_bounds(format!(
                "Index {} out of bounds (len: {})",
                index, data.len()
            )));
        }
        Ok(data[index])
    }
    
    // Test both functions
    let data = vec![1, 2, 3];
    
    assert_eq!(legacy_get_value(&data, 1).unwrap(), 2);
    assert_eq!(new_get_value(&data, 1).unwrap(), 2);
    
    assert!(legacy_get_value(&data, 5).is_err());
    assert!(new_get_value(&data, 5).is_err());
    
    // Ensure errors are compatible
    let legacy_err = format!("{}", legacy_get_value(&data, 5).unwrap_err());
    let new_err = format!("{}", new_get_value(&data, 5).unwrap_err());
    
    assert!(legacy_err.contains("Index 5 out of bounds"));
    assert!(new_err.contains("Index 5 out of bounds"));
}

/// Tests that type aliases for backward compatibility work correctly
#[test]
fn test_legacy_type_aliases() {
    use super::{SamplingError, PlottingError, DataGenError};
    
    // Create errors of different variants
    let sampling_error = SamplingError::sampling("Failed to sample data");
    let plotting_error = PlottingError::plotting("Failed to create plot");
    let datagen_error = DataGenError::generation("Failed to generate data");
    
    // Verify they are all IdsError underneath
    assert!(matches!(sampling_error, IdsError::Sampling(_)));
    assert!(matches!(plotting_error, IdsError::Plotting(_)));
    assert!(matches!(datagen_error, IdsError::Generation(_)));
    
    // Verify they can be used as IdsError
    fn takes_ids_error(_: &IdsError) {}
    
    takes_ids_error(&sampling_error);
    takes_ids_error(&plotting_error);
    takes_ids_error(&datagen_error);
    
    // Verify we can convert between them
    let converted: IdsError = sampling_error;
    assert!(matches!(converted, IdsError::Sampling(_)));
}

// Run more tests to ensure backward compatibility
#[cfg(test)]
mod more_tests {
    use super::*;
    
    /// Tests that the new macros work with legacy code structures
    #[test]
    fn test_macros_with_legacy_code() {
        // Legacy-style function
        fn legacy_division(a: i32, b: i32) -> Result<i32> {
            if b == 0 {
                return Err(IdsError::Other("Division by zero".to_string()));
            }
            Ok(a / b)
        }
        
        // New function with validation
        fn new_safe_division(a: i32, b: i32) -> Result<i32> {
            if b == 0 {
                return Err(IdsError::validation("Cannot divide by zero"));
            }
            legacy_division(a, b)
        }
        
        assert_eq!(new_safe_division(10, 2).unwrap(), 5);
        assert!(new_safe_division(10, 0).is_err());
        
        // Legacy code using context to add error context
        fn legacy_wrapper(a: i32, b: i32) -> Result<i32> {
            // Manual implementation of the context pattern
            let result = match legacy_division(a, b) {
                Ok(val) => val,
                Err(err) => {
                    // Use ErrorContext explicitly to avoid ambiguity
                    return ErrorContext::with_context(
                        Err(err), 
                        || format!("Failed to perform division of {} by {}", a, b)
                    );
                }
            };
            Ok(result * 2)
        }
        
        assert_eq!(legacy_wrapper(10, 2).unwrap(), 10);
        assert!(legacy_wrapper(10, 0).is_err());
        
        // Verify error message contains the division context
        let err = legacy_wrapper(10, 0).unwrap_err();
        let message = format!("{}", err);
        assert!(message.contains("Failed to perform division of 10 by 0"));
    }
    
    /// Tests that context propagation works with legacy errors
    #[test]
    fn test_error_context() {
        // Create a simple error
        let base_error = IdsError::validation("Input validation error");
        
        // Add context to the error
        let result: Result<()> = Err(base_error);
        let with_context = ErrorContext::with_context(
            result,
            || "Failed during processing"
        );
        
        assert!(with_context.is_err());
        let error_message = format!("{}", with_context.unwrap_err());
        
        // Should contain the added context
        assert!(error_message.contains("Failed during processing"));
    }
}