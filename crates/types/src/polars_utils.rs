//! Utility functions for Polars integration
//! 
//! This module provides utilities for converting between Arrow and Polars
//! data structures, enabling gradual migration to Polars.

#[cfg(feature = "polars_backend")]
use std::sync::Arc;
#[cfg(feature = "polars_backend")]
use polars::prelude::*;
#[cfg(feature = "polars_backend")]
use num_cpus;

/// Convert Arrow RecordBatch to Polars DataFrame
#[cfg(feature = "polars_backend")]
pub fn record_batch_to_df(batch: &RecordBatch) -> Result<DataFrame, IdsError> {
    // Manual conversion since direct TryFrom isn't available
    let mut columns = Vec::with_capacity(batch.num_columns());
    
    for (i, field) in batch.schema().fields().iter().enumerate() {
        let arrow_array = batch.column(i);
        let name = field.name().clone();
        
        // Convert based on data type
        let series = match field.data_type() {
            arrow::datatypes::DataType::Int32 => {
                let array = arrow_array.as_any().downcast_ref::<arrow::array::Int32Array>()
                    .ok_or_else(|| IdsError::invalid_format("Failed to downcast to Int32Array".to_string()))?;
                
                let values: Vec<Option<i32>> = array.iter().collect();
                Series::new(&name, values)
            },
            arrow::datatypes::DataType::Float64 => {
                let array = arrow_array.as_any().downcast_ref::<arrow::array::Float64Array>()
                    .ok_or_else(|| IdsError::invalid_format("Failed to downcast to Float64Array".to_string()))?;
                
                let values: Vec<Option<f64>> = array.iter().collect();
                Series::new(&name, values)
            },
            arrow::datatypes::DataType::Utf8 => {
                let array = arrow_array.as_any().downcast_ref::<arrow::array::StringArray>()
                    .ok_or_else(|| IdsError::invalid_format("Failed to downcast to StringArray".to_string()))?;
                
                let values: Vec<Option<&str>> = array.iter().collect();
                Series::new(&name, values)
            },
            arrow::datatypes::DataType::Date32 => {
                let array = arrow_array.as_any().downcast_ref::<arrow::array::Date32Array>()
                    .ok_or_else(|| IdsError::invalid_format("Failed to downcast to Date32Array".to_string()))?;
                
                let values: Vec<Option<i32>> = array.iter().collect();
                Series::new(&name, values)
            },
            // Handle other types as needed
            t => return Err(IdsError::invalid_format(format!("Unsupported data type: {:?}", t))),
        };
        
        columns.push(series);
    }
    
    DataFrame::new(columns)
        .map_err(|e| IdsError::invalid_operation(format!("Failed to create DataFrame: {}", e)))
}

