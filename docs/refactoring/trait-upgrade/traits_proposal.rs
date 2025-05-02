use crate::error::Result;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

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
}

/// Object-safe trait for registry loaders (no associated types)
pub trait AsyncRegisterLoader: Send + Sync + 'static {
    /// Get the name of the register
    fn register_name(&self) -> &'static str;

    /// Get the schema for this registry
    fn get_schema(&self) -> SchemaRef;

    /// Load records from the register (async)
    fn load<'a>(
        &'a self,
        base_path: &'a str,
        pnr_filter: Option<&'a PnrFilter>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RecordBatch>>> + Send + 'a>>;

    /// Create a DataFusion SessionContext with this registry
    fn create_context<'a>(
        &'a self,
        base_path: &'a str,
        pnr_filter: Option<&'a PnrFilter>,
    ) -> Pin<Box<dyn Future<Output = Result<SessionContext>> + Send + 'a>>;
}

/// Type alias for a boxed register loader trait object
pub type BoxedRegisterLoader = Box<dyn AsyncRegisterLoader>;

/// Helper macro to implement async methods for the trait
#[macro_export]
macro_rules! impl_loader_method {
    ($self:ident, $method:ident, $($args:tt)*) => {
        Box::pin(async move {
            $self.$method($($args)*).await
        })
    };
}

/// Helper trait for implementing common loader functionality
pub trait RegisterLoaderImpl: AsyncRegisterLoader {
    /// Default implementation of create_context
    async fn default_create_context(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<SessionContext> {
        let ctx = SessionContext::new();
        let schema = self.get_schema();

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
            let primary_key = self.get_primary_key();
            
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
                        WHERE {primary_key} IN ({pnrs_list})
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
        let ctx = self.default_create_context(base_path, pnr_filter).await?;
        let table_name = self.register_name().to_lowercase();

        let df = ctx.table(&table_name).await?;
        let result = df.collect().await?;
        Ok(result)
    }

    /// Get the primary key field name for this registry
    fn get_primary_key(&self) -> &'static str {
        "PNR" // Default, can be overridden
    }
}

/// Registry factory that returns trait objects
pub struct RegisterLoaderFactory;

impl RegisterLoaderFactory {
    /// Create a registry loader from a registry name
    pub fn from_name(name: &str) -> Result<BoxedRegisterLoader> {
        // Implementation would return appropriate concrete loaders
        // All wrapped as BoxedRegisterLoader trait objects
        todo!("Implement by creating appropriate register loader based on name")
    }
}