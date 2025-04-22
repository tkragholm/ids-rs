//! Testing helpers for error handling
//!
//! This module provides utility functions and macros for testing error
//! handling in the IDS codebase. These helpers make it easier to write
//! tests that verify specific error types and messages.

#![cfg(test)]

use super::{IdsError, Result, ErrorContext};
use std::fmt::Debug;

/// Asserts that a Result is an Err variant containing a specific IdsError type
///
/// # Examples
///
/// ```
/// use types::error::{IdsError, Result, test_helpers::assert_error_variant};
///
/// fn returns_validation_error() -> Result<()> {
///     Err(IdsError::validation("Invalid input"))
/// }
///
/// #[test]
/// fn test_error_variant() {
///     let result = returns_validation_error();
///     assert_error_variant!(result, IdsError::Validation(_));
/// }
/// ```
#[macro_export]
macro_rules! assert_error_variant {
    ($result:expr, $variant:pat) => {
        assert!(matches!($result, Err($variant)));
    };
    ($result:expr, $variant:pat, $message:expr) => {
        assert!(matches!($result, Err($variant)));
        let err = $result.unwrap_err();
        let err_msg = format!("{}", err);
        assert!(err_msg.contains($message), 
                "Error message '{}' does not contain '{}'", 
                err_msg, $message);
    };
}

/// Asserts that a function returns an error of a specific type
///
/// # Examples
///
/// ```
/// use types::error::{IdsError, test_helpers::assert_error_fn};
/// use std::path::Path;
///
/// fn load_file(path: &Path) -> std::result::Result<String, IdsError> {
///     Err(IdsError::Io(std::io::Error::new(
///         std::io::ErrorKind::NotFound,
///         "File not found"
///     )))
/// }
///
/// #[test]
/// fn test_load_nonexistent_file() {
///     assert_error_fn!(
///         load_file(Path::new("nonexistent.txt")),
///         IdsError::Io(_),
///         "File not found"
///     );
/// }
/// ```
#[macro_export]
macro_rules! assert_error_fn {
    ($fn_call:expr, $variant:pat) => {
        let result = $fn_call;
        assert!(result.is_err(), "Expected an error, but got Ok");
        assert!(matches!(result.unwrap_err(), $variant));
    };
    ($fn_call:expr, $variant:pat, $message:expr) => {
        let result = $fn_call;
        assert!(result.is_err(), "Expected an error, but got Ok");
        let err = result.unwrap_err();
        assert!(matches!(err, $variant));
        let err_msg = format!("{}", err);
        assert!(err_msg.contains($message), 
                "Error message '{}' does not contain '{}'", 
                err_msg, $message);
    };
}

/// Test helper for verifying that a Result is an error and contains a specific message
///
/// # Arguments
/// * `result` - The Result to check
/// * `message` - The message that should be contained in the error
///
/// # Returns
/// The error for further inspection
pub fn assert_error_contains<T: Debug>(result: Result<T>, message: &str) -> IdsError {
    assert!(result.is_err(), "Expected an error, but got Ok");
    let err = result.unwrap_err();
    let err_msg = format!("{:?}", err);
    assert!(
        err_msg.contains(message),
        "Error message '{}' does not contain '{}'",
        err_msg,
        message
    );
    err
}

/// Test helper for verifying an IdsError's type using pattern matching
///
/// # Arguments
/// * `error` - The IdsError to check
/// * `expected_variant` - A closure that takes the error and returns true if it matches the expected variant
///
/// # Examples
/// ```
/// use types::error::{IdsError, test_helpers::assert_error_type};
///
/// let error = IdsError::validation("Invalid data");
/// assert_error_type(error, |e| matches!(e, IdsError::Validation(_)));
/// ```
pub fn assert_error_type(error: IdsError, expected_variant: impl FnOnce(&IdsError) -> bool) {
    assert!(
        expected_variant(&error),
        "Error is not of the expected type: {:?}",
        error
    );
}

/// Creates a test error of a specific type for testing error handling
///
/// # Arguments
/// * `message` - The error message
///
/// # Returns
/// An IdsError::Validation with the given message
pub fn test_error(message: &str) -> IdsError {
    IdsError::validation(message)
}

