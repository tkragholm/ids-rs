use std::collections::HashMap;
use std::sync::Arc;

use arrow::array::{ArrayRef, Date32Array, Float64Array, StringArray};
use arrow::datatypes::{DataType, Schema};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;

use super::Store;
use crate::error::{IdsError, Result};
use crate::model::covariate::{Covariate, CovariateValue};
use crate::model::pnr::Pnr;

/// Arrow schema for the store
pub type ArrowSchema = Arc<Schema>;

/// Arrow-based data store implementation
pub struct ArrowStore {
    /// Record batches
    batches: Vec<RecordBatch>,

    /// Mapping from PNR to batch and row indices
    pnr_to_row: HashMap<String, Vec<(usize, usize)>>, // (batch_idx, row_idx)

    /// Schema
    schema: ArrowSchema,
}

impl ArrowStore {
    /// Create a new Arrow store with the given schema
    #[must_use] pub fn new(schema: ArrowSchema) -> Self {
        Self {
            batches: Vec::new(),
            pnr_to_row: HashMap::new(),
            schema,
        }
    }

    /// Add a record batch to the store
    pub fn add_batch(&mut self, batch: RecordBatch) -> Result<()> {
        // Ensure the batch schema matches our schema
        if !batch
            .schema()
            .fields()
            .iter()
            .zip(self.schema.fields().iter())
            .all(|(a, b)| a.name() == b.name() && a.data_type() == b.data_type())
        {
            return Err(IdsError::Validation(
                "Batch schema does not match store schema".to_string(),
            ));
        }

        // Find the PNR column
        let pnr_col_idx = batch
            .schema()
            .fields()
            .iter()
            .position(|f| f.name() == "PNR")
            .ok_or_else(|| IdsError::Validation("Batch does not contain PNR column".to_string()))?;

        // Get the PNR column as a StringArray
        let pnr_col = batch
            .column(pnr_col_idx)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Validation("PNR column is not a string array".to_string()))?;

        // Index the batch
        let batch_idx = self.batches.len();
        for row_idx in 0..batch.num_rows() {
            let pnr = pnr_col.value(row_idx);
            self.pnr_to_row
                .entry(pnr.to_string())
                .or_default()
                .push((batch_idx, row_idx));
        }

        // Add the batch
        self.batches.push(batch);
        Ok(())
    }

    /// Convert Arrow data to a Covariate
    fn arrow_to_covariate(&self, batch: &RecordBatch, row: usize, name: &str) -> Result<Covariate> {
        // Create a new covariate
        let mut covariate = Covariate::new(name);

        // Iterate through columns and add values
        for col_idx in 0..batch.num_columns() {
            let schema = batch.schema();
            let field = schema.field(col_idx);
            let column = batch.column(col_idx);

            if field.name() == "PNR" || field.name() == "DATE" {
                continue; // Skip metadata columns
            }

            let value = self.extract_value(column, row, field.data_type())?;
            covariate = covariate.with_value(field.name(), value);
        }

        Ok(covariate)
    }

    /// Extract a value from an Arrow column
    fn extract_value(
        &self,
        column: &ArrayRef,
        row: usize,
        data_type: &DataType,
    ) -> Result<CovariateValue> {
        if column.is_null(row) {
            return Ok(CovariateValue::None);
        }

        match data_type {
            DataType::Float64 => {
                let array = column.as_any().downcast_ref::<Float64Array>().unwrap();
                Ok(CovariateValue::Numeric(array.value(row)))
            }
            DataType::Utf8 => {
                let array = column.as_any().downcast_ref::<StringArray>().unwrap();
                Ok(CovariateValue::Categorical(array.value(row).to_string()))
            }
            DataType::Date32 => {
                let array = column.as_any().downcast_ref::<Date32Array>().unwrap();
                let days = array.value(row);
                let date = chrono::NaiveDate::from_num_days_from_ce_opt(days + 719163).unwrap_or_default(); // Adjust for Arrow epoch
                Ok(CovariateValue::Date(date))
            }
            _ => Err(IdsError::Validation(format!(
                "Unsupported data type: {data_type:?}"
            ))),
        }
    }
}

impl Store for ArrowStore {
    fn get_covariate(&self, pnr: &Pnr, name: &str, date: NaiveDate) -> Result<Option<Covariate>> {
        // Find records for this PNR
        let positions = match self.pnr_to_row.get(pnr.value()) {
            Some(positions) => positions,
            None => return Ok(None),
        };

        // Check each position for the date and covariate
        for &(batch_idx, row_idx) in positions {
            let batch = &self.batches[batch_idx];

            // Check date column
            let date_col_idx = batch
                .schema()
                .fields()
                .iter()
                .position(|f| f.name() == "DATE")
                .ok_or_else(|| {
                    IdsError::Validation("Batch does not contain DATE column".to_string())
                })?;

            let date_col = batch
                .column(date_col_idx)
                .as_any()
                .downcast_ref::<Date32Array>()
                .ok_or_else(|| {
                    IdsError::Validation("DATE column is not a date array".to_string())
                })?;

            let record_date =
                chrono::NaiveDate::from_num_days_from_ce_opt(date_col.value(row_idx) + 719163).unwrap_or_default();

            // If dates match, return the covariate
            if record_date == date {
                return Ok(Some(self.arrow_to_covariate(batch, row_idx, name)?));
            }
        }

        Ok(None)
    }

    fn get_covariates(&self, pnr: &Pnr, date: NaiveDate) -> Result<Vec<Covariate>> {
        // Find records for this PNR
        let positions = match self.pnr_to_row.get(pnr.value()) {
            Some(positions) => positions,
            None => return Ok(Vec::new()),
        };

        let mut covariates = Vec::new();

        // Check each position for the date
        for &(batch_idx, row_idx) in positions {
            let batch = &self.batches[batch_idx];

            // Check date column
            let date_col_idx = batch
                .schema()
                .fields()
                .iter()
                .position(|f| f.name() == "DATE")
                .ok_or_else(|| {
                    IdsError::Validation("Batch does not contain DATE column".to_string())
                })?;

            let date_col = batch
                .column(date_col_idx)
                .as_any()
                .downcast_ref::<Date32Array>()
                .ok_or_else(|| {
                    IdsError::Validation("DATE column is not a date array".to_string())
                })?;

            let record_date =
                chrono::NaiveDate::from_num_days_from_ce_opt(date_col.value(row_idx) + 719163).unwrap_or_default();

            // If dates match, add the covariate
            if record_date == date {
                covariates.push(self.arrow_to_covariate(batch, row_idx, "all")?);
            }
        }

        Ok(covariates)
    }

    fn add_covariate(&mut self, _pnr: &Pnr, _covariate: Covariate, _date: NaiveDate) -> Result<()> {
        Err(IdsError::Validation(
            "Adding covariates directly to an ArrowStore is not supported".to_string(),
        ))
    }
}
