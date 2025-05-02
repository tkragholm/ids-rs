use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::idan::IdanSchema;
use crate::data::schema::traits::RegistrySchema;
use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::Path;

/// IDAN registry loader for Danish employment statistics
pub struct IdanRegister;

#[async_trait::async_trait]
impl RegisterLoader for IdanRegister {
    type SchemaType = IdanSchema;

    /// Get the name of the register
    fn register_name(&self) -> &'static str {
        "IDAN"
    }

    /// Load records from the IDAN register
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing the IDAN parquet files
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
}