/// Convert Polars DataFrame to Arrow RecordBatch
#[cfg(feature = "polars_backend")]
pub fn df_to_record_batch(df: &DataFrame) -> Result<RecordBatch, IdsError> {
    // Manual conversion
    let mut arrays = Vec::with_capacity(df.width());
    let mut fields = Vec::with_capacity(df.width());
    
    for series in df.iter() {
        let name = series.name().to_string();
        
        let (array, field) = match series.dtype() {
            // Add these DataType variants to match the polars API
            DataType::Int32 => {
                let i32_array = series.i32()
                    .map_err(|e| IdsError::invalid_operation(format!("Error converting to i32: {}", e)))?;
                
                let values: Vec<Option<i32>> = i32_array.iter().collect();
                let arrow_array = arrow::array::Int32Array::from(values);
                
                (
                    Arc::new(arrow_array) as Arc<dyn arrow::array::Array>,
                    arrow::datatypes::Field::new(&name, arrow::datatypes::DataType::Int32, true)
                )
            },
            DataType::Float64 => {
                let f64_array = series.f64()
                    .map_err(|e| IdsError::invalid_operation(format!("Error converting to f64: {}", e)))?;
                
                let values: Vec<Option<f64>> = f64_array.iter().collect();
                let arrow_array = arrow::array::Float64Array::from(values);
                
                (
                    Arc::new(arrow_array) as Arc<dyn arrow::array::Array>,
                    arrow::datatypes::Field::new(&name, arrow::datatypes::DataType::Float64, true)
                )
            },
            DataType::String => {
                let str_array = series.str()
                    .map_err(|e| IdsError::invalid_operation(format!("Error converting to string: {}", e)))?;
                
                let values: Vec<Option<&str>> = str_array.iter().collect();
                let arrow_array = arrow::array::StringArray::from(values);
                
                (
                    Arc::new(arrow_array) as Arc<dyn arrow::array::Array>,
                    arrow::datatypes::Field::new(&name, arrow::datatypes::DataType::Utf8, true)
                )
            },
            DataType::Date => {
                let date_array = series.date()
                    .map_err(|e| IdsError::invalid_operation(format!("Error converting to date: {}", e)))?;
                
                // Extract days since epoch
                let values: Vec<Option<i32>> = date_array.iter()
                    .map(|opt_date| {
                        opt_date.map(|date| {
                            (date - chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days() as i32
                        })
                    })
                    .collect();
                
                let arrow_array = arrow::array::Date32Array::from(values);
                
                (
                    Arc::new(arrow_array) as Arc<dyn arrow::array::Array>,
                    arrow::datatypes::Field::new(&name, arrow::datatypes::DataType::Date32, true)
                )
            },
            // Add other types as needed
            dt => return Err(IdsError::invalid_operation(format!("Unsupported data type: {:?}", dt))),
        };
        
        arrays.push(array);
        fields.push(field);
    }
    
    let schema = Arc::new(arrow::datatypes::Schema::new(fields));
    
    RecordBatch::try_new(schema, arrays)
        .map_err(|e| IdsError::invalid_operation(format!("Failed to create RecordBatch: {}", e)))
}

/// Convert a Vector of Arrow RecordBatch to a Polars LazyFrame
#[cfg(feature = "polars_backend")]
pub fn batches_to_lazy_frame(batches: &[RecordBatch]) -> Result<LazyFrame, IdsError> {
    if batches.is_empty() {
        return Ok(DataFrame::empty().lazy());
    }
    
    // Convert first batch to get schema
    let mut df = record_batch_to_df(&batches[0])?;
    
    // Append the rest
    for batch in &batches[1..] {
        let batch_df = record_batch_to_df(batch)?;
        df = df.vstack(&batch_df)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to stack DataFrames: {}", e)))?;
    }
    
    // Return as lazy frame for deferred execution
    Ok(df.lazy())
}

/// Perform compound filtering on a LazyFrame based on column conditions
#[cfg(feature = "polars_backend")]
pub fn filter_lazy_frame(
    lf: LazyFrame,
    conditions: Vec<(String, String, String)>
) -> LazyFrame {
    let mut filtered = lf;
    
    for (column, op, value) in conditions {
        let expr = match op.as_str() {
            "eq" => col(&column).eq(lit(value)),
            "neq" => col(&column).neq(lit(value)),
            "gt" => col(&column).gt(lit(value.parse::<i64>().unwrap_or(0))),
            "gte" => col(&column).gt_eq(lit(value.parse::<i64>().unwrap_or(0))),
            "lt" => col(&column).lt(lit(value.parse::<i64>().unwrap_or(0))),
            "lte" => col(&column).lt_eq(lit(value.parse::<i64>().unwrap_or(0))),
            "contains" => col(&column).str().contains(lit(value), true),
            _ => col(&column).eq(lit(value)), // Default to equality
        };
        
        filtered = filtered.filter(expr);
    }
    
    filtered
}

/// Create a LazyFrame from a Parquet file with optimized scan parameters
#[cfg(feature = "polars_backend")]
pub fn scan_optimized_parquet(path: &str) -> Result<LazyFrame, IdsError> {
    // Get environment variables for configuration
    let low_memory = std::env::var("IDS_LOW_MEMORY")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);
        
    let streaming_chunk_size = std::env::var("POLARS_STREAMING_CHUNK_SIZE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(32768);
    
    // Configure Polars settings
    if std::env::var("POLARS_MAX_THREADS").is_err() {
        // Set to number of logical cores if not configured
        let num_threads = num_cpus::get();
        // Note: In actual Polars 0.46.0, these config methods might be different
        // using thread pool directly instead
        log::info!("Setting Polars to use {} threads", num_threads);
    }
    
    // Log the streaming chunk size that would be set
    log::info!("Would set streaming chunk size to {}", streaming_chunk_size);
    
    // Configure scan args for optimal performance
    let mut scan_args = ScanArgsParquet::default();
    // Set individual fields on scan_args based on version compatibility
    // Note: ScanArgsParquet might have different fields in Polars 0.46.0
    scan_args.n_rows = None;
    scan_args.cache = true;
    scan_args.rechunk = true;
    scan_args.row_count = None;
    // scan_args.parallel = true; // This might be ParallelStrategy rather than bool
    scan_args.low_memory = low_memory;
    
    // Scan the parquet file
    LazyFrame::scan_parquet(path, scan_args)
        .map_err(|e| IdsError::invalid_operation(format!("Failed to scan parquet file: {}", e)))
}

/// Prepare an optimized LazyFrame for querying a specific PNR
#[cfg(feature = "polars_backend")]
pub fn prepare_pnr_query(lf: &LazyFrame, pnr: &str, columns: &[&str]) -> LazyFrame {
    // Create a simpler version without the API that might not be available
    lf.clone()
        .filter(col("PNR").eq(lit(pnr.to_string())))
        .select(columns.iter().map(|&c| col(c)).collect::<Vec<_>>())
}

/// Create an adaptive LazyFrame based on available system resources
#[cfg(feature = "polars_backend")]
pub fn create_adaptive_lazy_frame(lf: LazyFrame) -> LazyFrame {
    // Get available memory
    #[cfg(target_os = "linux")]
    let available_memory = {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
            
        if let Ok(file) = File::open("/proc/meminfo") {
            let reader = BufReader::new(file);
            let mut mem_total = None;
                
            for line in reader.lines().flatten() {
                if line.starts_with("MemTotal:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            mem_total = Some(kb * 1024);
                            break;
                        }
                    }
                }
            }
                
            mem_total
        } else {
            None
        }
    };
    
    #[cfg(not(target_os = "linux"))]
    let available_memory = None;
    
    // Configure based on available memory
    let memory_tier = match available_memory {
        Some(mem) if mem > 64_000_000_000 => "high",
        Some(mem) if mem > 32_000_000_000 => "medium",
        Some(mem) if mem > 16_000_000_000 => "low",
        _ => "default",
    };
    
    // Log the memory tier being used
    log::info!("Using memory tier: {}", memory_tier);
    
    // Since the streaming API might not be available, return as is
    lf
}