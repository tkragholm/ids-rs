use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::akm::AkmSchema;
use crate::error::Result;
use arrow::record_batch::RecordBatch;

/// AKM registry loader for employment information
pub struct AkmRegister;

#[async_trait::async_trait]
impl RegisterLoader for AkmRegister {
    type SchemaType = AkmSchema;

    /// Get the name of the register
    fn register_name() -> &'static str {
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
        // Use the default DataFusion implementation
        self.default_load(base_path, pnr_filter).await
    }
}
