use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::akm::AkmSchema;
use crate::data::schema::traits::RegistrySchema;
use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::Path;

/// AKM registry loader for employment information
pub struct AkmRegister;

#[async_trait::async_trait]
impl RegisterLoader for AkmRegister {
    type SchemaType = AkmSchema;

    /// Get the name of the register
    fn register_name(&self) -> &'static str {
        "AKM"
    }

    /// Load records from the AKM register
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing the AKM parquet files
    /// * `pnr_filter` - Optional filter to only load data for specific PNRs
    ///
    /// # Returns
    /// * `Result<Vec<RecordBatch>>` - Arrow record batches containing the loaded data
    async fn load(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<Vec<RecordBatch>> {
        // Create a context
        let ctx = SessionContext::new();
        
        // Get schema
        let schema = Self::SchemaType::schema_arc();
        
        // Register the Parquet file/directory as a table
        let table_name = self.register_name().to_lowercase();
        
        // Verify the path exists
        let path = Path::new(base_path);
        if !path.exists() {
            return Err(IdsError::Validation(format!(
                "Path does not exist: {}",
                path.display()
            )));
        }
        
        // Register the parquet data
        ctx.register_parquet(
            &table_name,
            base_path,
            ParquetReadOptions::default().schema(schema.as_ref()),
        )
        .await?;
        
        // Get the table as a dataframe
        let mut df = ctx.table(&table_name).await?;
        
        // Apply filter if provided
        if let Some(filter) = pnr_filter {
            if filter.is_direct_filter() {
                // Convert HashSet to a list of literals for IN expression
                let pnr_list: Vec<Expr> = filter
                    .pnrs()
                    .iter()
                    .map(|pnr| lit(pnr.clone()))
                    .collect();
                
                if !pnr_list.is_empty() {
                    // Create filter: PNR IN (pnr1, pnr2, ...), false = not negated
                    df = df.filter(col("PNR").in_list(pnr_list, false))?;
                }
            } else if let Some(relation_col) = filter.relation_column() {
                // Similar approach for relation filtering
                let pnr_list: Vec<Expr> = filter
                    .pnrs()
                    .iter()
                    .map(|pnr| lit(pnr.clone()))
                    .collect();
                
                if !pnr_list.is_empty() {
                    // Create filter: relation_col IN (pnr1, pnr2, ...), false = not negated
                    df = df.filter(col(relation_col).in_list(pnr_list, false))?;
                }
            }
        }
        
        // Execute and collect the results
        let result = df.collect().await?;
        Ok(result)
    }
    
    /// Create a DataFusion context with this registry loaded as a table
    /// This overrides the default implementation for better performance
    async fn create_context(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<SessionContext> {
        let ctx = SessionContext::new();
        let schema = Self::SchemaType::schema_arc();
        let table_name = self.register_name().to_lowercase();
        
        // Register the parquet data
        ctx.register_parquet(
            &table_name,
            base_path,
            ParquetReadOptions::default().schema(schema.as_ref()),
        )
        .await?;
        
        // If we have a filter, create a filtered view
        if let Some(filter) = pnr_filter {
            if filter.is_direct_filter() {
                // Convert HashSet to a list of literals for IN expression
                let pnr_list: Vec<String> = filter
                    .pnrs()
                    .iter()
                    .map(|pnr| format!("'{pnr}'"))
                    .collect();
                
                if !pnr_list.is_empty() {
                    let pnrs_list = pnr_list.join(",");
                    let sql = format!(
                        "CREATE OR REPLACE VIEW {table_name}_filtered AS 
                         SELECT * FROM {table_name} 
                         WHERE PNR IN ({pnrs_list})"
                    );
                    
                    ctx.sql(&sql).await?;
                    
                    // Replace the original table with the filtered view
                    let filtered_view = format!(
                        "CREATE OR REPLACE TABLE {table_name} AS 
                         SELECT * FROM {table_name}_filtered"
                    );
                    ctx.sql(&filtered_view).await?;
                }
            } else if let Some(relation_col) = filter.relation_column() {
                // Similar approach for relation filtering
                let pnr_list: Vec<String> = filter
                    .pnrs()
                    .iter()
                    .map(|pnr| format!("'{pnr}'"))
                    .collect();
                
                if !pnr_list.is_empty() {
                    let pnrs_list = pnr_list.join(",");
                    let sql = format!(
                        "CREATE OR REPLACE VIEW {table_name}_filtered AS 
                         SELECT * FROM {table_name} 
                         WHERE {relation_col} IN ({pnrs_list})"
                    );
                    
                    ctx.sql(&sql).await?;
                    
                    // Replace the original table with the filtered view
                    let filtered_view = format!(
                        "CREATE OR REPLACE TABLE {table_name} AS 
                         SELECT * FROM {table_name}_filtered"
                    );
                    ctx.sql(&filtered_view).await?;
                }
            }
        }
        
        Ok(ctx)
    }
}
