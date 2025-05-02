use crate::error::Result;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::catalog::MemTable;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::listing::{ListingOptions, ListingTable, ListingTableConfig, ListingTableUrl};
use datafusion::datasource::TableProvider;
use datafusion::error::DataFusionError;
use datafusion::execution::context::{SessionConfig, SessionContext};
use datafusion::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Create a new `DataFusion` session context with optimized settings
#[must_use] pub fn create_optimized_context() -> SessionContext {
    let config = SessionConfig::new()
        .with_target_partitions(4)
        .with_batch_size(8192);
    SessionContext::new_with_config(config)
}

/// Create a `ListingTable` from a directory of parquet files
pub fn create_listing_table(
    dir_path: impl AsRef<Path>,
    schema: SchemaRef,
    table_partition_cols: Vec<String>,
    file_extension: &str,
) -> Result<Arc<dyn TableProvider>> {
    let path_str = dir_path.as_ref().to_string_lossy().to_string();
    
    // Create a URL for the listing table
    let url = ListingTableUrl::parse(path_str)?;
    
    // Create parquet format with optimized settings
    let parquet_format = ParquetFormat::default()
        .with_enable_pruning(true);
    
    // Create listing options with partition columns
    let mut options = ListingOptions::new(Arc::new(parquet_format))
        .with_file_extension(file_extension)
        .with_target_partitions(4);
        
    // Add partition columns to listing options directly
    if !table_partition_cols.is_empty() {
        // Convert String vec to (String, DataType::Utf8) tuples for table partition columns
        let partition_cols_with_types: Vec<(String, arrow::datatypes::DataType)> = 
            table_partition_cols.into_iter()
            .map(|col| (col, arrow::datatypes::DataType::Utf8))
            .collect();
            
        options = options.with_table_partition_cols(partition_cols_with_types);
    }
        
    // Create config with the options that include partition columns
    let config = ListingTableConfig::new(url)
        .with_schema(schema)
        .with_listing_options(options);
        
    // Create the listing table
    let table = ListingTable::try_new(config)?;
    
    Ok(Arc::new(table))
}

/// Create a `MemTable` from record batches
pub fn create_mem_table(batches: &[RecordBatch]) -> Result<Arc<dyn TableProvider>> {
    if batches.is_empty() {
        return Err(DataFusionError::Execution("Cannot create table from empty batches".to_string()).into());
    }
    
    let provider = MemTable::try_new(batches[0].schema(), vec![batches.to_vec()])?;
    Ok(Arc::new(provider))
}

/// Find all parquet files in a directory (recursively)
pub fn find_parquet_files(dir_path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let dir_path = dir_path.as_ref();
    if !dir_path.exists() || !dir_path.is_dir() {
        return Ok(Vec::new());
    }

    let mut result = Vec::new();
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
            result.push(path);
        } else if path.is_dir() {
            // Recursively scan subdirectories
            let sub_files = find_parquet_files(&path)?;
            result.extend(sub_files);
        }
    }

    Ok(result)
}

/// Register a listing table with the session context
pub async fn register_listing_table(
    ctx: &SessionContext,
    table_name: &str,
    dir_path: impl AsRef<Path>,
    schema: SchemaRef,
    partition_cols: Vec<String>,
) -> Result<()> {
    let table = create_listing_table(dir_path, schema, partition_cols, "parquet")?;
    ctx.register_table(table_name, table)?;
    Ok(())
}

/// Filter dataframe by a list of PNRs
pub fn filter_by_pnrs(
    df: DataFrame,
    pnrs: &HashSet<String>,
    pnr_column: &str,
) -> Result<DataFrame> {
    if pnrs.is_empty() {
        return Ok(df);
    }

    // Convert HashSet to a Vec for IN expression
    let pnr_values: Vec<_> = pnrs.iter().map(|pnr| lit(pnr.clone())).collect();
    
    // Create IN expression
    let filter_expr = col(pnr_column).in_list(pnr_values, false);
    
    // Apply filter
    Ok(df.filter(filter_expr)?)
}

/// Create and register multiple listing tables from a base directory
pub async fn register_tables_from_directories(
    ctx: &SessionContext,
    base_dir: impl AsRef<Path>,
    schemas: &[(&str, SchemaRef, Vec<String>)],
) -> Result<()> {
    for (name, schema, partition_cols) in schemas {
        let dir_path = base_dir.as_ref().join(name);
        if dir_path.exists() && dir_path.is_dir() {
            register_listing_table(ctx, name, dir_path, schema.clone(), partition_cols.clone()).await?;
        }
    }
    
    Ok(())
}