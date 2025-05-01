use crate::data::io::datafusion_utils::{create_optimized_context, register_listing_table, filter_by_pnrs};
use crate::error::{IdsError, Result};
use arrow::datatypes::SchemaRef;
use datafusion::execution::context::SessionContext;
use datafusion::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A registry loader that uses DataFusion to load registry data
pub struct RegistryLoader {
    /// Base directory for registry data
    base_dir: PathBuf,
    /// Optional filter for PNRs
    pnr_filter: Option<HashSet<String>>,
    /// Schema for the registry
    schema: SchemaRef,
    /// DataFusion session context
    ctx: SessionContext,
    /// Column name for PNR
    pnr_column: String,
}

impl RegistryLoader {
    /// Create a new registry loader
    pub fn new(
        base_dir: impl AsRef<Path>,
        schema: SchemaRef,
        pnr_column: &str,
        pnr_filter: Option<HashSet<String>>,
    ) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            pnr_filter,
            schema,
            ctx: create_optimized_context(),
            pnr_column: pnr_column.to_string(),
        }
    }
    
    /// Load the registry data
    pub async fn load(&self, table_name: &str) -> Result<DataFrame> {
        // Register the listing table
        register_listing_table(
            &self.ctx,
            table_name,
            &self.base_dir,
            self.schema.clone(),
            vec![],  // No partition columns
        ).await?;
        
        // Get the DataFrame
        let mut df = self.ctx.table(table_name).await?;
        
        // Apply PNR filter if provided
        if let Some(pnrs) = &self.pnr_filter {
            df = filter_by_pnrs(df, pnrs, &self.pnr_column)?;
        }
        
        Ok(df)
    }
    
    /// Load registry data with additional filters
    pub async fn load_with_filters(&self, table_name: &str, filters: Vec<Expr>) -> Result<DataFrame> {
        // Load the basic DataFrame
        let mut df = self.load(table_name).await?;
        
        // Apply additional filters
        for filter in filters {
            df = df.filter(filter)?;
        }
        
        Ok(df)
    }
    
    /// Count the number of records in the registry
    pub async fn count(&self, table_name: &str) -> Result<usize> {
        let df = self.load(table_name).await?;
        let count_df = df.aggregate(vec![], vec![count(lit(1))])?;
        let batches = count_df.collect().await?;
        
        if batches.is_empty() || batches[0].num_rows() == 0 {
            return Ok(0);
        }
        
        // Get the count from the first column of the first row
        let count_scalar = batches[0].column(0).get_value(0);
        let count = count_scalar.to_string().parse::<usize>().map_err(|e| {
            IdsError::Validation(format!("Failed to parse count as usize: {e}"))
        })?;
        
        Ok(count)
    }
    
    /// Get the schema for this registry
    pub fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
    
    /// Get the session context
    pub fn context(&self) -> &SessionContext {
        &self.ctx
    }
    
    /// Get a mutable reference to the session context
    pub fn context_mut(&mut self) -> &mut SessionContext {
        &mut self.ctx
    }
}