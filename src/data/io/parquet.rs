use crate::error::{IdsError, Result};
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;

use std::path::{Path, PathBuf};

use super::filtering::PnrFilter;

/// Unified ParquetReader with DataFusion integration
pub struct ParquetReader {
    path: PathBuf,
    schema: Option<SchemaRef>,
    batch_size: usize,
    parallel: bool,
    async_loading: bool,
}

impl ParquetReader {
    /// Create a new ParquetReader
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            schema: None,
            batch_size: 8192,
            parallel: true,
            async_loading: true,
        }
    }

    /// Set the schema for this reader
    pub fn with_schema(mut self, schema: SchemaRef) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Set the batch size for this reader
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Enable or disable parallel loading
    pub fn parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    /// Enable or disable async loading
    pub fn async_loading(mut self, async_loading: bool) -> Self {
        self.async_loading = async_loading;
        self
    }

    /// Read parquet data synchronously
    pub fn read(&self) -> Result<Vec<RecordBatch>> {
        // Check if path exists
        if !self.path.exists() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Path not found: {}", self.path.display()),
            )));
        }

        // Use tokio runtime for async operation if needed
        if self.async_loading {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| IdsError::Data(format!("Failed to create runtime: {e}")))?;

            runtime.block_on(self.read_async())
        } else {
            let ctx = SessionContext::new();
            let _config = ctx.runtime_env().clone();

            // Note: batch_size configuration is handled differently in DataFusion 47.0.0
            // Configure ctx with batch size if needed

            // Create read options
            let read_options = if let Some(schema) = &self.schema {
                ParquetReadOptions::default().schema(&schema)
            } else {
                ParquetReadOptions::default()
            };

            // Register as a table and collect results
            let df = ctx.read_parquet(self.path.to_string_lossy().to_string(), read_options);

            // Use tokio to run async operation
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| IdsError::Data(format!("Failed to create runtime: {e}")))?;

            let result = runtime.block_on(async {
                let df = df.await?;
                df.collect().await
            })?;

            Ok(result)
        }
    }

    /// Read parquet data asynchronously
    pub async fn read_async(&self) -> Result<Vec<RecordBatch>> {
        // Create DataFusion context
        let ctx = SessionContext::new();

        // Note: batch_size configuration is handled differently in DataFusion 47.0.0
        // Configure ctx with batch size if needed

        // Create read options
        let read_options = if let Some(schema) = &self.schema {
            ParquetReadOptions::default().schema(&schema)
        } else {
            ParquetReadOptions::default()
        };

        // Register as a table and collect results
        let df = ctx
            .read_parquet(self.path.to_string_lossy().to_string(), read_options)
            .await?;

        let result = df.collect().await?;
        Ok(result)
    }

    /// Read parquet data with PNR filter
    pub async fn read_with_pnr_filter(&self, pnr_filter: &PnrFilter) -> Result<Vec<RecordBatch>> {
        // Create DataFusion context
        let ctx = SessionContext::new();

        // Note: batch_size configuration is handled differently in DataFusion 47.0.0
        // Configure ctx with batch size if needed

        // Create read options
        let read_options = if let Some(schema) = &self.schema {
            ParquetReadOptions::default().schema(&schema)
        } else {
            ParquetReadOptions::default()
        };

        // Read as DataFrame
        let mut df = ctx
            .read_parquet(self.path.to_string_lossy().to_string(), read_options)
            .await?;

        // Apply PNR filter
        if pnr_filter.is_direct_filter() {
            // Convert HashSet to literals for the IN expression
            let pnr_values: Vec<Expr> = pnr_filter
                .pnrs()
                .iter()
                .map(|pnr| lit(pnr.clone()))
                .collect();

            // Create IN expression
            if !pnr_values.is_empty() {
                df = df.filter(col("PNR").in_list(pnr_values, false))?;
            }
        } else if let Some(relation_col) = pnr_filter.relation_column() {
            // Convert HashSet to literals for the IN expression
            let pnr_values: Vec<Expr> = pnr_filter
                .pnrs()
                .iter()
                .map(|pnr| lit(pnr.clone()))
                .collect();

            // Create IN expression on relation column
            if !pnr_values.is_empty() {
                df = df.filter(col(relation_col).in_list(pnr_values, false))?;
            }
        }

        // Collect and return results
        let result = df.collect().await?;
        Ok(result)
    }

    /// List all parquet files in a directory
    pub fn list_parquet_files(dir_path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        let dir_path = dir_path.as_ref();
        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory not found: {}", dir_path.display()),
            )));
        }

        let mut result = Vec::new();
        for entry in std::fs::read_dir(dir_path).map_err(|e| {
            IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("Failed to read directory {}: {}", dir_path.display(), e),
            ))
        })? {
            let entry = entry.map_err(|e| {
                IdsError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to read directory entry: {e}"),
                ))
            })?;

            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "parquet") {
                result.push(path);
            }
        }

        Ok(result)
    }
}

/// Load parquet files from a directory using DataFusion
pub async fn load_parquet_directory(
    dir_path: impl AsRef<Path>,
    schema: Option<SchemaRef>,
    pnr_filter: Option<&PnrFilter>,
) -> Result<Vec<RecordBatch>> {
    let dir_path = dir_path.as_ref();

    // Create DataFusion context
    let ctx = SessionContext::new();

    // Create read options
    // Getting a reference to the schema to address lifetime issues
    let read_options = match schema {
        Some(schema) => ParquetReadOptions::default().schema(schema.as_ref()),
        None => ParquetReadOptions::default(),
    };

    // Read parquet files into DataFrame
    let mut df = ctx
        .read_parquet(dir_path.to_string_lossy().to_string(), read_options)
        .await?;

    // Apply PNR filter if provided
    if let Some(filter) = pnr_filter {
        if filter.is_direct_filter() {
            // Convert HashSet to literals for the IN expression
            let pnr_values: Vec<Expr> = filter.pnrs().iter().map(|pnr| lit(pnr.clone())).collect();

            // Create IN expression
            if !pnr_values.is_empty() {
                df = df.filter(col("PNR").in_list(pnr_values, false))?;
            }
        } else if let Some(relation_col) = filter.relation_column() {
            // Convert HashSet to literals for the IN expression
            let pnr_values: Vec<Expr> = filter.pnrs().iter().map(|pnr| lit(pnr.clone())).collect();

            // Create IN expression on relation column
            if !pnr_values.is_empty() {
                df = df.filter(col(relation_col).in_list(pnr_values, false))?;
            }
        }
    }

    // Collect and return results
    let result = df.collect().await?;
    Ok(result)
}
