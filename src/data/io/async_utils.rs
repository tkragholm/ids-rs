use super::filtering::PnrFilter;
use crate::error::Result;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::Path;

/// Load parquet files asynchronously
pub async fn load_parquet_files_async(
    path: &Path,
    schema: Option<&SchemaRef>,
    _batch_size: usize,
) -> Result<Vec<RecordBatch>> {
    // Create session context
    let ctx = SessionContext::new();

    // Note: batch_size configuration is handled differently in DataFusion 47.0.0
    // Configure ctx with batch size if needed

    // Create read options
    let read_options = match schema {
        Some(schema) => ParquetReadOptions::default().schema(schema),
        None => ParquetReadOptions::default(),
    };

    // Read parquet
    let df = ctx
        .read_parquet(path.to_string_lossy().to_string(), read_options)
        .await?;

    // Collect and return
    Ok(df.collect().await?)
}

/// Load parquet files asynchronously with PNR filter
pub async fn load_parquet_files_async_with_filter(
    path: &Path,
    schema: Option<&SchemaRef>,
    pnr_filter: &PnrFilter,
    _batch_size: usize,
) -> Result<Vec<RecordBatch>> {
    // Create session context
    let ctx = SessionContext::new();

    // Note: batch_size configuration is handled differently in DataFusion 47.0.0
    // Configure ctx with batch size if needed

    // Create read options
    let read_options = match schema {
        Some(schema) => ParquetReadOptions::default().schema(schema),
        None => ParquetReadOptions::default(),
    };

    // Read parquet
    let mut df = ctx
        .read_parquet(path.to_string_lossy().to_string(), read_options)
        .await?;

    // Apply PNR filter
    if let Some(expr) = pnr_filter.to_expr() {
        df = df.filter(expr)?;
    }

    // Collect and return
    Ok(df.collect().await?)
}

/// Load a directory of parquet files asynchronously
pub async fn load_parquet_directory_async(
    dir_path: &Path,
    schema: Option<&SchemaRef>,
    pnr_filter: Option<&PnrFilter>,
    _batch_size: usize,
) -> Result<Vec<RecordBatch>> {
    // Create session context
    let ctx = SessionContext::new();

    // Note: batch_size configuration is handled differently in DataFusion 47.0.0
    // Configure ctx with batch size if needed

    // Create read options
    let read_options = match schema {
        Some(schema) => ParquetReadOptions::default().schema(schema),
        None => ParquetReadOptions::default(),
    };

    // Read parquet directory
    let mut df = ctx
        .read_parquet(dir_path.to_string_lossy().to_string(), read_options)
        .await?;

    // Apply PNR filter if provided
    if let Some(filter) = pnr_filter {
        if let Some(expr) = filter.to_expr() {
            df = df.filter(expr)?;
        }
    }

    // Collect and return
    Ok(df.collect().await?)
}
