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
    #[must_use] pub const fn new(pnrs: HashSet<String>) -> Self {
        Self {
            pnrs,
            direct_filter: true,
            relation_column: None,
        }
    }

    /// Create a PNR filter with relation
    #[must_use] pub fn with_relation(pnrs: HashSet<String>, relation_column: &str) -> Self {
        Self {
            pnrs,
            direct_filter: false,
            relation_column: Some(relation_column.to_string()),
        }
    }

    /// Get the PNRs in this filter
    #[must_use] pub const fn pnrs(&self) -> &HashSet<String> {
        &self.pnrs
    }

    /// Get the relation column if any
    #[must_use] pub fn relation_column(&self) -> Option<&str> {
        self.relation_column.as_deref()
    }

    /// Check if this is a direct filter
    #[must_use] pub const fn is_direct_filter(&self) -> bool {
        self.direct_filter
    }
    
    /// Convert to a data::filter::pnr::PnrFilter
    #[must_use] pub fn to_io_filter(&self) -> crate::data::filter::pnr::PnrFilter {
        if self.is_direct_filter() {
            crate::data::filter::pnr::PnrFilter::new(self.pnrs.clone())
        } else if let Some(relation_col) = self.relation_column() {
            crate::data::filter::pnr::PnrFilter::with_relation(self.pnrs.clone(), relation_col)
        } else {
            // Fallback to direct filter
            crate::data::filter::pnr::PnrFilter::new(self.pnrs.clone())
        }
    }
}

/// Base trait for registry loaders
#[async_trait::async_trait]
pub trait RegisterLoader: Send + Sync + 'static {
    /// The schema type for this registry
    type SchemaType: RegistrySchema;

    /// Get the name of the register
    fn register_name(&self) -> &'static str;

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
            self.register_name().to_lowercase(),
            base_path,
            ParquetReadOptions::default().schema(schema.as_ref()),
        )
        .await?;

        // Apply PNR filter if provided
        if let Some(filter) = pnr_filter {
            // Create SQL execution for the filter
            let table_name = self.register_name().to_lowercase();
            if filter.is_direct_filter() {
                // Create PNR IN list SQL
                let pnrs_list = filter
                    .pnrs()
                    .iter()
                    .map(|p| format!("'{p}'"))
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
                    .map(|p| format!("'{p}'"))
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
        let table_name = self.register_name().to_lowercase();

        let df = ctx.table(&table_name).await?;
        let result = df.collect().await?;
        Ok(result)
    }

    /// Get the schema for this registry
    fn get_schema(&self) -> SchemaRef {
        Self::SchemaType::schema_arc()
    }
}

// We'll use a trait object for any type of register loader
pub type AnyRegisterLoader = Box<dyn std::any::Any + Send + Sync>;

/// Dynamic registry loader trait, avoids using async methods that aren't dyn-compatible
pub trait DynamicLoader: Send + Sync + 'static {
    /// Get the name of the register
    fn name(&self) -> &'static str;
    
    /// Get the schema for this registry
    fn schema(&self) -> SchemaRef;
}
