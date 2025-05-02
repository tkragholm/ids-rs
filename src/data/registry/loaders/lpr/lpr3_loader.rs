//! LPR version 3 registry loader
//!
//! This module contains registry loader for the LPR version 3 registry.

use crate::data::registry::loaders::lpr::{LprComponents, LprRegistry, LprVersion};
use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::lpr::lpr3::Lpr3Schema;
use arrow::array::Array;

use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;

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
        // Use the load_components method to load all components
        let components = self.load_components(base_path, pnr_filter).await?;

        // Return kontakter data directly - no joining in the loader
        if let Some(kontakter_data) = components.lpr3_kontakter {
            if !kontakter_data.is_empty() {
                log::info!(
                    "Loaded kontakter data with {} record batches",
                    kontakter_data.len()
                );
                return Ok(kontakter_data);
            }
        }

        // If no kontakter data, return empty result
        log::warn!("No kontakter data found");
        Ok(Vec::new())
    }
}

impl Lpr3Register {
    // Using the shared batches_to_dataframe function from mod.rs
}

impl LprRegistry for Lpr3Register {
    /// Get the LPR version
    fn version(&self) -> LprVersion {
        LprVersion::V3
    }

    async fn load_components(
        &self,
        base_path: &str,
        pnr_filter: Option<&crate::data::registry::traits::PnrFilter>,
    ) -> Result<LprComponents> {
        use arrow::array::StringArray;

        // Resolve base path to an absolute path
        let abs_base_path = crate::utils::path_utils::resolve_path(base_path)?;
        log::debug!(
            "Resolved base path for LPR3 components: '{}'",
            abs_base_path.display()
        );

        // Find LPR files
        let lpr_paths = self.find_files(&abs_base_path)?;
        log::debug!(
            "Found LPR3 paths with kontakter_path={:?}, diagnoser_path={:?}, procedurer_path={:?}",
            lpr_paths.kontakter_path,
            lpr_paths.diagnoser_path,
            lpr_paths.procedurer_path
        );

        // Create a new LprComponents to hold the results
        let mut components = LprComponents::new();

        // Load kontakter data if path is available (required)
        if let Some(kontakter_path) = &lpr_paths.kontakter_path {
            let abs_kontakter_path = crate::utils::path_utils::resolve_path(kontakter_path)?;
            log::info!(
                "Loading LPR3 kontakter data from '{}'",
                abs_kontakter_path.display()
            );

            // Check cache first
            let cached_data = {
                let cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                    .lock()
                    .unwrap();
                cache.get_lpr3_kontakter(&abs_kontakter_path)
            };

            if let Some(cached) = cached_data {
                log::info!(
                    "Using cached LPR3 kontakter data ({} batches)",
                    cached.len()
                );
                components = components.with_lpr3_kontakter(cached);
            } else {
                // Create DataFusion context for data manipulation
                let ctx = SessionContext::new();

                // Need to load kontakter data from source
                if let Ok(_) = ctx
                    .register_parquet(
                        "lpr3_kontakter",
                        abs_kontakter_path.to_string_lossy().to_string(),
                        ParquetReadOptions::default(),
                    )
                    .await
                {
                    log::info!("Successfully registered kontakter data as table 'lpr3_kontakter'");

                    // Get as DataFrame and apply filter
                    let mut kontakter_df = ctx.table("lpr3_kontakter").await?;

                    // Apply filter if provided
                    if let Some(filter) = pnr_filter {
                        if filter.is_direct_filter() {
                            let pnr_list: Vec<Expr> =
                                filter.pnrs().iter().map(|pnr| lit(pnr.clone())).collect();

                            if !pnr_list.is_empty() {
                                // LPR3 uses CPR instead of PNR
                                kontakter_df =
                                    kontakter_df.filter(col("CPR").in_list(pnr_list, false))?;
                            }
                        } else if let Some(relation_col) = filter.relation_column() {
                            let pnr_list: Vec<Expr> =
                                filter.pnrs().iter().map(|pnr| lit(pnr.clone())).collect();

                            if !pnr_list.is_empty() {
                                kontakter_df = kontakter_df
                                    .filter(col(relation_col).in_list(pnr_list, false))?;
                            }
                        }
                    }

                    // Collect kontakter data
                    let kontakter_batches = kontakter_df.collect().await?;
                    if !kontakter_batches.is_empty() {
                        let record_count = kontakter_batches
                            .iter()
                            .map(|b| b.num_rows())
                            .sum::<usize>();
                        log::info!(
                            "Collected {} kontakter data batches with {} total records",
                            kontakter_batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr3_kontakter(
                                &abs_kontakter_path,
                                kontakter_batches.clone(),
                            );
                        }

                        components = components.with_lpr3_kontakter(kontakter_batches);
                    } else {
                        log::warn!("No kontakter data records found");
                    }
                } else {
                    log::warn!("Failed to register kontakter data using DataFusion");

                    // Try to load using fallback method - parquet loader
                    log::info!(
                        "Falling back to io::parquet::load_parquet_directory for kontakter data"
                    );

                    // Fallback implementation - manually use parquet loader
                    let io_filter = pnr_filter.map(|f| f.to_io_filter());
                    if let Ok(batches) = crate::data::io::parquet::load_parquet_directory(
                        &abs_kontakter_path,
                        None, // Don't enforce schema
                        io_filter.as_ref(),
                    )
                    .await
                    {
                        let record_count = batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::info!(
                            "Loaded {} kontakter batches with {} total records using fallback method",
                            batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr3_kontakter(&abs_kontakter_path, batches.clone());
                        }

                        components = components.with_lpr3_kontakter(batches);
                    } else {
                        log::error!(
                            "Failed to load kontakter data using both DataFusion and fallback method"
                        );
                        return Err(IdsError::Data(
                            "Failed to load LPR3 kontakter data".to_string(),
                        ));
                    }
                }
            }
        }

        // Load diagnoser data if path is available
        if let Some(diagnoser_path) = &lpr_paths.diagnoser_path {
            let abs_diagnoser_path = crate::utils::path_utils::resolve_path(diagnoser_path)?;
            log::info!(
                "Loading LPR3 diagnoser data from '{}'",
                abs_diagnoser_path.display()
            );

            // Check cache first
            let cached_data = {
                let cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                    .lock()
                    .unwrap();
                cache.get_lpr3_diagnoser(&abs_diagnoser_path)
            };

            if let Some(cached) = cached_data {
                log::info!(
                    "Using cached LPR3 diagnoser data ({} batches)",
                    cached.len()
                );
                components = components.with_lpr3_diagnoser(cached);
            } else {
                // Create DataFusion context for data manipulation
                let ctx = SessionContext::new();

                // Need to load diagnoser data from source
                if let Ok(_) = ctx
                    .register_parquet(
                        "lpr3_diagnoser",
                        abs_diagnoser_path.to_string_lossy().to_string(),
                        ParquetReadOptions::default(),
                    )
                    .await
                {
                    log::info!("Successfully registered diagnoser data as table 'lpr3_diagnoser'");

                    // Get as DataFrame
                    let mut diagnoser_df = ctx.table("lpr3_diagnoser").await?;

                    // Apply kontakter filter if we have kontakter data with kontakt_ids
                    if let Some(kontakter_data) = &components.lpr3_kontakter {
                        if !kontakter_data.is_empty() {
                            // Extract kontakt_ids from kontakter data to filter diagnoser
                            let mut kontakt_ids = Vec::new();

                            for batch in kontakter_data {
                                if let Ok(kontakt_id_idx) = batch.schema().index_of("kontakt_id") {
                                    if let Some(kontakt_id_array) = batch
                                        .column(kontakt_id_idx)
                                        .as_any()
                                        .downcast_ref::<StringArray>()
                                    {
                                        for i in 0..kontakt_id_array.len() {
                                            if !kontakt_id_array.is_null(i) {
                                                kontakt_ids
                                                    .push(kontakt_id_array.value(i).to_string());
                                            }
                                        }
                                    }
                                }
                            }

                            // Apply filter if we have kontakt_ids
                            if !kontakt_ids.is_empty() {
                                let kontakt_id_exprs: Vec<Expr> =
                                    kontakt_ids.iter().map(|id| lit(id.clone())).collect();

                                diagnoser_df = diagnoser_df
                                    .filter(col("kontakt_id").in_list(kontakt_id_exprs, false))?;
                            }
                        }
                    }

                    // Collect diagnoser data
                    let diagnoser_batches = diagnoser_df.collect().await?;
                    if !diagnoser_batches.is_empty() {
                        let record_count = diagnoser_batches
                            .iter()
                            .map(|b| b.num_rows())
                            .sum::<usize>();
                        log::info!(
                            "Collected {} diagnoser data batches with {} total records",
                            diagnoser_batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr3_diagnoser(
                                &abs_diagnoser_path,
                                diagnoser_batches.clone(),
                            );
                        }

                        components = components.with_lpr3_diagnoser(diagnoser_batches);
                    } else {
                        log::warn!("No diagnoser data records found");
                    }
                } else {
                    log::warn!("Failed to register diagnoser data using DataFusion");

                    // Try fallback method
                    // Fallback implementation - manually use parquet loader
                    let io_filter = pnr_filter.map(|f| f.to_io_filter());
                    if let Ok(batches) = crate::data::io::parquet::load_parquet_directory(
                        &abs_diagnoser_path,
                        None, // Don't enforce schema
                        io_filter.as_ref(),
                    )
                    .await
                    {
                        let record_count = batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::info!(
                            "Loaded {} diagnoser batches with {} total records using fallback method",
                            batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr3_diagnoser(&abs_diagnoser_path, batches.clone());
                        }

                        components = components.with_lpr3_diagnoser(batches);
                    } else {
                        log::warn!("Failed to load diagnoser data using both DataFusion and fallback method");
                        // Continue without diagnoser data
                    }
                }
            }
        }

        // Load procedurer data if path is available
        if let Some(procedurer_path) = &lpr_paths.procedurer_path {
            let abs_procedurer_path = crate::utils::path_utils::resolve_path(procedurer_path)?;
            log::info!(
                "Loading LPR3 procedurer data from '{}'",
                abs_procedurer_path.display()
            );

            // Check cache first
            let cached_data = {
                let cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                    .lock()
                    .unwrap();
                cache.get_lpr3_procedurer(&abs_procedurer_path)
            };

            if let Some(cached) = cached_data {
                log::info!(
                    "Using cached LPR3 procedurer data ({} batches)",
                    cached.len()
                );
                components = components.with_lpr3_procedurer(cached);
            } else {
                // Create DataFusion context for data manipulation
                let ctx = SessionContext::new();

                // Need to load procedurer data from source
                if let Ok(_) = ctx
                    .register_parquet(
                        "lpr3_procedurer",
                        abs_procedurer_path.to_string_lossy().to_string(),
                        ParquetReadOptions::default(),
                    )
                    .await
                {
                    log::info!(
                        "Successfully registered procedurer data as table 'lpr3_procedurer'"
                    );

                    // Get as DataFrame
                    let mut procedurer_df = ctx.table("lpr3_procedurer").await?;

                    // Apply kontakter filter if we have kontakter data with kontakt_ids
                    if let Some(kontakter_data) = &components.lpr3_kontakter {
                        if !kontakter_data.is_empty() {
                            // Extract kontakt_ids from kontakter data to filter procedurer
                            let mut kontakt_ids = Vec::new();

                            for batch in kontakter_data {
                                if let Ok(kontakt_id_idx) = batch.schema().index_of("kontakt_id") {
                                    if let Some(kontakt_id_array) = batch
                                        .column(kontakt_id_idx)
                                        .as_any()
                                        .downcast_ref::<StringArray>()
                                    {
                                        for i in 0..kontakt_id_array.len() {
                                            if !kontakt_id_array.is_null(i) {
                                                kontakt_ids
                                                    .push(kontakt_id_array.value(i).to_string());
                                            }
                                        }
                                    }
                                }
                            }

                            // Apply filter if we have kontakt_ids
                            if !kontakt_ids.is_empty() {
                                let kontakt_id_exprs: Vec<Expr> =
                                    kontakt_ids.iter().map(|id| lit(id.clone())).collect();

                                procedurer_df = procedurer_df
                                    .filter(col("kontakt_id").in_list(kontakt_id_exprs, false))?;
                            }
                        }
                    }

                    // Collect procedurer data
                    let procedurer_batches = procedurer_df.collect().await?;
                    if !procedurer_batches.is_empty() {
                        let record_count = procedurer_batches
                            .iter()
                            .map(|b| b.num_rows())
                            .sum::<usize>();
                        log::info!(
                            "Collected {} procedurer data batches with {} total records",
                            procedurer_batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr3_procedurer(
                                &abs_procedurer_path,
                                procedurer_batches.clone(),
                            );
                        }

                        components = components.with_lpr3_procedurer(procedurer_batches);
                    } else {
                        log::warn!("No procedurer data records found");
                    }
                } else {
                    log::warn!("Failed to register procedurer data using DataFusion");

                    // Try fallback method
                    // Fallback implementation - manually use parquet loader
                    let io_filter = pnr_filter.map(|f| f.to_io_filter());
                    if let Ok(batches) = crate::data::io::parquet::load_parquet_directory(
                        &abs_procedurer_path,
                        None, // Don't enforce schema
                        io_filter.as_ref(),
                    )
                    .await
                    {
                        let record_count = batches.iter().map(|b| b.num_rows()).sum::<usize>();
                        log::info!(
                            "Loaded {} procedurer batches with {} total records using fallback method",
                            batches.len(),
                            record_count
                        );

                        // Store in cache for future use
                        {
                            let mut cache = crate::data::registry::loaders::lpr::get_lpr_cache()
                                .lock()
                                .unwrap();
                            cache.store_lpr3_procedurer(&abs_procedurer_path, batches.clone());
                        }

                        components = components.with_lpr3_procedurer(batches);
                    } else {
                        log::warn!("Failed to load procedurer data using both DataFusion and fallback method");
                        // Continue without procedurer data
                    }
                }
            }
        }

        // Return the components
        log::info!("Completed loading LPR3 components");
        Ok(components)
    }
}
