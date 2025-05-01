use crate::data::registry::traits::PnrFilter;
use crate::error::Result;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::catalog::MemTable;
use datafusion::execution::context::SessionContext;
use datafusion::prelude::*;
use rand::random;
use std::path::Path;
use std::sync::Arc;

/// Create a new `DataFusion` session context with optimized settings
#[must_use] pub fn create_optimized_context() -> SessionContext {
    let config = SessionConfig::new()
        .with_target_partitions(4) // Reasonable default without depending on num_cpus
        .with_batch_size(8192);
    SessionContext::new_with_config(config)
}

/// Register a parquet file or directory with the session context
pub async fn register_parquet(
    ctx: &SessionContext,
    table_name: &str,
    path: &str,
    schema: SchemaRef,
) -> Result<()> {
    // Verify the path exists
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        return Err(crate::error::IdsError::Validation(format!(
            "Path does not exist: {}",
            path_obj.display()
        )));
    }

    // Register with DataFusion
    ctx.register_parquet(
        table_name,
        path,
        ParquetReadOptions::default().schema(schema.as_ref()),
    )
    .await?;

    Ok(())
}

/// Apply a PNR filter to a `DataFrame`
pub fn apply_pnr_filter(df: DataFrame, pnr_filter: &PnrFilter) -> Result<DataFrame> {
    if pnr_filter.is_direct_filter() {
        // Convert HashSet to a list of literals for IN expression
        let pnr_list: Vec<Expr> = pnr_filter
            .pnrs()
            .iter()
            .map(|pnr| lit(pnr.clone()))
            .collect();

        if !pnr_list.is_empty() {
            // Create filter: PNR IN (pnr1, pnr2, ...)
            return Ok(df.filter(col("PNR").in_list(pnr_list, false))?);
        }
    } else if let Some(relation_col) = pnr_filter.relation_column() {
        // Similar approach for relation filtering
        let pnr_list: Vec<Expr> = pnr_filter
            .pnrs()
            .iter()
            .map(|pnr| lit(pnr.clone()))
            .collect();

        if !pnr_list.is_empty() {
            // Create filter: relation_col IN (pnr1, pnr2, ...)
            return Ok(df.filter(col(relation_col).in_list(pnr_list, false))?);
        }
    }

    Ok(df)
}

/// Convert `RecordBatches` to `DataFrame`
pub async fn batches_to_dataframe(
    ctx: &SessionContext,
    batches: Vec<RecordBatch>,
) -> Result<DataFrame> {
    if batches.is_empty() {
        return Err(crate::error::IdsError::Validation(
            "Cannot create DataFrame from empty batches".to_string(),
        ));
    }

    let provider = MemTable::try_new(batches[0].schema(), vec![batches])?;
    let provider_ref = Arc::new(provider);
    // Register as a temporary table and then create DataFrame from it
    let temp_name = format!("temp_table_{}", random::<u64>());
    ctx.register_table(&temp_name, provider_ref)?;
    let df = ctx.table(&temp_name).await?;
    Ok(df)
}

/// Set common `DataFusion` logging level
pub fn set_datafusion_log_level(level: &str) {
    // Only set if not already set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("datafusion={level},datafusion_optimizer={level}"),
        );
        let _ = env_logger::try_init();
    }
}