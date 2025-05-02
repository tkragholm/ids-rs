//! LPR version 2 registry loader
//!
//! This module contains registry loader for the LPR version 2 registry.

use crate::data::registry::loaders::lpr::{LprComponents, LprRegistry, LprVersion};
use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::lpr::lpr2::Lpr2Schema;

use crate::error::{IdsError, Result};
use arrow::array::Array;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;

/// LPR version 2 registry loader
pub struct Lpr2Register;

#[async_trait::async_trait]
impl RegisterLoader for Lpr2Register {
    type SchemaType = Lpr2Schema;

    /// Get the name of the register
    fn register_name(&self) -> &'static str {
        "LPR2"
    }

    /// Load records from the LPR v2 register
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing the LPR v2 parquet files
    /// * `pnr_filter` - Optional filter to only load data for specific PNRs
    ///
    /// # Returns
    /// * `Result<Vec<RecordBatch>>` - Arrow record batches containing the loaded data
    async fn load(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<Vec<RecordBatch>> {
        // Create unique call ID for diagnostic tracing
        let uuid_str = uuid::Uuid::new_v4().to_string();
        let call_id = uuid_str.split('-').next().unwrap_or("unknown");
        log::info!(
            "[DIAG-{}] ENTER Lpr2Register::load with base_path: {}",
            call_id,
            base_path
        );

        // Load components from the LPR data source
        let components = self.load_components(base_path, pnr_filter).await?;

        // Return admin data directly - no joining in the loader
        if let Some(admin_data) = components.lpr2_adm {
            if !admin_data.is_empty() {
                log::info!(
                    "[DIAG-{}] Loaded admin data with {} record batches",
                    call_id,
                    admin_data.len()
                );
                return Ok(admin_data);
            }
        }

        // If no admin data, return empty result
        log::warn!("[DIAG-{}] No admin data found", call_id);
        Ok(Vec::new())
    }
}

impl Lpr2Register {
    // Using the shared batches_to_dataframe function from mod.rs
}

// Using the shared find_parquet_files_in_dir function from mod.rs

impl LprRegistry for Lpr2Register {
    /// Get the LPR version
    fn version(&self) -> LprVersion {
        LprVersion::V2
    }

