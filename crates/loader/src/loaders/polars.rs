#[cfg(feature = "polars_backend")]
use crate::{
    IdsError,
    ArrowStore,
    StoreLoader,
    RegisterPathConfig,
    LoaderProgress,
};

#[cfg(feature = "polars_backend")]
use std::path::{Path, PathBuf};

/// Polars-backed data loader for improved performance
/// 
/// This loader uses Polars' lazy evaluation and predicate pushdown for better performance.
/// Note: Requires the "polars_backend" feature to be enabled.
#[cfg(feature = "polars_backend")]
pub struct PolarsLoader;

#[cfg(feature = "polars_backend")]
impl PolarsLoader {
    pub fn new() -> Self {
        Self
    }
    
    /// Load data using predicate pushdown and lazy evaluation
    /// 
    /// This will only load data that matches the filter criteria, reducing memory usage.
    pub fn load_with_pnr_filter(
        &self,
        base_path: String,
        pnr_filter: &[String],
    ) -> Result<ArrowStore, IdsError> {
        use polars::prelude::*;
        use polars::lazy::dsl::*;
        
        log::info!("Loading data with Polars backend and PNR filtering");
        
        // Convert paths to standard form based on the directory structure
        let base_dir = Path::new(&base_path);
        let paths = crate::utils::detect_data_structure(base_dir)?;
        
        // Create a PNR filter Series for predicate pushdown
        let pnr_series = Series::new("filter_values", pnr_filter);
        
        // Create a progress tracker
        let progress = LoaderProgress::new();
        
        // Create a new store (Arrow backend)
        // In a production version, we could create a PolarsBackend implementation
        let mut store = ArrowStore::new()?;
        
        // Load family data first
        if let Some(family_path) = paths.get("family") {
            let family_spinner = progress.start_with_spinner("Loading family data".to_string());
            
            // Create a lazy frame with predicate pushdown
            let family_lf = LazyFrame::scan_parquet(
                family_path.to_str().unwrap_or(""),
                ScanArgsParquet {
                    n_rows: None,
                    cache: true,
                    parallel: true,
                    ..Default::default()
                }
            )?;
            
            // Filter by PNR using predicate pushdown
            let filtered_lf = family_lf.filter(col("PNR").is_in(lit(pnr_series.clone())));
            
            // Execute the query and collect results
            let family_df = filtered_lf.collect()?;
            
            // Convert to Arrow record batches
            let batches = family_df.to_arrow_record_batch()?;
            
            family_spinner.finish_with_message(format!("Loaded family data with {} rows", family_df.height()));
            
            // Add to store
            store.load_family_relations(vec![batches])?;
        } else {
            return Err(IdsError::invalid_operation("Family relations file not found - required for loading"));
        }
        
        // Helper function to load register data with PNR filtering
        let load_register = |path: &Path, name: &str, year_or_period: &str| -> Result<(), IdsError> {
            let spinner = progress.start_with_spinner(format!("Loading {} {}", name, year_or_period));
            
            // Scan parquet with parallel processing and predicate pushdown
            if let Ok(lf) = LazyFrame::scan_parquet(
                path.to_str().unwrap_or(""),
                ScanArgsParquet {
                    n_rows: None,
                    cache: true,
                    parallel: true,
                    low_memory: false,
                    ..Default::default()
                }
            ) {
                // Filter by PNR
                let filtered_lf = lf.filter(col("PNR").is_in(lit(pnr_series.clone())));
                
                // Execute query with optimizations
                let df_result = filtered_lf.collect();
                
                match df_result {
                    Ok(df) => {
                        if df.height() > 0 {
                            // Convert to Arrow record batches
                            let batches = df.to_arrow_record_batch()?;
                            
                            // Add to appropriate store
                            match name {
                                "AKM" => {
                                    if let Ok(year) = year_or_period.parse::<i32>() {
                                        store.load_akm_data(year, &[batches])?;
                                    }
                                },
                                "BEF" => {
                                    store.load_bef_data(year_or_period.to_string(), &[batches])?;
                                },
                                "IND" => {
                                    if let Ok(year) = year_or_period.parse::<i32>() {
                                        store.load_ind_data(year, &[batches])?;
                                    }
                                },
                                "UDDF" => {
                                    store.load_uddf_data(year_or_period.to_string(), &[batches])?;
                                },
                                _ => {}
                            }
                            
                            spinner.finish_with_message(format!("Loaded {} {} with {} rows", name, year_or_period, df.height()));
                        } else {
                            spinner.finish_with_message(format!("No rows for {} {} after filtering", name, year_or_period));
                        }
                    },
                    Err(e) => {
                        spinner.finish_with_message(format!("Error loading {} {}: {}", name, year_or_period, e));
                        log::error!("Error loading {} {}: {}", name, year_or_period, e);
                    }
                }
            }
            
            Ok(())
        };
        
        // Load AKM data
        if let Some(akm_dir) = paths.get("akm") {
            let akm_dir = akm_dir.as_path();
            
            // Find all parquet files in AKM directory
            let akm_files = crate::utils::find_all_parquet_files(akm_dir, None)?;
            
            // Process each file
            for file in akm_files {
                if let Some(year_str) = crate::utils::path::extract_year_from_filename(&file) {
                    let _ = load_register(&file, "AKM", &year_str);
                }
            }
        }
        
        // Same pattern for BEF, IND, and UDDF
        if let Some(bef_dir) = paths.get("bef") {
            let bef_dir = bef_dir.as_path();
            let bef_files = crate::utils::find_all_parquet_files(bef_dir, None)?;
            
            for file in bef_files {
                if let Some(period) = crate::utils::path::extract_period_from_filename(&file) {
                    let _ = load_register(&file, "BEF", &period);
                }
            }
        }
        
        if let Some(ind_dir) = paths.get("ind") {
            let ind_dir = ind_dir.as_path();
            let ind_files = crate::utils::find_all_parquet_files(ind_dir, None)?;
            
            for file in ind_files {
                if let Some(year_str) = crate::utils::path::extract_year_from_filename(&file) {
                    let _ = load_register(&file, "IND", &year_str);
                }
            }
        }
        
        if let Some(uddf_dir) = paths.get("uddf") {
            let uddf_dir = uddf_dir.as_path();
            let uddf_files = crate::utils::find_all_parquet_files(uddf_dir, None)?;
            
            for file in uddf_files {
                if let Some(period) = crate::utils::path::extract_period_from_filename(&file) {
                    let _ = load_register(&file, "UDDF", &period);
                }
            }
        }
        
        log::info!("Polars data loading complete");
        Ok(store)
    }
}

