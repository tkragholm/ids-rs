//! LPR version 3 registry loader
//!
//! This module contains registry loader for the LPR version 3 registry.

use crate::data::registry::loaders::lpr::{LprRegistry, LprVersion};
use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::lpr::lpr3::Lpr3Schema;
use crate::data::schema::registry::lpr::LprSchema;
use crate::data::schema::traits::RegistrySchema;
use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::Path;

/// LPR version 3 registry loader
pub struct Lpr3Register;

#[async_trait::async_trait]
impl RegisterLoader for Lpr3Register {
    type SchemaType = Lpr3Schema;

    /// Get the name of the register
    fn register_name(&self) -> &'static str {
        "LPR3"
    }

    /// Load records from the LPR v3 register
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing the LPR v3 parquet files
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
        let _schema = Self::SchemaType::schema_arc();

        // Find paths to LPR v3 files
        let paths = self.find_files(Path::new(base_path))?;

        // Verify the path exists
        let path = Path::new(base_path);
        if !path.exists() {
            return Err(IdsError::Validation(format!(
                "Path does not exist: {}",
                path.display()
            )));
        }

        // Register tables for each component
        if let Some(kontakter_path) = &paths.kontakter_path {
            let kontakter_schema = Self::SchemaType::component_schema("kontakter")
                .ok_or_else(|| IdsError::Validation("Failed to get kontakter schema".into()))?;

            ctx.register_parquet(
                "lpr3_kontakter",
                kontakter_path.to_string_lossy().as_ref(),
                ParquetReadOptions::default().schema(&kontakter_schema),
            )
            .await?;
        }

        if let Some(diagnoser_path) = &paths.diagnoser_path {
            let diagnoser_schema = Self::SchemaType::component_schema("diagnoser")
                .ok_or_else(|| IdsError::Validation("Failed to get diagnoser schema".into()))?;

            ctx.register_parquet(
                "lpr3_diagnoser",
                diagnoser_path.to_string_lossy().as_ref(),
                ParquetReadOptions::default().schema(&diagnoser_schema),
            )
            .await?;
        }

        if let Some(procedurer_path) = &paths.procedurer_path {
            let procedurer_schema = Self::SchemaType::component_schema("procedurer")
                .ok_or_else(|| IdsError::Validation("Failed to get procedurer schema".into()))?;

            ctx.register_parquet(
                "lpr3_procedurer",
                procedurer_path.to_string_lossy().as_ref(),
                ParquetReadOptions::default().schema(&procedurer_schema),
            )
            .await?;
        }

        // Join the tables if they exist
        let mut sql_parts = Vec::new();
        let table_name = self.register_name().to_lowercase();

        // Start with kontakter data
        sql_parts.push(format!(
            "CREATE OR REPLACE TABLE {table_name} AS SELECT * FROM lpr3_kontakter"
        ));

        // Apply PNR filter if provided
        if let Some(filter) = pnr_filter {
            if filter.is_direct_filter() {
                let pnrs_list = filter
                    .pnrs()
                    .iter()
                    .map(|p| format!("'{p}'"))
                    .collect::<Vec<_>>()
                    .join(",");

                if !pnrs_list.is_empty() {
                    sql_parts.push(format!(
                        "CREATE OR REPLACE TABLE {table_name} AS
                         SELECT * FROM {table_name}
                         WHERE CPR IN ({pnrs_list})"
                    ));
                }
            } else if let Some(relation_col) = filter.relation_column() {
                let pnrs_list = filter
                    .pnrs()
                    .iter()
                    .map(|p| format!("'{p}'"))
                    .collect::<Vec<_>>()
                    .join(",");

                if !pnrs_list.is_empty() {
                    sql_parts.push(format!(
                        "CREATE OR REPLACE TABLE {table_name} AS
                         SELECT * FROM {table_name}
                         WHERE {relation_col} IN ({pnrs_list})"
                    ));
                }
            }
        }

        // Execute all SQL statements
        for sql in sql_parts {
            ctx.sql(&sql).await?;
        }

        // Get the result and return
        let df = ctx.table(&table_name).await?;
        let result = df.collect().await?;
        Ok(result)
    }
}

impl LprRegistry for Lpr3Register {
    /// Get the LPR version
    fn version(&self) -> LprVersion {
        LprVersion::V3
    }
}
