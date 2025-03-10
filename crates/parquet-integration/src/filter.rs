use std::collections::HashSet;
use std::sync::Arc;

use arrow_array::{Array, BooleanArray, RecordBatch, StringArray};
use arrow_select::filter::filter_record_batch;
use log::debug;

use crate::error::{ParquetIntegrationError, Result};

/// Engine for filtering record batches
#[derive(Clone)]
pub struct FilterEngine {
    /// PNR filter
    pnr_filter: Option<Arc<HashSet<String>>>,
}

impl FilterEngine {
    /// Create a new filter engine
    pub fn new() -> Self {
        Self {
            pnr_filter: None,
        }
    }
    
    /// Set PNR filter
    pub fn with_pnr_filter(mut self, pnr_filter: HashSet<String>) -> Self {
        self.pnr_filter = Some(Arc::new(pnr_filter));
        self
    }
    
    /// Filter a batch by PNR
    pub fn filter_batch(&self, batch: &RecordBatch) -> Result<RecordBatch> {
        if let Some(pnr_filter) = &self.pnr_filter {
            filter_batch_by_pnr(batch, pnr_filter)
        } else {
            Ok(batch.clone())
        }
    }
    
    /// Filter multiple batches
    pub fn filter_batches(&self, batches: Vec<RecordBatch>) -> Result<Vec<RecordBatch>> {
        if let Some(pnr_filter) = &self.pnr_filter {
            let mut filtered_batches = Vec::with_capacity(batches.len());
            for batch in batches {
                let filtered = filter_batch_by_pnr(&batch, pnr_filter)?;
                
                // Only add non-empty batches
                if filtered.num_rows() > 0 {
                    filtered_batches.push(filtered);
                }
            }
            Ok(filtered_batches)
        } else {
            Ok(batches)
        }
    }
}

/// Filter a record batch based on PNR values
pub fn filter_batch_by_pnr(
    batch: &RecordBatch,
    pnr_filter: &HashSet<String>,
) -> Result<RecordBatch> {
    // Try to find the PNR column
    let pnr_col_idx = batch
        .schema()
        .fields()
        .iter()
        .enumerate()
        .find(|(_, field)| field.name().to_lowercase() == "pnr")
        .map(|(idx, _)| idx);

    // If no PNR column found, return the batch as is
    let Some(pnr_idx) = pnr_col_idx else {
        return Ok(batch.clone());
    };

    // Get the PNR column
    let pnr_array = batch.column(pnr_idx);

    // Try to cast to string array
    let pnr_strings = match pnr_array.as_any().downcast_ref::<StringArray>() {
        Some(arr) => arr,
        None => {
            return Err(ParquetIntegrationError::OperationError(
                "Could not cast PNR column to StringArray".to_string(),
            ));
        }
    };

    // Create a boolean mask for filtering
    let mut mask = Vec::with_capacity(batch.num_rows());
    for i in 0..pnr_strings.len() {
        mask.push(pnr_filter.contains(pnr_strings.value(i)));
    }

    // Count matches for debugging
    let matches = mask.iter().filter(|&&b| b).count();
    debug!("PNR filter: {} matches out of {} rows", matches, mask.len());

    // Filter the batch using the mask
    let filtered_batch = filter_record_batch(batch, &BooleanArray::from(mask))
        .map_err(|e| ParquetIntegrationError::ArrowError(format!("Error filtering batch: {}", e)))?;

    Ok(filtered_batch)
}

/// Post-process a list of record batches with PNR filtering
pub fn post_process_batches(
    batches: Vec<RecordBatch>,
    pnr_filter: Option<&Arc<HashSet<String>>>,
) -> Result<Vec<RecordBatch>> {
    match pnr_filter {
        Some(filter) => {
            // Filter each batch
            let mut filtered_batches = Vec::with_capacity(batches.len());
            for batch in batches {
                let filtered = filter_batch_by_pnr(&batch, filter)?;
                
                // Only add non-empty batches
                if filtered.num_rows() > 0 {
                    filtered_batches.push(filtered);
                }
            }
            Ok(filtered_batches)
        }
        None => Ok(batches),
    }
}