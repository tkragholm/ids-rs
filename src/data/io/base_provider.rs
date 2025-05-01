//! Base table provider for consistent DataFusion integration
//!
//! This module provides a common base for TableProvider implementations
//! to ensure consistency across different table providers.

use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::datasource::listing::PartitionedFile;
use datafusion::datasource::object_store::ObjectStoreUrl;
use datafusion::datasource::physical_plan::{FileScanConfigBuilder, ParquetSource};
use datafusion::datasource::source::DataSourceExec;
use datafusion::datasource::{TableProvider, TableType};
use datafusion::logical_expr::{Expr, TableProviderFilterPushDown};
use datafusion::physical_plan::ExecutionPlan;

use std::any::Any;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Base trait for custom table providers
///
/// This trait standardizes common TableProvider functionality
/// while allowing specific implementations to customize filtering
/// and file selection logic.
#[async_trait::async_trait]
pub trait BaseTableProvider: TableProvider + Send + Sync {
    /// Get the schema for this table
    fn get_schema(&self) -> SchemaRef;

    /// Get the list of files to process
    fn get_file_list(&self) -> &[PathBuf];

    /// Filter files based on predicates
    fn filter_files(&self, filters: &[&Expr]) -> Vec<PathBuf>;

    /// Whether this provider supports a specific filter
    fn supports_filter(&self, filter: &Expr) -> TableProviderFilterPushDown;

    /// Create an execution plan from files
    async fn create_execution_plan(
        &self,
        state: &dyn datafusion::catalog::Session,
        projection: Option<&Vec<usize>>,
        files: &[PathBuf],
        limit: Option<usize>,
    ) -> datafusion::error::Result<Arc<dyn ExecutionPlan>> {
        // If no files, return an empty plan
        if files.is_empty() {
            let empty_schema = self.get_schema();
            let empty_batch = RecordBatch::new_empty(empty_schema.clone());
            let provider =
                datafusion::datasource::MemTable::try_new(empty_schema, vec![vec![empty_batch]])?;
            return provider.scan(state, projection, &[], limit).await;
        }

        // Create the format with predicate pushdown
        let format = ParquetSource::default()
            .with_enable_page_index(true) // Enable page-level pruning
            .with_pushdown_filters(true); // Enable predicate pushdown
        let format_arc = Arc::new(format);

        // Create FileScanConfig using builder
        let url = ObjectStoreUrl::parse("file://")?;

        // Start building the config
        let mut config_builder = FileScanConfigBuilder::new(url, self.get_schema(), format_arc);

        // Add each file individually with size info for better statistics
        for file_path in files {
            let file_size = fs::metadata(file_path).map(|m| m.len()).unwrap_or(0);
            let file_path_str = file_path.to_string_lossy().to_string();
            config_builder =
                config_builder.with_file(PartitionedFile::new(file_path_str, file_size));
        }

        // Add projection if provided
        if let Some(proj) = projection {
            config_builder = config_builder.with_projection(Some(proj.clone()));
        }

        // Add limit if provided
        if let Some(lim) = limit {
            config_builder = config_builder.with_limit(Some(lim));
        }

        // Build the config
        let config = config_builder.build();

        // Create DataSourceExec with the config and return it
        Ok(DataSourceExec::from_data_source(config))
    }
}

/// A generic table provider implementation
#[derive(Debug)]
pub struct GenericTableProvider {
    schema: SchemaRef,
    file_list: Vec<PathBuf>,
    filter_columns: HashSet<String>,
}

impl GenericTableProvider {
    /// Create a new generic table provider
    pub fn new(
        schema: SchemaRef,
        file_list: Vec<PathBuf>,
        filter_columns: HashSet<String>,
    ) -> Self {
        Self {
            schema,
            file_list,
            filter_columns,
        }
    }

    /// Create a new provider from a directory of parquet files
    pub fn from_directory(
        dir_path: impl AsRef<Path>,
        schema: SchemaRef,
        filter_columns: HashSet<String>,
    ) -> std::io::Result<Self> {
        let file_list = Self::find_parquet_files(dir_path)?;
        Ok(Self::new(schema, file_list, filter_columns))
    }

    /// Find all parquet files in a directory (recursively)
    pub fn find_parquet_files(dir_path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
        let dir_path = dir_path.as_ref();
        if !dir_path.exists() || !dir_path.is_dir() {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
                result.push(path);
            } else if path.is_dir() {
                // Recursively scan subdirectories
                let sub_files = Self::find_parquet_files(&path)?;
                result.extend(sub_files);
            }
        }

        Ok(result)
    }
}

#[async_trait::async_trait]
impl BaseTableProvider for GenericTableProvider {
    fn get_schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    fn get_file_list(&self) -> &[PathBuf] {
        &self.file_list
    }

    fn filter_files(&self, _filters: &[&Expr]) -> Vec<PathBuf> {
        // Simple implementation - no filtering at file level
        self.file_list.clone()
    }

    fn supports_filter(&self, filter: &Expr) -> TableProviderFilterPushDown {
        // Check if the filter uses a column we support
        match filter {
            Expr::BinaryExpr(binary) => {
                if let Expr::Column(col) = &*binary.left {
                    if self.filter_columns.contains(&col.name) {
                        return TableProviderFilterPushDown::Exact;
                    }
                }
            }
            Expr::InList(in_list) => {
                if let Expr::Column(col) = &*in_list.expr {
                    if self.filter_columns.contains(&col.name) {
                        return TableProviderFilterPushDown::Exact;
                    }
                }
            }
            _ => {}
        }

        TableProviderFilterPushDown::Inexact
    }
}

#[async_trait::async_trait]
impl TableProvider for GenericTableProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.get_schema()
    }

    fn table_type(&self) -> TableType {
        TableType::Base
    }

    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> datafusion::error::Result<Vec<TableProviderFilterPushDown>> {
        let result = filters
            .iter()
            .map(|filter| self.supports_filter(filter))
            .collect();

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn scan(
        &self,
        state: &dyn datafusion::catalog::Session,
        projection: Option<&Vec<usize>>,
        filters: &[&Expr],
        limit: Option<usize>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = datafusion::error::Result<Arc<dyn datafusion::physical_plan::ExecutionPlan>>> + Send>> {        
        Box::pin(async move {
        // Filter files based on predicates
        let filtered_files = self.filter_files(filters);

        // Create execution plan
        self.create_execution_plan(state, projection, &filtered_files, limit)
            .await
        })
    }
}
