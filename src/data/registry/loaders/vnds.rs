use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::vnds::{VndsSchema, VndsStandardizedSchema, MigrationType};
use crate::data::schema::traits::RegistrySchema;
use crate::error::{IdsError, Result};
use crate::utils::date_utils;
use arrow::array::{Array, ArrayRef, Date32Array, StringArray};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::Path;
use std::sync::Arc;

/// Registry loader for the Danish Migration Registry (VNDS)
pub struct VndsRegister;

#[async_trait::async_trait]
impl RegisterLoader for VndsRegister {
    type SchemaType = VndsSchema;

    /// Get the name of the register
    fn register_name(&self) -> &'static str {
        "VNDS"
    }

    /// Load records from the VNDS register
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing the VNDS parquet files
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
        let batches = df.collect().await?;
        
        // Standardize the batches
        let standardized_batches = batches
            .into_iter()
            .map(|batch| standardize_vnds_batch(&batch))
            .collect::<Result<Vec<_>>>()?;
        
        Ok(standardized_batches)
    }
}

/// Convert a VNDS batch to standardized format
fn standardize_vnds_batch(batch: &RecordBatch) -> Result<RecordBatch> {
    // Extract columns
    let pnr_col = batch
        .column_by_name("PNR")
        .ok_or_else(|| IdsError::Data("PNR column not found in VNDS data".to_string()))?;

    let code_col = batch
        .column_by_name("INDUD_KODE")
        .ok_or_else(|| IdsError::Data("INDUD_KODE column not found in VNDS data".to_string()))?;

    let date_col = batch
        .column_by_name("HAEND_DATO")
        .ok_or_else(|| IdsError::Data("HAEND_DATO column not found in VNDS data".to_string()))?;

    let pnr_array = pnr_col.clone();

    // Convert migration code to standardized form
    let code_array = if let Some(string_array) = code_col.as_any().downcast_ref::<StringArray>() {
        let mut migration_types = Vec::with_capacity(string_array.len());

        for i in 0..string_array.len() {
            if string_array.is_null(i) {
                migration_types.push(None);
            } else {
                let code = string_array.value(i);
                if let Some(migration_type) = MigrationType::from_code(code) {
                    migration_types.push(Some(migration_type.as_str().to_string()));
                } else {
                    migration_types.push(None);
                }
            }
        }

        Arc::new(StringArray::from(migration_types)) as ArrayRef
    } else {
        return Err(IdsError::Data(
            "INDUD_KODE column is not a StringArray".to_string(),
        ));
    };

    // Convert date column to Date32
    let date_array = if let Some(string_array) = date_col.as_any().downcast_ref::<StringArray>() {
        // Parse dates and create Date32Array
        let mut parsed_dates = Vec::with_capacity(string_array.len());
        let mut nulls = Vec::with_capacity(string_array.len());

        for i in 0..string_array.len() {
            if string_array.is_null(i) {
                parsed_dates.push(0); // Placeholder value for null
                nulls.push(i);
            } else {
                let date_str = string_array.value(i);
                if let Some(date) = date_utils::parse_danish_date(date_str) {
                    parsed_dates.push(date_utils::date_to_days_since_epoch(date));
                } else {
                    parsed_dates.push(0); // Placeholder value for null
                    nulls.push(i);
                }
            }
        }

        // Create a null buffer
        let mut null_buffer = vec![true; parsed_dates.len()];
        for &i in &nulls {
            null_buffer[i] = false;
        }

        // Create Date32Array with the values and null buffer
        let date_array = Date32Array::new(
            parsed_dates.into(),
            Some(arrow::buffer::NullBuffer::new(
                arrow::buffer::BooleanBuffer::from(null_buffer),
            )),
        );
        Arc::new(date_array) as ArrayRef
    } else {
        return Err(IdsError::Data(
            "HAEND_DATO column is not a StringArray".to_string(),
        ));
    };

    // Create standardized batch
    let standardized_batch = RecordBatch::try_new(
        Arc::new(VndsStandardizedSchema::schema()),
        vec![pnr_array, code_array, date_array],
    )?;

    Ok(standardized_batch)
}