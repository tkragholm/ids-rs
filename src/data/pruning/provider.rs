use super::statistics::RegistryPruningStatistics;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::datasource::listing::PartitionedFile;
use datafusion::datasource::object_store::ObjectStoreUrl;
use datafusion::datasource::physical_plan::{FileScanConfigBuilder, ParquetSource};
use datafusion::datasource::source::DataSourceExec;
use datafusion::datasource::{TableProvider, TableType};
use datafusion::logical_expr::{Expr, TableProviderFilterPushDown};
use std::any::Any;
use std::path::PathBuf;
use std::sync::Arc;

/// A custom table provider with pruning support
#[derive(Debug)]
pub struct PrunableTableProvider {
    schema: SchemaRef,
    statistics: Arc<RegistryPruningStatistics>,
    file_list: Vec<PathBuf>,
}

impl PrunableTableProvider {
    /// Create a new prunable table provider
    #[must_use] pub fn new(
        schema: SchemaRef,
        statistics: RegistryPruningStatistics,
        file_list: Vec<PathBuf>,
    ) -> Self {
        Self {
            schema,
            statistics: Arc::new(statistics),
            file_list,
        }
    }
}

#[async_trait::async_trait]
impl TableProvider for PrunableTableProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    fn table_type(&self) -> TableType {
        TableType::Base
    }

    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> datafusion::error::Result<Vec<TableProviderFilterPushDown>> {
        // We can push down filters for columns we have statistics for
        let mut result = Vec::with_capacity(filters.len());

        for &filter in filters {
            // Check if this is a filter we can push down
            match filter {
                Expr::BinaryExpr(binary) => {
                    // Extract column name if left side is a column
                    if let Expr::Column(col) = &*binary.left {
                        let column_name = &col.name;

                        // Check if we have statistics for this column
                        if self.statistics.min_values.contains_key(column_name)
                            || self.statistics.max_values.contains_key(column_name)
                        {
                            result.push(TableProviderFilterPushDown::Exact);
                            continue;
                        }
                    }
                }
                Expr::InList(in_list) => {
                    if let Expr::Column(col) = &*in_list.expr {
                        let column_name = &col.name;

                        // We have special handling for PNR columns
                        if column_name == "PNR" || column_name == "CPR" {
                            result.push(TableProviderFilterPushDown::Exact);
                            continue;
                        }
                    }
                }
                _ => {}
            }

            result.push(TableProviderFilterPushDown::Inexact);
        }

        Ok(result)
    }

    // Implement a TableProvider that can scan files with pruning
    async fn scan(
        &self,
        _state: &dyn datafusion::catalog::Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> datafusion::error::Result<Arc<dyn datafusion::physical_plan::ExecutionPlan>> {
        use std::fs;

        // Convert filters to references for compatibility with existing code
        let filter_refs: Vec<&Expr> = filters.iter().collect();

        // Filter the file list based on filters
        let files = if filters.is_empty() {
            self.file_list.clone()
        } else {
            // For each filter, get files that pass it
            let mut filtered_files = self.file_list.clone();
            filtered_files.retain(|file| {
                // For each file, check if the statistics suggest it should be processed
                let file_stats = self
                    .statistics
                    .files
                    .iter()
                    .find(|stats| stats.path == *file);

                if let Some(stats) = file_stats {
                    for filter in &filter_refs {
                        if !stats.should_process(filter) {
                            return false;
                        }
                    }
                    true
                } else {
                    // If we don't have statistics, process the file
                    true
                }
            });
            filtered_files
        };

        if files.is_empty() {
            // Return an empty plan
            let empty_schema = self.schema.clone();
            let empty_batch = RecordBatch::new_empty(empty_schema.clone());
            let provider =
                datafusion::datasource::MemTable::try_new(empty_schema, vec![vec![empty_batch]])?;
            
            // Create a new empty array of &Expr for compatibility with the updated API
            let empty_filters: Vec<Expr> = Vec::new();
            let plan = provider.scan(_state, projection, &empty_filters, limit).await?;

            return Ok(plan);
        }

        // Create the format with predicate pushdown
        let format = ParquetSource::default()
            .with_enable_page_index(true) // Enable page-level pruning
            .with_pushdown_filters(true); // Enable predicate pushdown
        let format_arc = Arc::new(format);

        // Create FileScanConfig using builder
        let url = ObjectStoreUrl::parse("file://")?;

        // Start building the config
        let mut config_builder = FileScanConfigBuilder::new(url, self.schema.clone(), format_arc);

        // Add each file individually with size info for better statistics
        for file_path in &files {
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
