use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::dodsaarsag::{DodsaarsagSchema, DodsaarsagStandardizedSchema};
use crate::data::schema::traits::RegistrySchema;
use crate::error::{IdsError, Result};
use crate::model::icd10::diagnosis_pattern::normalize_diagnosis_code;
use crate::model::icd10::Icd10Chapter;
use arrow::array::{Array, ArrayRef, StringArray};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::Path;
use std::sync::Arc;

/// Registry loader for the Danish Death Cause Registry (DODSAARSAG)
pub struct DodsaarsagRegister;

#[async_trait::async_trait]
impl RegisterLoader for DodsaarsagRegister {
    type SchemaType = DodsaarsagSchema;

    /// Get the name of the register
    fn register_name(&self) -> &'static str {
        "DODSAARSAG"
    }

    /// Load records from the DODSAARSAG register
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing the DODSAARSAG parquet files
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
            .map(|batch| standardize_dodsaarsag_batch(&batch))
            .collect::<Result<Vec<_>>>()?;
        
        Ok(standardized_batches)
    }
}

/// Convert a DODSAARSAG batch to standardized format
fn standardize_dodsaarsag_batch(batch: &RecordBatch) -> Result<RecordBatch> {
    // Extract columns
    let pnr_col = batch
        .column_by_name("PNR")
        .ok_or_else(|| IdsError::Data("PNR column not found in DODSAARSAG data".to_string()))?;

    let cause_col = batch.column_by_name("C_AARSAG").ok_or_else(|| {
        IdsError::Data("C_AARSAG column not found in DODSAARSAG data".to_string())
    })?;

    let condition_col = batch.column_by_name("C_TILSTAND").ok_or_else(|| {
        IdsError::Data("C_TILSTAND column not found in DODSAARSAG data".to_string())
    })?;

    let pnr_array = pnr_col.clone();
    
    // Downcast arrays to StringArray
    let cause_array = cause_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("C_AARSAG column is not a StringArray".to_string()))?;
        
    let condition_array = condition_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("C_TILSTAND column is not a StringArray".to_string()))?;

    // Process and normalize causes and create chapter information
    let mut normalized_causes = Vec::with_capacity(cause_array.len());
    let mut normalized_conditions = Vec::with_capacity(condition_array.len());
    let mut chapters = Vec::with_capacity(cause_array.len());

    for i in 0..cause_array.len() {
        // Process cause code
        if cause_array.is_null(i) {
            normalized_causes.push(None);
            chapters.push(None);
        } else {
            let cause_code = cause_array.value(i);
            if let Some(normalized) = normalize_diagnosis_code(cause_code) {
                normalized_causes.push(Some(normalized.full_code.clone()));

                // Determine ICD-10 chapter
                if let Some(chapter) = Icd10Chapter::from_code(&normalized.full_code) {
                    chapters.push(Some(chapter.description().to_string()));
                } else {
                    chapters.push(None);
                }
            } else {
                normalized_causes.push(Some(cause_code.to_string()));
                chapters.push(None);
            }
        }

        // Process condition code
        if condition_array.is_null(i) {
            normalized_conditions.push(None);
        } else {
            let condition_code = condition_array.value(i);
            if let Some(normalized) = normalize_diagnosis_code(condition_code) {
                normalized_conditions.push(Some(normalized.full_code.clone()));
            } else {
                normalized_conditions.push(Some(condition_code.to_string()));
            }
        }
    }

    // Create ArrayRef objects for the new batch
    let normalized_cause_array = Arc::new(StringArray::from(normalized_causes)) as ArrayRef;
    let normalized_condition_array = Arc::new(StringArray::from(normalized_conditions)) as ArrayRef;
    let chapter_array = Arc::new(StringArray::from(chapters)) as ArrayRef;

    // Create standardized batch
    let standardized_batch = RecordBatch::try_new(
        Arc::new(DodsaarsagStandardizedSchema::schema()),
        vec![
            pnr_array,
            normalized_cause_array,
            normalized_condition_array,
            chapter_array,
        ],
    )?;

    Ok(standardized_batch)
}