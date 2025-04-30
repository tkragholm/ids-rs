//! Examples of using the new DataFusion-based registry data API
//!
//! This module contains examples of using the new DataFusion-based registry data API.

use super::prelude::*;
use crate::error::Result;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::collections::HashSet;

/// Example: Load AKM data with PNR filter
pub async fn load_akm_data(base_path: &str, pnrs: &[&str]) -> Result<Vec<RecordBatch>> {
    // Create PNR filter
    let pnr_set: HashSet<String> = pnrs.iter().map(|&p| p.to_string()).collect();
    let pnr_filter = PnrFilter::new(pnr_set);

    // Create AKM registry loader
    let akm_registry = AkmRegister;

    // Load data
    akm_registry.load(base_path, Some(&pnr_filter)).await
}

/// Example: Use the ParquetReader directly
pub async fn load_with_parquet_reader(path: &str) -> Result<Vec<RecordBatch>> {
    // Create a ParquetReader with AKM schema
    let reader = ParquetReader::new(path)
        .with_schema(AkmSchema::schema_arc())
        .with_batch_size(10000);

    // Read async
    reader.read_async().await
}

/// Example: Use a transform pipeline
pub async fn transform_akm_data(base_path: &str, pnrs: &[&str]) -> Result<Vec<RecordBatch>> {
    // Create PNR filter
    let pnr_set: HashSet<String> = pnrs.iter().map(|&p| p.to_string()).collect();
    let pnr_filter = PnrFilter::new(pnr_set);

    // Create a session context
    let ctx = SessionContext::new();

    // Create AKM registry loader
    let akm_registry = AkmRegister;

    // Create context with registered table
    let ctx = akm_registry
        .create_context(base_path, Some(&pnr_filter))
        .await?;

    // Create a transform pipeline
    let pipeline = TransformPipeline::new()
        .add_filter(col("SOCIO").is_not_null())
        .add_operation(|df| {
            Ok(df.with_column(
                "SOCIO_CATEGORY",
                when(col("SOCIO").lt_eq(lit(2)))
                    .then(lit("Low"))
                    .when(col("SOCIO").lt_eq(lit(4)))
                    .then(lit("Medium"))
                    .otherwise(lit("High")),
            )?)
        });

    // Execute the pipeline
    pipeline.execute(&ctx, "akm").await
}

/// Example: Use the SQL engine
pub async fn query_with_sql(base_path: &str, pnrs: &[&str]) -> Result<Vec<RecordBatch>> {
    // Create PNR filter
    let pnr_set: HashSet<String> = pnrs.iter().map(|&p| p.to_string()).collect();
    let pnr_filter = PnrFilter::new(pnr_set);

    // Create SQL engine
    let mut sql_engine = RegistrySqlEngine::new();

    // Register registry
    sql_engine
        .register_registry(&AkmRegister, base_path, Some(&pnr_filter), None)
        .await?;

    // Execute SQL query
    sql_engine
        .execute_sql(
            "
        SELECT
            PNR,
            SOCIO,
            CASE
                WHEN SOCIO <= 2 THEN 'Low'
                WHEN SOCIO <= 4 THEN 'Medium'
                ELSE 'High'
            END AS SOCIO_CATEGORY
        FROM
            akm
        WHERE
            SOCIO IS NOT NULL
    ",
        )
        .await
}

/// Example: Join multiple registries
pub async fn join_registries(
    akm_path: &str,
    bef_path: &str,
    pnrs: &[&str],
) -> Result<Vec<RecordBatch>> {
    // Create PNR filter
    let pnr_set: HashSet<String> = pnrs.iter().map(|&p| p.to_string()).collect();
    let pnr_filter = PnrFilter::new(pnr_set);

    // Create SQL engine
    let mut sql_engine = RegistrySqlEngine::new();

    // Register registries
    sql_engine
        .register_registry(&AkmRegister, akm_path, Some(&pnr_filter), None)
        .await?;

    // Execute SQL join query
    sql_engine
        .execute_sql(
            "
        SELECT
            a.PNR,
            a.SOCIO,
            b.GENDER,
            b.BIRTHDAY
        FROM
            akm a
        JOIN
            bef b ON a.PNR = b.PNR
        WHERE
            a.SOCIO IS NOT NULL
    ",
        )
        .await
}
