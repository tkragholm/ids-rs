//! Examples of the new error handling patterns
//!
//! This file contains practical examples of using the error handling macros
//! and utilities for the IDS codebase. It is not meant to be used directly,
//! but rather as a reference and documentation for developers.

use std::path::Path;
use std::fs::File;
use std::io::Read;

use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;

use crate::error::{IdsError, Result, ErrorContext};
use crate::{ensure, try_with_context, bail};

/// Example function demonstrating error handling with file operations
///
/// This function loads a configuration file and parses it as JSON.
///
/// # Arguments
/// * `path` - The path to the configuration file
///
/// # Returns
/// Configuration settings as a map
///
/// # Errors
/// * `IdsError::Io` if the file cannot be read
/// * `IdsError::Json` if the file is not valid JSON
/// * `IdsError::Config` if the configuration is invalid
pub fn load_config(path: &Path) -> Result<serde_json::Value> {
    // Use ensure! to validate inputs
    ensure!(path.exists(), "Configuration file does not exist at: {}", path.display());
    
    // Use try_with_context! to add context to operations that might fail
    let mut file = try_with_context!(
        File::open(path),
        "Failed to open configuration file at {}",
        path.display()
    );
    
    let mut contents = String::new();
    try_with_context!(
        file.read_to_string(&mut contents),
        "Failed to read configuration file"
    );
    
    // Parse the JSON with context
    let config: serde_json::Value = try_with_context!(
        serde_json::from_str(&contents),
        "Failed to parse configuration as JSON"
    );
    
    // Use bail! for validation failures
    if !config.is_object() {
        bail!(IdsError::config("Configuration must be a JSON object"));
    }
    
    // Validate required fields
    if !config.get("version").is_some() {
        bail!("Configuration is missing required 'version' field");
    }
    
    Ok(config)
}

/// Example function demonstrating error handling with Arrow operations
///
/// This function loads a dataset from a Parquet file and performs some basic validation.
///
/// # Arguments
/// * `path` - The path to the Parquet file
/// * `schema` - The expected Arrow schema
///
/// # Returns
/// The loaded record batch
///
/// # Errors
/// * `IdsError::Io` if the file cannot be read
/// * `IdsError::ArrowWithContext` for Arrow-related errors
/// * `IdsError::Validation` if the data is invalid
pub fn load_dataset(path: &Path, schema: &Schema) -> Result<RecordBatch> {
    // Validate inputs
    ensure!(path.exists(), "Dataset file does not exist at: {}", path.display());
    
    // Simulate reading a Parquet file (in a real implementation, this would use arrow::parquet)
    let batch = match path.extension().and_then(|ext| ext.to_str()) {
        Some("parquet") => {
            // In a real implementation, this would use parquet::arrow::arrow_reader
            // Here we're just simulating an error for demonstration
            if path.to_string_lossy().contains("invalid") {
                Err(arrow::error::ArrowError::IoError(
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid Parquet file")
                ))
            } else {
                // Simulate a successful load
                Ok(RecordBatch::new_empty(schema.clone()))
            }
        },
        _ => bail!(IdsError::invalid_format(format!(
            "Unsupported file format: expected .parquet file, got {}",
            path.display()
        ))),
    };
    
    // Add context to the Arrow operation
    let batch = batch.with_context(|| format!(
        "Failed to read Parquet file at {}",
        path.display()
    ))?;
    
    // Validate the loaded data
    if batch.num_rows() == 0 {
        bail!("Dataset is empty");
    }
    
    // Validate schema compatibility
    if batch.schema().fields().len() != schema.fields().len() {
        bail!(IdsError::validation(format!(
            "Schema mismatch: expected {} fields, got {}",
            schema.fields().len(),
            batch.schema().fields().len()
        )));
    }
    
    Ok(batch)
}

/// Example function demonstrating error handling with date operations
///
/// This function parses a date string in various formats and returns a NaiveDate.
///
/// # Arguments
/// * `date_str` - The date string to parse
///
/// # Returns
/// The parsed date
///
/// # Errors
/// * `IdsError::InvalidDate` if the date cannot be parsed
pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    // Try different date formats
    let date = match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_) => NaiveDate::parse_from_str(date_str, "%d/%m/%Y")
            .or_else(|_| NaiveDate::parse_from_str(date_str, "%m-%d-%Y")),
    };
    
    // Use with_context to add a descriptive message
    date.with_context(|| format!("Failed to parse date string: '{}'", date_str))?;
    
    // Return the parsed date
    date.map_err(|e| IdsError::from(e)) // This line won't be reached due to above check
}

/// Example of using the error handling utilities in a more complex scenario
///
/// This function demonstrates how to combine multiple error handling techniques
/// in a practical data processing scenario.
///
/// # Errors
/// Various error types depending on what fails
pub fn process_data(config_path: &Path, data_path: &Path) -> Result<Vec<String>> {
    // Load configuration
    let config = load_config(config_path)?;
    
    // Extract configuration values with validation
    let batch_size = config.get("batch_size")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| IdsError::config("Missing or invalid 'batch_size' configuration"))?;
    
    ensure!(batch_size > 0, "Batch size must be positive");
    
    // Create schema
    let schema = Schema::new(vec![
        // In a real implementation, this would contain actual field definitions
    ]);
    
    // Load dataset
    let batch = load_dataset(data_path, &schema)?;
    
    // Process data (simulated)
    let mut results = Vec::new();
    
    for i in 0..batch.num_rows() {
        // In a real implementation, this would do actual data processing
        // Here we're just demonstrating error handling patterns
        
        if i % batch_size as usize == 0 && i > 0 {
            // Log progress or do batch processing
            println!("Processed {} of {} rows", i, batch.num_rows());
        }
        
        // Simulate processing each row with potential errors
        if i == batch.num_rows() / 2 {
            // Simulate a processing error at the midpoint
            bail!("Processing error at row {}: simulated failure", i);
        }
        
        // Add a result for this row
        results.push(format!("Processed row {}", i));
    }
    
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_date_valid() {
        // Test with different formats
        assert!(parse_date("2023-01-15").is_ok());
        assert!(parse_date("15/01/2023").is_ok());
        assert!(parse_date("01-15-2023").is_ok());
    }
    
    #[test]
    fn test_parse_date_invalid() {
        // Test with invalid date
        let result = parse_date("not-a-date");
        assert!(result.is_err());
        
        // Verify error type and message
        if let Err(err) = result {
            assert!(matches!(err, IdsError::InvalidDate(_)));
            assert!(format!("{}", err).contains("Failed to parse date string"));
        }
    }
}