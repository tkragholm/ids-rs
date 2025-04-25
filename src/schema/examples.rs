//! Examples of using the schema module
//!
//! This module contains examples of how to use the schema module, particularly
//! the advanced parquet filtering functionality.

use crate::error::Result;
use crate::schema::filter_expr::col;
use crate::schema::{read_parquet_with_filter, load_parquet_files_parallel_with_filter};
use std::path::Path;

/// Example: Filter parquet file by PNR
///
/// # Arguments
/// * `path` - Path to the parquet file
/// * `pnr` - The PNR value to filter by
///
/// # Returns
/// A result with the filtered record batches
pub fn example_filter_by_pnr(path: &str, pnr: &str) -> Result<()> {
    // Create a path from the string
    let path = Path::new(path);
    
    // Create a filter expression for PNR equality
    let filter = col("PNR").eq(pnr);
    
    // Read and filter the parquet file
    let batches = read_parquet_with_filter(path, &filter, None)?;
    
    // Print the number of matching records
    println!("Found {} records with PNR = {}", 
        batches.iter().map(arrow::array::RecordBatch::num_rows).sum::<usize>(), 
        pnr);
    
    Ok(())
}

/// Example: Advanced filtering with multiple conditions
///
/// # Arguments
/// * `dir` - Directory containing parquet files
/// * `min_age` - Minimum age to filter by
/// * `region_prefix` - Region code prefix to filter by
///
/// # Returns
/// A result with the filtered record batches
pub fn example_advanced_filter(dir: &str, min_age: i64, region_prefix: &str) -> Result<()> {
    // Create a path from the string
    let dir = Path::new(dir);
    
    // Create composite filter expression
    let age_filter = col("AGE").gt(min_age);
    let region_filter = col("REGION").starts_with(region_prefix);
    
    // Combine filters with AND
    let filter = age_filter.and(region_filter);
    
    // Define columns to include in the result (projection)
    let column_list = ["PNR".to_string(), 
        "AGE".to_string(), 
        "REGION".to_string(), 
        "GENDER".to_string()];
    let columns = Some(&column_list[..]);
    
    // Read and filter all parquet files in the directory
    let batches = load_parquet_files_parallel_with_filter(dir, &filter, columns)?;
    
    // Print the number of matching records
    println!("Found {} records with AGE > {} AND REGION starting with '{}'", 
        batches.iter().map(arrow::array::RecordBatch::num_rows).sum::<usize>(), 
        min_age, 
        region_prefix);
    
    Ok(())
}

/// Example: Simple date range filtering
///
/// # Arguments
/// * `path` - Path to the parquet file
/// * `start_date` - Start date as days since epoch
/// * `end_date` - End date as days since epoch
///
/// # Returns
/// A result with the filtered record batches
pub fn example_date_range_filter(path: &str, start_date: i64, end_date: i64) -> Result<()> {
    // Create a path from the string
    let path = Path::new(path);
    
    // Create date range filter
    let after_start = col("DATE").gt(start_date);
    let before_end = col("DATE").lt(end_date);
    
    // Combine filters
    let date_range = after_start.and(before_end);
    
    // Read and filter the parquet file
    let batches = read_parquet_with_filter(path, &date_range, None)?;
    
    // Print the number of matching records
    println!("Found {} records with DATE between {} and {}", 
        batches.iter().map(arrow::array::RecordBatch::num_rows).sum::<usize>(), 
        start_date, 
        end_date);
    
    Ok(())
}