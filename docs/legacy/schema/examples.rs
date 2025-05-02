//! Examples of using the schema module
//!
//! This module contains examples of how to use the schema module, particularly
//! the advanced parquet filtering functionality. Includes both synchronous and
//! asynchronous Parquet reading examples.

use crate::error::Result;
use crate::schema::filter_expr::col;
use crate::schema::{
    load_parquet_files_parallel_async,

    load_parquet_files_parallel_with_filter,
    // Synchronous parquet reading
    read_parquet_with_filter,
    read_parquet_with_filter_async,
};
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
    println!(
        "Found {} records with PNR = {}",
        batches
            .iter()
            .map(arrow::array::RecordBatch::num_rows)
            .sum::<usize>(),
        pnr
    );

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
    let column_list = [
        "PNR".to_string(),
        "AGE".to_string(),
        "REGION".to_string(),
        "GENDER".to_string(),
    ];
    let columns = Some(&column_list[..]);

    // Read and filter all parquet files in the directory
    let batches = load_parquet_files_parallel_with_filter(dir, &filter, columns)?;

    // Print the number of matching records
    println!(
        "Found {} records with AGE > {} AND REGION starting with '{}'",
        batches
            .iter()
            .map(arrow::array::RecordBatch::num_rows)
            .sum::<usize>(),
        min_age,
        region_prefix
    );

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
    println!(
        "Found {} records with DATE between {} and {}",
        batches
            .iter()
            .map(arrow::array::RecordBatch::num_rows)
            .sum::<usize>(),
        start_date,
        end_date
    );

    Ok(())
}

/// Example: Asynchronous Parquet file reading with filtering
///
/// This example demonstrates how to use the async Parquet reader
/// to efficiently filter data from slow storage.
///
/// # Arguments
/// * `path` - Path to the parquet file
/// * `min_age` - Minimum age to filter by
/// * `region_prefix` - Region code prefix to filter by
///
/// # Returns
/// A result with the filtered record batches
pub async fn example_async_filter(path: &str, min_age: i64, region_prefix: &str) -> Result<()> {
    // Create a path from the string
    let path = Path::new(path);

    // Create composite filter expression
    let age_filter = col("AGE").gt(min_age);
    let region_filter = col("REGION").starts_with(region_prefix);

    // Combine filters with AND
    let filter = age_filter.and(region_filter);

    // Define columns to include in the result (projection)
    let column_list = [
        "PNR".to_string(),
        "AGE".to_string(),
        "REGION".to_string(),
        "GENDER".to_string(),
    ];

    // Read and filter the file asynchronously
    // Note: The batch_size parameter is optional, here we set it explicitly
    let batches = read_parquet_with_filter_async(
        path,
        &filter,
        Some(&column_list[..]),
        Some(32768), // Larger batch size for better throughput
    )
    .await?;

    // Print the number of matching records
    println!(
        "Found {} records with AGE > {} AND REGION starting with '{}'",
        batches
            .iter()
            .map(arrow::array::RecordBatch::num_rows)
            .sum::<usize>(),
        min_age,
        region_prefix
    );

    Ok(())
}

/// Example: Asynchronous loading of multiple Parquet files in parallel
///
/// This example demonstrates how to load multiple files in parallel
/// using the async API for better performance with slow storage.
///
/// # Arguments
/// * `dir` - Directory containing parquet files
///
/// # Returns
/// A result with the loaded record batches
pub async fn example_async_parallel_load(dir: &str) -> Result<()> {
    // Create a path from the string
    let dir = Path::new(dir);

    // Load all files in parallel asynchronously
    // Uses default batch size from the environment or the built-in default
    let batches = load_parquet_files_parallel_async(dir, None, None).await?;

    // Print the total number of records loaded
    let total_rows = batches
        .iter()
        .map(arrow::array::RecordBatch::num_rows)
        .sum::<usize>();
    println!(
        "Loaded {} record batches with {} total rows from {}",
        batches.len(),
        total_rows,
        dir.display()
    );

    Ok(())
}