#[cfg(feature = "polars_backend")]
impl StoreLoader for PolarsLoader {
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError> {
        // This is a placeholder implementation that delegates to ParquetLoader
        // In a real implementation, we would use Polars for loading
        log::warn!("Using Polars backend without PNR filtering may use a lot of memory");
        log::warn!("Consider using load_with_pnr_filter instead");
        
        let loader = crate::loaders::ParquetLoader::new();
        loader.load_from_path(base_path)
    }
    
    fn load_with_custom_paths(config: RegisterPathConfig) -> Result<ArrowStore, IdsError> {
        // This is a placeholder implementation that delegates to ParquetLoader
        // In a real implementation, we would use Polars for loading
        log::warn!("Using Polars backend without PNR filtering may use a lot of memory");
        log::warn!("Consider using load_with_pnr_filter instead");
        
        crate::loaders::ParquetLoader::load_with_custom_paths(config)
    }
}

// Stub implementation when polars_backend feature is disabled
#[cfg(not(feature = "polars_backend"))]
pub struct PolarsLoader;

#[cfg(not(feature = "polars_backend"))]
impl PolarsLoader {
    pub fn new() -> Self {
        Self
    }
    
    pub fn load_with_pnr_filter(
        &self,
        _base_path: String,
        _pnr_filter: &[String],
    ) -> Result<crate::ArrowStore, crate::IdsError> {
        Err(crate::IdsError::invalid_operation(
            "Polars backend is not enabled. Recompile with --features polars_backend".to_string()
        ))
    }
}

#[cfg(not(feature = "polars_backend"))]
impl crate::StoreLoader for PolarsLoader {
    fn load_from_path(_base_path: String) -> Result<crate::ArrowStore, crate::IdsError> {
        Err(crate::IdsError::invalid_operation(
            "Polars backend is not enabled. Recompile with --features polars_backend".to_string()
        ))
    }
    
    fn load_with_custom_paths(_config: crate::RegisterPathConfig) -> Result<crate::ArrowStore, crate::IdsError> {
        Err(crate::IdsError::invalid_operation(
            "Polars backend is not enabled. Recompile with --features polars_backend".to_string()
        ))
    }
}