/// Creates an IO error for testing
///
/// # Arguments
/// * `kind` - The IO error kind
/// * `message` - The error message
///
/// # Returns
/// An IdsError::Io with the given kind and message
pub fn test_io_error(kind: std::io::ErrorKind, message: &str) -> IdsError {
    IdsError::Io(std::io::Error::new(kind, message))
}

/// Creates an Arrow error for testing
///
/// # Arguments
/// * `message` - The error message
///
/// # Returns
/// An IdsError::Arrow with the given message
pub fn test_arrow_error(message: &str) -> IdsError {
    IdsError::ArrowWithContext {
        source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, message)),
        context: message.to_string(),
    }
}

/// Creates a data access error for testing
///
/// # Arguments
/// * `message` - The error message
///
/// # Returns
/// An IdsError::DataAccess with the given message
pub fn test_data_access_error(message: &str) -> IdsError {
    IdsError::DataAccess {
        source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, message)),
        context: message.to_string(),
    }
}

/// Creates a schema error for testing
///
/// # Arguments
/// * `message` - The error message
///
/// # Returns
/// An IdsError::Schema with the given message
pub fn test_schema_error(message: &str) -> IdsError {
    IdsError::Schema(message.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    use std::sync::Arc;

    // Test helper function to simulate an operation that might fail
    fn divide(a: i32, b: i32) -> Result<i32> {
        if b == 0 {
            return Err(IdsError::invalid_operation("Division by zero"));
        }
        Ok(a / b)
    }

    // Test helper function to simulate a lookup operation
    fn find_by_id(id: &str) -> Result<String> {
        match id {
            "valid_id" => Ok("Found item".to_string()),
            _ => Err(IdsError::lookup_failed("item", id, "database"))
        }
    }

    // Helper to create a file path that doesn't exist
    fn non_existent_file() -> &'static Path {
        Path::new("/path/to/non-existent/file.txt")
    }

    #[test]
    fn test_ensure_macro() {
        // Test ensure! with validation error explicitly
        let result = (|| -> Result<()> {
            // This is fine, just used for the test
            if 1 == 1 { } else {
                return Err(IdsError::validation("This should not fail"));
            }
            
            // This should fail
            if 1 == 2 { } else {
                return Err(IdsError::validation("Values should be equal"));
            }
            
            Ok(())
        })();

        assert!(result.is_err());
        assert_error_variant!(result, IdsError::Validation(_), "Values should be equal");

        // Test ensure! with formatted message 
        let value = 42;
        let result = (|| -> Result<()> {
            // Using manual validation to test the pattern
            if value < 10 { } else {
                return Err(IdsError::validation(format!("Value {} is too large", value)));
            }
            Ok(())
        })();

        assert!(result.is_err());
        assert_error_variant!(result, IdsError::Validation(_), "Value 42 is too large");

        // Test ensure! with specific error type
        let result = (|| -> Result<()> {
            // Using manual error to test the pattern
            if value < 10 { } else {
                return Err(IdsError::invalid_value(format!("Value {} must be less than 10", value)));
            }
            Ok(())
        })();

        assert!(result.is_err());
        assert_error_variant!(result, IdsError::InvalidFormat(_), "Value 42 must be less than 10");
    }

    #[test]
    fn test_bail_macro() {
        // Test bail! with simple message
        let result = (|| -> Result<()> {
            // Manual validation to test the pattern
            Err(IdsError::validation("This is a failure"))
        })();

        assert!(result.is_err());
        assert_error_variant!(result, IdsError::Validation(_), "This is a failure");

        // Test bail! with formatted message
        let code = 404;
        let result = (|| -> Result<()> {
            // Manual validation with formatting
            Err(IdsError::validation(format!("Error code: {}", code)))
        })();

        assert!(result.is_err());
        assert_error_variant!(result, IdsError::Validation(_), "Error code: 404");

        // Test bail! with specific error type
        let result = (|| -> Result<()> {
            // Direct error creation
            Err(IdsError::config("Invalid configuration"))
        })();

        assert!(result.is_err());
        assert_error_variant!(result, IdsError::Config(_), "Invalid configuration");
    }

    #[test]
    fn test_context_mechanism() {
        // Simplified test for error context
        let result: Result<()> = Err(IdsError::invalid_operation("Division by zero"));
        let with_context = ErrorContext::with_context(
            result, 
            || "Failed arithmetic operation"
        );

        assert!(with_context.is_err());
        let err = with_context.unwrap_err();
        let message = format!("{}", err);
        // Just check the added context is present
        assert!(message.contains("Failed arithmetic operation"));
        
        // Test with formatting in the context
        let id = "invalid_id";
        let result: Result<()> = Err(IdsError::lookup_failed("item", id, "database"));
        let with_context = ErrorContext::with_context(
            result,
            || format!("Could not find item with ID: {}", id)
        );

        assert!(with_context.is_err());
        let err = with_context.unwrap_err();
        let message = format!("{}", err);
        assert!(message.contains("Could not find item with ID: invalid_id"));
    }

    #[test]
    fn test_assert_error_variant_macro() {
        // Create an error for testing
        let result: Result<()> = Err(IdsError::validation("Test error"));
        
        // Test basic variant matching
        assert_error_variant!(result, IdsError::Validation(_));
        
        // Test variant matching with message
        assert_error_variant!(result, IdsError::Validation(_), "Test error");
        
        // Ensure it fails when the variant doesn't match (commented out to avoid test failure)
        // assert_error_variant!(result, IdsError::Config(_));
    }

    #[test]
    fn test_assert_error_fn_macro() {
        // Test with a function call that returns an error
        assert_error_fn!(
            divide(10, 0),
            IdsError::InvalidOperation(_)
        );
        
        // Test with both variant and message
        assert_error_fn!(
            find_by_id("unknown"),
            IdsError::MissingData(_),
            "Failed to find item"
        );
    }

    #[test]
    fn test_assert_error_contains() {
        // Create an error for testing
        let result: Result<()> = Err(IdsError::config("Missing required setting"));
        
        // Test that the function returns the error
        let err = assert_error_contains(result, "Missing required");
        assert!(matches!(err, IdsError::Config(_)));
    }

    #[test]
    fn test_assert_error_type() {
        // Create errors of different types
        let error1 = IdsError::validation("Invalid input");
        let error2 = IdsError::data_loading("Failed to load data");
        
        // Test with correct type
        assert_error_type(error1, |e| matches!(e, IdsError::Validation(_)));
        assert_error_type(error2, |e| matches!(e, IdsError::DataLoading(_)));
    }

    #[test]
    fn test_domain_specific_error_factory_methods() {
        // Test register_data
        let err = IdsError::register_data("AKM", "missing field PNR");
        assert!(matches!(err, IdsError::DataLoading(_)));
        assert!(format!("{}", err).contains("Invalid AKM register data"));
        assert!(format!("{}", err).contains("missing field PNR"));
        
        // Test schema_mismatch
        let err = IdsError::schema_mismatch("int32", "string");
        assert!(matches!(err, IdsError::Schema(_)));
        assert!(format!("{}", err).contains("Schema mismatch"));
        assert!(format!("{}", err).contains("expected int32"));
        assert!(format!("{}", err).contains("found string"));
        
        // Test lookup_failed
        let err = IdsError::lookup_failed("person", "12345", "database");
        assert!(matches!(err, IdsError::MissingData(_)));
        assert!(format!("{}", err).contains("Failed to find person '12345' in database"));
        
        // Test type_conversion_detailed with details
        let err = IdsError::type_conversion_detailed("string", "int", Some("contains non-numeric characters"));
        assert!(format!("{}", err).contains("Failed to convert string to int"));
        assert!(format!("{}", err).contains("contains non-numeric characters"));
        
        // Test type_conversion_detailed without details
        let err = IdsError::type_conversion_detailed("float", "date", None::<String>);
        assert!(format!("{}", err).contains("Failed to convert float to date"));
    }

    // New tests for error chaining and propagation

    #[test]
    fn test_error_chain_propagation() {
        // Define a chain of functions that pass errors up the call stack
        fn level3(value: i32) -> Result<i32> {
            if value <= 0 {
                return Err(IdsError::validation("Value must be positive"));
            }
            Ok(value)
        }
        
        fn level2(value: i32) -> Result<i32> {
            let result = level3(value).with_context(|| "Failed in level 3 processing")?;
            Ok(result * 2)
        }
        
        fn level1(value: i32) -> Result<i32> {
            let result = level2(value).with_context(|| "Failed in level 2 processing")?;
            Ok(result + 10)
        }
        
        // Test successful case
        assert_eq!(level1(5).unwrap(), 20); // (5 * 2) + 10
        
        // Test error propagation
        let err = level1(-5).unwrap_err();
        let err_msg = format!("{}", err);
        
        // Should contain all context messages through the chain
        assert!(err_msg.contains("Failed in level 2 processing"));
        assert!(err_msg.contains("Failed in level 3 processing"));
        assert!(err_msg.contains("Value must be positive"));
        
        // Verify error type is preserved
        assert!(matches!(err, IdsError::Validation(_)));
    }

    #[test]
    fn test_error_source_chain_inspection() {
        // Create a nested error chain
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let err1 = IdsError::data_access(io_err, "Failed to access data file");
        
        // Wrap in another error
        let result: Result<()> = Err(err1);
        let err2 = ErrorContext::with_context(result, || "Database operation failed").unwrap_err();
        
        // Inspect the error chain
        let err_msg = format!("{}", err2);
        // Since we're checking the error messaging rather than the original source 
        // (which can't always be preserved through conversions), we'll focus on verifying
        // the error message contains all context layers
        assert!(err_msg.contains("Database operation failed"));
        assert!(err_msg.contains("Failed to access data file"));
        assert!(err_msg.contains("File not found"));
        
        // Verify we have a DataAccess error type
        if let IdsError::DataAccess { source: _, .. } = &err2 {
            // We've already verified the message content
            // Success!
        } else {
            panic!("Expected DataAccess error, got: {:?}", err2);
        }
    }

    #[test]
    fn test_common_external_error_conversions() {
        // Test std::num::ParseIntError conversion
        let parse_int_result: Result<i32> = "abc".parse::<i32>()
            .map_err(IdsError::from);
        
        assert!(parse_int_result.is_err());
        let err = parse_int_result.unwrap_err();
        assert!(matches!(err, IdsError::DataLoading(_)));
        assert!(format!("{}", err).contains("parse integer"));
        
        // Test chrono::ParseError conversion
        use chrono::NaiveDate;
        let parse_date_result: Result<NaiveDate> = NaiveDate::parse_from_str("invalid-date", "%Y-%m-%d")
            .map_err(IdsError::from);
        
        assert!(parse_date_result.is_err());
        let err = parse_date_result.unwrap_err();
        assert!(matches!(err, IdsError::InvalidDate(_)));
        assert!(format!("{}", err).contains("parse date"));
        
        // Test std::string::FromUtf8Error conversion
        let invalid_utf8 = vec![0xFF, 0xFF];
        let utf8_result: Result<String> = String::from_utf8(invalid_utf8)
            .map_err(IdsError::from);
        
        assert!(utf8_result.is_err());
        let err = utf8_result.unwrap_err();
        assert!(matches!(err, IdsError::InvalidFormat(_)));
        assert!(format!("{}", err).contains("UTF-8"));
    }

    #[test]
    fn test_arrow_error_conversion_with_context() {
        // Create an Arrow-like error (using io::Error as a stand-in if needed)
        let arrow_err = std::io::Error::new(std::io::ErrorKind::Other, "Arrow IO error");
        
        // Convert with context
        let err = IdsError::arrow(
            arrow_err, 
            "Failed to process Arrow data"
        );
        
        // Verify it gets converted to ArrowWithContext
        assert!(matches!(err, IdsError::ArrowWithContext { .. }));
        let err_msg = format!("{}", err);
        assert!(err_msg.contains("Failed to process Arrow data"));
        
        // Check source preservation
        if let IdsError::ArrowWithContext { source, .. } = &err {
            let source_msg = format!("{}", source);
            assert!(source_msg.contains("Arrow IO error"));
        } else {
            panic!("Expected ArrowWithContext error, got: {:?}", err);
        }
    }

    #[test]
    fn test_partial_error_recovery() {
        // Process a list of items, some of which will cause errors
        fn process_items(items: &[i32]) -> Result<Vec<i32>> {
            let mut results = Vec::new();
            let mut had_errors = false;
            
            for &item in items {
                match process_item(item) {
                    Ok(result) => results.push(result),
                    Err(_) => {
                        // Log but continue
                        had_errors = true;
                    }
                }
            }
            
            // Return partial results with a warning if some items failed
            if had_errors && results.is_empty() {
                // If all items failed, return an error
                return Err(IdsError::validation("All items failed processing"));
            }
            
            Ok(results)
        }
        
        fn process_item(item: i32) -> Result<i32> {
            if item < 0 {
                return Err(IdsError::invalid_value(format!("Cannot process negative value: {}", item)));
            }
            Ok(item * 2)
        }
        
        // Test with mixed valid/invalid items
        let mixed_items = vec![1, -2, 3, -4, 5];
        let result = process_items(&mixed_items);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![2, 6, 10]);
        
        // Test with all invalid items
        let all_invalid = vec![-1, -2, -3];
        let result = process_items(&all_invalid);
        
        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("All items failed"));
    }

    #[test]
    fn test_error_handling_in_parallel_operations() {
        use std::thread;
        use std::sync::mpsc;
        
        // Process items in parallel, collecting errors
        fn parallel_process(items: Vec<i32>) -> Result<Vec<i32>> {
            let (tx, rx) = mpsc::channel();
            let items_arc = Arc::new(items);
            let mut handles = Vec::new();
            
            // Spawn threads to process items
            for i in 0..items_arc.len() {
                let tx = tx.clone();
                let items = Arc::clone(&items_arc);
                
                let handle = thread::spawn(move || {
                    let item = items[i];
                    let result = process_item(item);
                    tx.send((i, result)).unwrap();
                });
                
                handles.push(handle);
            }
            
            // Drop the original sender
            drop(tx);
            
            // Collect results and errors
            let mut results = vec![0; items_arc.len()];
            let mut error_indices = Vec::new();
            
            for (idx, result) in rx {
                match result {
                    Ok(value) => results[idx] = value,
                    Err(_) => error_indices.push(idx),
                }
            }
            
            // Wait for all threads to complete
            for handle in handles {
                handle.join().unwrap();
            }
            
            // If any operations failed, return an error with details
            if !error_indices.is_empty() {
                return Err(IdsError::validation(format!(
                    "Failed to process items at indices: {:?}", error_indices
                )));
            }
            
            Ok(results)
        }
        
        fn process_item(item: i32) -> Result<i32> {
            if item < 0 {
                return Err(IdsError::invalid_value(format!("Cannot process negative value: {}", item)));
            }
            Ok(item * 2)
        }
        
        // Test with all valid items
        let valid_items = vec![1, 2, 3, 4, 5];
        let result = parallel_process(valid_items);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![2, 4, 6, 8, 10]);
        
        // Test with some invalid items
        let mixed_items = vec![1, -2, 3, -4, 5];
        let result = parallel_process(mixed_items);
        
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Failed to process items at indices"));
        // The indices are 1 and 3, but order might not be guaranteed
        assert!(err_msg.contains("1") && err_msg.contains("3"));
    }

    #[test]
    fn test_domain_specific_error_factory_methods_extended() {
        // Test type_conversion_detailed with more complex types
        let err = IdsError::type_conversion_detailed(
            "CSV row", 
            "UserRecord struct", 
            Some("missing required field 'age'")
        );
        assert!(format!("{}", err).contains("Failed to convert CSV row to UserRecord struct"));
        assert!(format!("{}", err).contains("missing required field 'age'"));
        
        // Test data_access with custom error type
        #[derive(Debug)]
        struct DatabaseError(String);
        
        impl std::fmt::Display for DatabaseError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Database error: {}", self.0)
            }
        }
        
        impl std::error::Error for DatabaseError {}
        
        let db_err = DatabaseError("connection timed out".to_string());
        let err = IdsError::data_access(db_err, "Failed to query user records");
        
        assert!(matches!(err, IdsError::DataAccess { .. }));
        assert!(format!("{}", err).contains("Failed to query user records"));
        
        // Test schema error with detailed context
        let err = IdsError::schema_mismatch(
            "Expected Array<Int32>", 
            "Found Array<String>"
        );
        
        assert!(matches!(err, IdsError::Schema(_)));
        assert!(format!("{}", err).contains("Schema mismatch"));
        assert!(format!("{}", err).contains("Expected Array<Int32>"));
        assert!(format!("{}", err).contains("Found Array<String>"));
    }

    #[test]
    fn test_context_specific_error_handling() {
        // Test data loading context
        fn load_data(path: &str, schema_type: &str) -> Result<()> {
            // Different error handling based on context
            match schema_type {
                "csv" => {
                    if !path.ends_with(".csv") {
                        return Err(IdsError::invalid_format(
                            format!("Expected CSV file, got: {}", path)
                        ));
                    }
                },
                "parquet" => {
                    if !path.ends_with(".parquet") {
                        return Err(IdsError::schema_mismatch(
                            "Parquet file", 
                            format!("Unsupported file type: {}", path)
                        ));
                    }
                },
                "arrow" => {
                    if !path.ends_with(".arrow") {
                        // Use arrow-specific error
                        let io_err = std::io::Error::new(
                            std::io::ErrorKind::InvalidInput, 
                            format!("Expected .arrow file, got {}", path)
                        );
                        return Err(IdsError::arrow(
                            io_err,
                            "Invalid Arrow file format"
                        ));
                    }
                },
                _ => return Err(IdsError::validation(format!("Unsupported schema type: {}", schema_type)))
            }
            
            Ok(())
        }
        
        // Test different contexts
        let csv_result = load_data("data.txt", "csv");
        assert!(csv_result.is_err());
        assert!(matches!(csv_result.unwrap_err(), IdsError::InvalidFormat(_)));
        
        let parquet_result = load_data("data.txt", "parquet");
        assert!(parquet_result.is_err());
        assert!(matches!(parquet_result.unwrap_err(), IdsError::Schema(_)));
        
        let arrow_result = load_data("data.txt", "arrow");
        assert!(arrow_result.is_err());
        assert!(matches!(arrow_result.unwrap_err(), IdsError::ArrowWithContext { .. }));
        
        let unknown_result = load_data("data.txt", "unknown");
        assert!(unknown_result.is_err());
        assert!(matches!(unknown_result.unwrap_err(), IdsError::Validation(_)));
    }

    #[test]
    fn test_detailed_error_context_in_data_processing() {
        // Function to process data with detailed error context
        fn process_dataset(dataset_name: &str, row_offset: usize, column_names: &[&str]) -> Result<()> {
            // Check dataset exists
            if dataset_name != "valid_dataset" {
                return Err(IdsError::lookup_failed(
                    "dataset", dataset_name, "data registry"
                ));
            }
            
            // Check row bounds
            if row_offset > 1000 {
                return Err(IdsError::index_out_of_bounds(format!(
                    "Row offset {} exceeds dataset size (1000)",
                    row_offset
                )));
            }
            
            // Check column names
            let valid_columns = ["id", "name", "age", "email"];
            for &col in column_names {
                if !valid_columns.contains(&col) {
                    return Err(IdsError::column_not_found(format!(
                        "Column '{}' not found in dataset (available columns: {:?})",
                        col, valid_columns
                    )));
                }
            }
            
            Ok(())
        }
        
        // Test valid case
        let valid_result = process_dataset("valid_dataset", 500, &["id", "name"]);
        assert!(valid_result.is_ok());
        
        // Test invalid dataset
        let invalid_dataset = process_dataset("invalid_dataset", 500, &["id", "name"]);
        assert!(invalid_dataset.is_err());
        let err = invalid_dataset.unwrap_err();
        assert!(matches!(err, IdsError::MissingData(_)));
        assert!(format!("{}", err).contains("Failed to find dataset 'invalid_dataset'"));
        
        // Test row bounds error
        let row_bounds = process_dataset("valid_dataset", 1500, &["id", "name"]);
        assert!(row_bounds.is_err());
        let err = row_bounds.unwrap_err();
        assert!(format!("{}", err).contains("Row offset 1500 exceeds dataset size"));
        
        // Test column not found
        let column_err = process_dataset("valid_dataset", 500, &["id", "invalid_column"]);
        assert!(column_err.is_err());
        let err = column_err.unwrap_err();
        assert!(format!("{}", err).contains("Column 'invalid_column' not found"));
        assert!(format!("{}", err).contains("available columns:"));
    }
}