    async fn load_components(
        &self,
        base_path: &str,
        pnr_filter: Option<&crate::data::registry::traits::PnrFilter>,
    ) -> Result<LprComponents> {
        use arrow::array::StringArray;
        // Arrow imports for type handling

        // Create a longer-lived string value
        let uuid_str = uuid::Uuid::new_v4().to_string();
        let call_id = uuid_str.split('-').next().unwrap_or("unknown");
        log::info!(
            "[DIAG-{}] Loading LPR2 components from '{}'",
            call_id,
            base_path
        );

        // Resolve base path to an absolute path
        let abs_base_path = crate::utils::path_utils::resolve_path(base_path)?;
        log::debug!(
            "[DIAG-{}] Resolved base path: '{}'",
            call_id,
            abs_base_path.display()
        );

        // Find LPR files
        let lpr_paths = self.find_files(&abs_base_path)?;
        log::debug!(
            "[DIAG-{}] Found LPR paths with admin_path={:?}, diag_path={:?}, proc_path={:?}",
            call_id,
            lpr_paths.admin_path,
            lpr_paths.diag_path,
            lpr_paths.proc_path
        );

        // Create a new LprComponents to hold the results
        let mut components = LprComponents::new();

        // Load admin data if path is available
        if let Some(admin_path) = &lpr_paths.admin_path {
            let abs_admin_path = crate::utils::path_utils::resolve_path(admin_path)?;
            log::info!(
                "[DIAG-{}] Loading LPR2 admin data from '{}'",
                call_id,
                abs_admin_path.display()
            );

            // Check cache first
            let cached_data = {
                let cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                    .lock()
                    .unwrap();
                cache.get_lpr2_admin(&abs_admin_path)
            };

            if let Some(cached) = cached_data {
                log::info!(
                    "[DIAG-{}] Using cached LPR2 admin data ({} batches)",
                    call_id,
                    cached.len()
                );
                components = components.with_lpr2_adm(cached);
            } else {
                // Need to load from source
                let ctx = SessionContext::new();
                let admin_table_name = format!("lpr2_adm_{}", call_id);

                if let Ok(_) = ctx
                    .register_parquet(
                        &admin_table_name,
                        abs_admin_path.to_string_lossy().to_string(),
                        ParquetReadOptions::default(),
                    )
                    .await
                {
                    log::info!(
                        "[DIAG-{}] Successfully registered admin data as table '{}'",
                        call_id,
                        admin_table_name
                    );

                    // Get as DataFrame and apply filter
                    let mut admin_df = ctx.table(&admin_table_name).await?;

                    // Apply filter if provided
                    if let Some(pnr_filter_val) = pnr_filter {
                        if pnr_filter_val.is_direct_filter() {
                            let pnr_list: Vec<Expr> = pnr_filter_val
                                .pnrs()
                                .iter()
                                .map(|pnr| lit(pnr.clone()))
                                .collect();

                            if !pnr_list.is_empty() {
                                admin_df = admin_df.filter(col("PNR").in_list(pnr_list, false))?;
                            }
                        } else if let Some(relation_col) = pnr_filter_val.relation_column() {
                            let pnr_list: Vec<Expr> = pnr_filter_val
                                .pnrs()
                                .iter()
                                .map(|pnr| lit(pnr.clone()))
                                .collect();

                            if !pnr_list.is_empty() {
                                admin_df =
                                    admin_df.filter(col(relation_col).in_list(pnr_list, false))?;
                            }
                        }
                    }

                    // Collect admin data
                    let admin_batches = admin_df.collect().await?;
                    if !admin_batches.is_empty() {
                        let record_count =
                            admin_batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::info!(
                            "[DIAG-{}] Collected {} admin data batches with {} total records",
                            call_id,
                            admin_batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr2_admin(&abs_admin_path, admin_batches.clone());
                        }

                        components = components.with_lpr2_adm(admin_batches);
                    } else {
                        log::warn!("[DIAG-{}] No admin data records found", call_id);
                    }
                } else {
                    log::warn!(
                        "[DIAG-{}] Failed to register admin data using DataFusion",
                        call_id
                    );

                    // Try to load the admin data directly using the parquet loader
                    log::debug!(
                        "[DIAG-{}] Falling back to io::parquet::load_parquet_directory for admin data",
                        call_id
                    );

                    // Fallback implementation - manually use parquet loader
                    let io_filter = pnr_filter.map(|f| f.to_io_filter());

                    if let Ok(batches) = crate::data::io::parquet::load_parquet_directory(
                        &abs_admin_path,
                        None, // Don't enforce schema - use the schema from the file
                        io_filter.as_ref(),
                    )
                    .await
                    {
                        let record_count = batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::debug!(
                            "[DIAG-{}] Loaded {} admin batches with {} total records using fallback method",
                            call_id,
                            batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr2_admin(&abs_admin_path, batches.clone());
                        }

                        components = components.with_lpr2_adm(batches);
                    } else {
                        log::error!(
                            "[DIAG-{}] Failed to load admin data using both DataFusion and fallback method",
                            call_id
                        );
                        return Err(IdsError::Data(format!(
                            "Failed to load LPR2 admin data from {}",
                            abs_admin_path.display()
                        )));
                    }
                }
            }
        }

        // Load diagnosis data if path is available
        if let Some(diag_path) = &lpr_paths.diag_path {
            let abs_diag_path = crate::utils::path_utils::resolve_path(diag_path)?;
            log::debug!(
                "[DIAG-{}] Loading LPR2 diagnosis data from '{}'",
                call_id,
                abs_diag_path.display()
            );

            // Check cache first
            let cached_data = {
                let cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                    .lock()
                    .unwrap();
                cache.get_lpr2_diag(&abs_diag_path)
            };

            if let Some(cached) = cached_data {
                log::debug!(
                    "[DIAG-{}] Using cached LPR2 diagnosis data ({} batches)",
                    call_id,
                    cached.len()
                );
                components = components.with_lpr2_diag(cached);
            } else {
                // Need to load from source
                let ctx = SessionContext::new();
                let diag_table_name = format!("lpr2_diag_{}", call_id);

                if let Ok(_) = ctx
                    .register_parquet(
                        &diag_table_name,
                        abs_diag_path.to_string_lossy().to_string(),
                        ParquetReadOptions::default(),
                    )
                    .await
                {
                    log::debug!(
                        "[DIAG-{}] Successfully registered diagnosis data as table '{}'",
                        call_id,
                        diag_table_name
                    );

                    // Get diagnosis data as DataFrame and apply filter via RECNUM -> V_PNR relationship
                    let mut diag_df = ctx.table(&diag_table_name).await?;

                    // Apply PNR filter if needed
                    if let Some(_pnr_filter_val) = pnr_filter {
                        if components.lpr2_adm.is_some() {
                            // Since diagnosis data doesn't have direct PNR column, we join with admin data
                            // to filter by RECNUM where admin.PNR matches our filter
                            let admin_data = components.lpr2_adm.as_ref().unwrap();

                            if !admin_data.is_empty() {
                                // Create a vector to store record numbers from filtered admin data
                                let mut recnums = Vec::new();
                                for batch in admin_data {
                                    if let Ok(recnum_idx) = batch.schema().index_of("RECNUM") {
                                        if let Some(recnum_array) = batch
                                            .column(recnum_idx)
                                            .as_any()
                                            .downcast_ref::<StringArray>()
                                        {
                                            for i in 0..recnum_array.len() {
                                                if !recnum_array.is_null(i) {
                                                    recnums.push(recnum_array.value(i).to_string());
                                                }
                                            }
                                        }
                                    }
                                }

                                // Only apply filter if we have recnums to filter by
                                if !recnums.is_empty() {
                                    let recnum_exprs: Vec<Expr> =
                                        recnums.iter().map(|r| lit(r.clone())).collect();

                                    diag_df = diag_df
                                        .filter(col("RECNUM").in_list(recnum_exprs, false))?;
                                }
                            }
                        }
                    }

                    // Collect diagnosis data
                    let diag_batches = diag_df.collect().await?;
                    if !diag_batches.is_empty() {
                        let record_count = diag_batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::debug!(
                            "[DIAG-{}] Collected {} diagnosis data batches with {} total records",
                            call_id,
                            diag_batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr2_diag(&abs_diag_path, diag_batches.clone());
                        }

                        components = components.with_lpr2_diag(diag_batches);
                    } else {
                        log::warn!("[DIAG-{}] No diagnosis data records found", call_id);
                    }
                } else {
                    log::warn!(
                        "[DIAG-{}] Failed to register diagnosis data using DataFusion",
                        call_id
                    );

                    // Try to load the diagnosis data directly using parquet loader
                    log::debug!(
                        "[DIAG-{}] Falling back to io::parquet::load_parquet_directory for diagnosis data",
                        call_id
                    );

                    // Fallback implementation - manually use parquet loader
                    let io_filter = pnr_filter.map(|f| f.to_io_filter());

                    if let Ok(batches) = crate::data::io::parquet::load_parquet_directory(
                        &abs_diag_path,
                        None, // Don't enforce schema - use the schema from the file
                        io_filter.as_ref(),
                    )
                    .await
                    {
                        let record_count = batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::info!(
                            "[DIAG-{}] Loaded {} diagnosis batches with {} total records using fallback method",
                            call_id,
                            batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr2_diag(&abs_diag_path, batches.clone());
                        }

                        components = components.with_lpr2_diag(batches);
                    } else {
                        log::error!(
                            "[DIAG-{}] Failed to load diagnosis data using both DataFusion and fallback method",
                            call_id
                        );
                        // Don't return error, try to continue with other data
                    }
                }
            }
        }

        // Load procedure data if path is available
        if let Some(proc_path) = &lpr_paths.proc_path {
            let abs_proc_path = crate::utils::path_utils::resolve_path(proc_path)?;
            log::info!(
                "[DIAG-{}] Loading LPR2 procedure data from '{}'",
                call_id,
                abs_proc_path.display()
            );

            // Check cache first
            let cached_data = {
                let cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                    .lock()
                    .unwrap();
                cache.get_lpr2_bes(&abs_proc_path)
            };

            if let Some(cached) = cached_data {
                log::info!(
                    "[DIAG-{}] Using cached LPR2 procedure data ({} batches)",
                    call_id,
                    cached.len()
                );
                components = components.with_lpr2_bes(cached);
            } else {
                // Need to load from source
                let ctx = SessionContext::new();
                let proc_table_name = format!("lpr2_proc_{}", call_id);

                if let Ok(_) = ctx
                    .register_parquet(
                        &proc_table_name,
                        abs_proc_path.to_string_lossy().to_string(),
                        ParquetReadOptions::default(),
                    )
                    .await
                {
                    log::info!(
                        "[DIAG-{}] Successfully registered procedure data as table '{}'",
                        call_id,
                        proc_table_name
                    );

                    // Get procedure data as DataFrame and apply filter via RECNUM relationship
                    let mut proc_df = ctx.table(&proc_table_name).await?;

                    // Apply PNR filter if needed - similar to diagnosis data
                    if let Some(_pnr_filter_val) = pnr_filter {
                        if components.lpr2_adm.is_some() {
                            // Since procedure data doesn't have direct PNR column, we join with admin data
                            // to filter by RECNUM where admin.PNR matches our filter
                            let admin_data = components.lpr2_adm.as_ref().unwrap();

                            if !admin_data.is_empty() {
                                // Get RECNUMs from filtered admin data
                                let mut recnums = Vec::new();
                                for batch in admin_data {
                                    if let Ok(recnum_idx) = batch.schema().index_of("RECNUM") {
                                        if let Some(recnum_array) = batch
                                            .column(recnum_idx)
                                            .as_any()
                                            .downcast_ref::<StringArray>()
                                        {
                                            for i in 0..recnum_array.len() {
                                                if !recnum_array.is_null(i) {
                                                    recnums.push(recnum_array.value(i).to_string());
                                                }
                                            }
                                        }
                                    }
                                }

                                // Only apply filter if we have recnums to filter by
                                if !recnums.is_empty() {
                                    let recnum_exprs: Vec<Expr> =
                                        recnums.iter().map(|r| lit(r.clone())).collect();

                                    proc_df = proc_df
                                        .filter(col("RECNUM").in_list(recnum_exprs, false))?;
                                }
                            }
                        }
                    }

                    // Collect procedure data
                    let proc_batches = proc_df.collect().await?;
                    if !proc_batches.is_empty() {
                        let record_count = proc_batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::info!(
                            "[DIAG-{}] Collected {} procedure data batches with {} total records",
                            call_id,
                            proc_batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr2_bes(&abs_proc_path, proc_batches.clone());
                        }

                        components = components.with_lpr2_bes(proc_batches);
                    } else {
                        log::warn!("[DIAG-{}] No procedure data records found", call_id);
                    }
                } else {
                    log::warn!(
                        "[DIAG-{}] Failed to register procedure data using DataFusion",
                        call_id
                    );

                    // Try to load the procedure data directly using parquet loader
                    log::info!(
                        "[DIAG-{}] Falling back to io::parquet::load_parquet_directory for procedure data",
                        call_id
                    );

                    // Fallback implementation - manually use parquet loader
                    let io_filter = pnr_filter.map(|f| f.to_io_filter());

                    if let Ok(batches) = crate::data::io::parquet::load_parquet_directory(
                        &abs_proc_path,
                        None, // Don't enforce schema - use the schema from the file
                        io_filter.as_ref(),
                    )
                    .await
                    {
                        let record_count = batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::info!(
                            "[DIAG-{}] Loaded {} procedure batches with {} total records using fallback method",
                            call_id,
                            batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr2_bes(&abs_proc_path, batches.clone());
                        }

                        components = components.with_lpr2_bes(batches);
                    } else {
                        log::error!(
                            "[DIAG-{}] Failed to load procedure data using both DataFusion and fallback method",
                            call_id
                        );
                        // Don't return error, try to continue with other data
                    }
                }
            }
        }

        // Return the components
        log::info!("[DIAG-{}] Completed loading LPR2 components", call_id);
        Ok(components)
    }
}
