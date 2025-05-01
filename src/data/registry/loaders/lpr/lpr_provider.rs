//! LPR table provider for `DataFusion` integration
//!
//! This module contains a custom table provider for LPR data.

use crate::data::registry::loaders::lpr::{LprPaths, LprVersion};
use crate::error::{IdsError, Result};
use arrow::datatypes::SchemaRef;
use datafusion::catalog::Session;
use datafusion::datasource::listing::{ListingTableUrl, PartitionedFile};
use datafusion::datasource::object_store::ObjectStoreUrl;
use datafusion::datasource::physical_plan::{FileScanConfigBuilder, ParquetSource};
use datafusion::datasource::source::DataSourceExec;
use datafusion::datasource::{TableProvider, TableType};
use datafusion::error::Result as DFResult;
use datafusion::logical_expr::TableProviderFilterPushDown;
use datafusion::logical_expr::{Expr, LogicalPlan};

use datafusion::physical_plan::ExecutionPlan;
use std::any::Any;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

/// Alias for `DataFusion`'s filter pushdown enum
type FilterPushdown = TableProviderFilterPushDown;

/// Custom table provider for LPR data
#[derive(Debug)]
pub struct LprTableProvider {
    /// LPR version
    version: LprVersion,
    /// Paths to LPR files
    paths: LprPaths,
    /// Schema
    schema: SchemaRef,
    /// PNR filter
    pnr_filter: Option<HashSet<String>>,
}

impl LprTableProvider {
    /// Create a new LPR table provider
    #[must_use] pub const fn new(
        version: LprVersion,
        paths: LprPaths,
        schema: SchemaRef,
        pnr_filter: Option<HashSet<String>>,
    ) -> Self {
        Self {
            version,
            paths,
            schema,
            pnr_filter,
        }
    }

    /// Find LPR files based on the version and paths
    fn find_lpr_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        match self.version {
            LprVersion::V2 => {
                // Find admin files
                if let Some(admin_path) = &self.paths.admin_path {
                    files.extend(find_parquet_files(admin_path)?);
                }

                // Find diagnosis files
                if let Some(diag_path) = &self.paths.diag_path {
                    files.extend(find_parquet_files(diag_path)?);
                }

                // Find procedure files
                if let Some(proc_path) = &self.paths.proc_path {
                    files.extend(find_parquet_files(proc_path)?);
                }
            }
            LprVersion::V3 => {
                // Find kontakter files
                if let Some(kontakter_path) = &self.paths.kontakter_path {
                    files.extend(find_parquet_files(kontakter_path)?);
                }

                // Find diagnoser files
                if let Some(diagnoser_path) = &self.paths.diagnoser_path {
                    files.extend(find_parquet_files(diagnoser_path)?);
                }

                // Find procedurer files
                if let Some(procedurer_path) = &self.paths.procedurer_path {
                    files.extend(find_parquet_files(procedurer_path)?);
                }
            }
        }

        Ok(files)
    }
}

/// Helper function to find parquet files in a directory
fn find_parquet_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if !dir.exists() || !dir.is_dir() {
        return Ok(files);
    }

    for entry_result in fs::read_dir(dir).map_err(|e| {
        IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Failed to read directory {}: {}", dir.display(), e),
        ))
    })? {
        let entry = entry_result.map_err(|e| {
            IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read directory entry: {e}"),
            ))
        })?;

        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
            files.push(path);
        }
    }

    Ok(files)
}

#[async_trait::async_trait]
impl TableProvider for LprTableProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    fn table_type(&self) -> TableType {
        TableType::Base
    }

    fn get_table_definition(&self) -> Option<&str> {
        None
    }

    fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>> {
        None
    }

    fn supports_filters_pushdown(&self, filters: &[&Expr]) -> DFResult<Vec<FilterPushdown>> {
        // We can push down PNR filter but not complex filters
        let mut pushdowns = Vec::new();

        for filter in filters {
            if let Expr::BinaryExpr(expr) = *filter {
                if let Expr::Column(col) = expr.left.as_ref() {
                    // LPR v2 uses PNR, LPR v3 uses CPR
                    let pnr_col = match self.version {
                        LprVersion::V2 => "PNR",
                        LprVersion::V3 => "CPR",
                    };

                    if col.name == pnr_col {
                        pushdowns.push(FilterPushdown::Exact);
                    }
                }
            }
        }

        if pushdowns.is_empty() {
            Ok(vec![FilterPushdown::Unsupported])
        } else {
            Ok(pushdowns)
        }
    }

    fn scan(
        &self,
        _state: &dyn Session,
        projection: Option<&Vec<usize>>,
        _filters: &[&Expr],
        limit: Option<usize>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = DFResult<Arc<dyn datafusion::physical_plan::ExecutionPlan>>> + Send>> {
        Box::pin(async move {
        // Find all the parquet files
        let parquet_files = self.find_lpr_files().map_err(|e| {
            datafusion::error::DataFusionError::Execution(format!("Error finding LPR files: {e}"))
        })?;

        if parquet_files.is_empty() {
            return Err(datafusion::error::DataFusionError::Execution(
                "No LPR files found in the specified paths".to_string(),
            ));
        }

        // Create the listing table URL
        let _url = ListingTableUrl::parse(self.paths.base_path.to_string_lossy())?;

        // Create the partitioned files list
        let mut files = Vec::new();
        for file_path in parquet_files {
            // Get the file size for statistics
            let file_size = fs::metadata(&file_path)
                .map(|m| m.len()) // Already u64
                .unwrap_or(0);

            // Convert to string path
            let file_path_str = file_path.to_string_lossy().to_string();

            // Add the file
            files.push(PartitionedFile::new(file_path_str, file_size));
        }

        // In DataFusion 47.0.0, we use DataSourceExec with a FileScanConfig

        // Create the format (ParquetSource as the FileSource implementation)
        let format = ParquetSource::default()
            .with_enable_page_index(true) // Enable page-level pruning
            .with_pushdown_filters(true); // Enable predicate pushdown
        let format_arc = Arc::new(format);

        // Create FileScanConfig using builder
        let url = ObjectStoreUrl::parse(self.paths.base_path.to_string_lossy())?;

        // Start building the config
        let mut config_builder = FileScanConfigBuilder::new(url, self.schema.clone(), format_arc);

        // Add each file individually since with_file takes a single file
        for file in files {
            config_builder = config_builder.with_file(file);
        }

        // Add projection and limit if provided
        if let Some(proj) = projection {
            config_builder = config_builder.with_projection(Some(proj.clone()));
        }

        if let Some(lim) = limit {
            config_builder = config_builder.with_limit(Some(lim));
        }

        // Build the config
        let config = config_builder.build();

        // Create DataSourceExec with the config and return it
        // DataSourceExec implements ExecutionPlan trait
        Ok(DataSourceExec::from_data_source(config) as Arc<dyn datafusion::physical_plan::ExecutionPlan>)
        })
    }
}
