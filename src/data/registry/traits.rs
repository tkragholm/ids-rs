use crate::data::schema::traits::RegistrySchema;
use crate::error::Result;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::collections::HashSet;

/// A type for PNR filtering
pub struct PnrFilter {
    pnrs: HashSet<String>,
    direct_filter: bool,
    relation_column: Option<String>,
}

impl PnrFilter {
    /// Create a new PNR filter
    pub fn new(pnrs: HashSet<String>) -> Self {
        Self {
            pnrs,
            direct_filter: true,
            relation_column: None,
        }
    }

    /// Create a PNR filter with relation
    pub fn with_relation(pnrs: HashSet<String>, relation_column: &str) -> Self {
        Self {
            pnrs,
            direct_filter: false,
            relation_column: Some(relation_column.to_string()),
        }
    }

    /// Get the PNRs in this filter
    pub fn pnrs(&self) -> &HashSet<String> {
        &self.pnrs
    }

    /// Get the relation column if any
    pub fn relation_column(&self) -> Option<&str> {
        self.relation_column.as_deref()
    }

    /// Check if this is a direct filter
    pub fn is_direct_filter(&self) -> bool {
        self.direct_filter
    }
}

/// Base trait for registry loaders
#[async_trait::async_trait]
pub trait RegisterLoader: Send + Sync + 'static {
    /// The schema type for this registry
    type SchemaType: RegistrySchema;

    /// Get the name of the register
    fn register_name() -> &'static str;

    /// Load records from the register (async)
    async fn load(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<Vec<RecordBatch>>;

    /// Create a DataFusion SessionContext with this registry
    async fn create_context(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<SessionContext> {
        let ctx = SessionContext::new();
        let schema = Self::SchemaType::schema_arc();

        // Register source as a table
        ctx.register_parquet(
            Self::register_name().to_lowercase(),
            base_path,
            ParquetReadOptions::default().with_file_schema(schema),
        )
        .await?;

        // Apply PNR filter if provided
        if let Some(filter) = pnr_filter {
            // Create SQL execution for the filter
            let table_name = Self::register_name().to_lowercase();
            if filter.is_direct_filter() {
                // Create PNR IN list SQL
                let pnrs_list = filter
                    .pnrs()
                    .iter()
                    .map(|p| format!("'{}'", p))
                    .collect::<Vec<_>>()
                    .join(",");

                if !pnrs_list.is_empty() {
                    let sql = format!(
                        "
                        CREATE OR REPLACE TABLE {table_name} AS
                        SELECT * FROM {table_name}
                        WHERE PNR IN ({pnrs_list})
                    "
                    );
                    ctx.sql(&sql).await?;
                }
            } else if let Some(relation_col) = filter.relation_column() {
                // Handle relation filtering
                let pnrs_list = filter
                    .pnrs()
                    .iter()
                    .map(|p| format!("'{}'", p))
                    .collect::<Vec<_>>()
                    .join(",");

                if !pnrs_list.is_empty() {
                    let sql = format!(
                        "
                        CREATE OR REPLACE TABLE {table_name} AS
                        SELECT * FROM {table_name}
                        WHERE {relation_col} IN ({pnrs_list})
                    "
                    );
                    ctx.sql(&sql).await?;
                }
            }
        }

        Ok(ctx)
    }

    /// Default implementation of load using DataFusion
    async fn default_load(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<Vec<RecordBatch>> {
        let ctx = self.create_context(base_path, pnr_filter).await?;
        let table_name = Self::register_name().to_lowercase();

        let df = ctx.table(&table_name).await?;
        let result = df.collect().await?;
        Ok(result)
    }

    /// Get the schema for this registry
    fn get_schema() -> SchemaRef {
        Self::SchemaType::schema_arc()
    }
}
