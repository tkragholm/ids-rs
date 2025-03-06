use crate::LoaderProgress;
use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;
use indicatif::ProgressBar;

use crossbeam_channel::bounded;
use crossbeam_deque::{Injector, Steal, Worker};
use parking_lot::Mutex;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ProjectionMask;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use types::arrow_utils::{ArrowAccess, ArrowUtils};
use types::error::IdsError;

/// Reads a Parquet file and returns its contents as a vector of `RecordBatches`.
/// Uses a parallel processing pipeline with crossbeam channels and worker threads for better performance.
///
/// # Arguments
///
/// * `path` - A file path to the Parquet file to be read
/// * `schema` - An optional Arrow Schema for projecting specific columns
/// * `progress` - An optional progress tracker for user feedback
/// * `pnr_filter` - An optional set of PNRs to filter the data by
///
/// # Returns
///
/// A Result containing a vector of `RecordBatches` or an `IdsError`
///
/// # Errors
///
/// Returns an `IdsError` if:
/// - The file cannot be opened
/// - The file is not a valid Parquet file
/// - There are issues reading the record batches
pub fn read_parquet(
    path: &Path,
    schema: Option<&Schema>,
    progress: Option<&LoaderProgress>,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Attempting to read parquet file: {}", path.display());

    // Check if the file exists
    if !path.exists() {
        log::error!("File not found: {}", path.display());
        return Err(IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path.display()),
        )));
    }

    // Try to get the canonical path to better understand where we're looking
    let canonical_path = match path.canonicalize() {
        Ok(p) => {
            log::debug!(
                "Canonical path resolved: {} -> {}",
                path.display(),
                p.display()
            );
            p
        }
        Err(e) => {
            log::warn!(
                "Unable to resolve canonical path for {}: {}",
                path.display(),
                e
            );
            path.to_path_buf()
        }
    };

    // Double-check if it's a file
    if !path.is_file() {
        log::error!("Path exists but is not a file: {}", path.display());
        return Err(IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path is not a file: {}", path.display()),
        )));
    }

    // Open the file
    let file = match File::open(path) {
        Ok(f) => {
            log::debug!("Successfully opened file: {}", canonical_path.display());
            f
        }
        Err(e) => {
            log::error!("Error opening file {}: {}", canonical_path.display(), e);
            return Err(IdsError::Io(e));
        }
    };

    // Get file size for progress reporting
    let file_size = match file.metadata() {
        Ok(meta) => {
            let size = meta.len();
            log::debug!("File size: {} bytes", size);
            size
        }
        Err(e) => {
            log::warn!("Unable to get file size for {}: {}", path.display(), e);
            0
        }
    };

    let pb = progress.map(|p| {
        p.create_file_progress(
            file_size,
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown"),
        )
    });

    let builder = match ParquetRecordBatchReaderBuilder::try_new(file) {
        Ok(b) => b,
        Err(e) => {
            log::error!(
                "Failed to create ParquetRecordBatchReaderBuilder for {}: {}",
                path.display(),
                e
            );
            return Err(IdsError::invalid_format(format!(
                "Invalid Parquet file format at {}: {}",
                path.display(),
                e
            )));
        }
    };

    // Get batch size from environment variable or use a larger default
    // This allows tuning based on server capabilities
    let batch_size = std::env::var("IDS_BATCH_SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(65536); // 4x larger than previous default
    
    log::debug!("Using batch size of {} rows for Parquet loading", batch_size);

    // Create the reader
    let reader = match schema {
        Some(s) => {
            // Safety check to prevent index out of bounds error
            let schema_len = s.fields().len();
            // Only include indices that are within the range of the Parquet schema
            let parquet_schema = builder.parquet_schema();

            // Get the root field count in a schema-descriptor safe way
            // Get count of root fields in the schema
            let root_schema = parquet_schema.root_schema();
            let parquet_schema_len = root_schema.get_fields().len();

            // Verify and log schema lengths
            log::debug!(
                "Arrow schema field count: {}, Parquet schema field count: {}",
                schema_len,
                parquet_schema_len
            );

            // Create a safe projection that only includes fields that exist in both schemas
            let safe_indices: Vec<usize> = (0..schema_len)
                .filter(|i| *i < parquet_schema_len)
                .collect();

            // Thread-local counter for schema mismatch warnings
            thread_local! {
                static SCHEMA_MISMATCH_WARNING_COUNT: std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
            }

            // Check for schema mismatch
            if safe_indices.len() < schema_len {
                // Only log the first schema mismatch warning per thread
                SCHEMA_MISMATCH_WARNING_COUNT.with(|count| {
                    let mut count = count.borrow_mut();
                    if *count == 0 {
                        // First occurrence - log as warning but with minimal details
                        log::warn!("Schema mismatch detected: Arrow schema has more fields than Parquet schema. Using only common fields.");
                        // More detailed message at debug level
                        log::debug!("Schema details: Arrow has {} fields, Parquet has {} fields",
                                  schema_len, parquet_schema_len);
                    } else if *count == 1 {
                        // Second occurrence - notify at info level that further warnings will be suppressed
                        log::info!("Additional schema mismatches will not be logged (total: {})", *count + 1);
                    }
                    *count += 1;
                });
            }

            let mask = ProjectionMask::roots(parquet_schema, safe_indices);
            builder
                .with_batch_size(batch_size)
                .with_projection(mask)
                .build()
                .map_err(|e| {
                    IdsError::invalid_format(format!("Failed to create Parquet reader: {}", e))
                })?
        }
        None => builder.with_batch_size(batch_size).build().map_err(|e| {
            IdsError::invalid_format(format!("Failed to create Parquet reader: {}", e))
        })?,
    };

    // Get number of worker threads from environment variable or use all available CPUs
    // This allows scaling to high-core servers without artificial limits
    let num_workers = std::env::var("IDS_MAX_THREADS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(num_cpus::get)
        .max(2); // At least 2 workers for parallelism
    
    log::debug!("Using {} worker threads for Parquet batch processing", num_workers);

    // Set up a parallel processing pipeline for batches
    // Using a work-stealing queue for dynamic load balancing across workers
    let (sender, receiver) = bounded::<Result<RecordBatch, String>>(num_workers * 4); 
    let global_injector = Arc::new(Injector::new());
    let batches_result = Arc::new(Mutex::new(Vec::new()));
    let error_result = Arc::new(Mutex::new(None));
    let pb_shared = pb.clone();

    // Create a pool of worker threads to process batches in parallel
    let worker_handles: Vec<_> = (0..num_workers)
        .map(|worker_id| {
            // Clone necessary resources for each worker
            let receiver = receiver.clone();
            let local_worker = Worker::new_fifo();
            let global_injector = global_injector.clone();
            let batches_result = batches_result.clone();
            let error_result = error_result.clone();
            let pb_clone = pb_shared.clone();
            let pnr_filter = pnr_filter.cloned();

            thread::spawn(move || {
                log::debug!("Worker thread {} started", worker_id);
                let utils = ArrowUtils;
                
                // Process tasks from the local worker queue, global injector, and other workers
                'worker_loop: loop {
                    // Check local queue first
                    if let Some(batch) = local_worker.pop() {
                        if let Err(e) = process_batch(batch, &utils, &pb_clone, &batches_result, &error_result, &pnr_filter) {
                            // Store the error and break out of the worker loop
                            let mut error = error_result.lock();
                            *error = Some(format!("Error processing batch: {}", e));
                            break 'worker_loop;
                        }
                        continue;
                    }
                    
                    // Check global queue next
                    match global_injector.steal() {
                        Steal::Success(batch) => {
                            if let Err(e) = process_batch(batch, &utils, &pb_clone, &batches_result, &error_result, &pnr_filter) {
                                // Store the error and break out of the worker loop
                                let mut error = error_result.lock();
                                *error = Some(format!("Error processing batch: {}", e));
                                break 'worker_loop;
                            }
                            continue;
                        }
                        Steal::Empty => {
                            // If global queue is empty, check if we have new batches from the channel
                            match receiver.try_recv() {
                                Ok(batch_result) => match batch_result {
                                    Ok(batch) => {
                                        if let Err(e) = process_batch(batch, &utils, &pb_clone, &batches_result, &error_result, &pnr_filter) {
                                            // Store the error and break out of the worker loop
                                            let mut error = error_result.lock();
                                            *error = Some(format!("Error processing batch: {}", e));
                                            break 'worker_loop;
                                        }
                                    }
                                    Err(error_msg) => {
                                        let mut error = error_result.lock();
                                        *error = Some(error_msg);
                                        break 'worker_loop;
                                    }
                                },
                                Err(crossbeam_channel::TryRecvError::Empty) => {
                                    // No tasks available right now, wait for a new batch
                                    match receiver.recv() {
                                        Ok(batch_result) => match batch_result {
                                            Ok(batch) => {
                                                if let Err(e) = process_batch(batch, &utils, &pb_clone, &batches_result, &error_result, &pnr_filter) {
                                                    // Store the error and break out of the worker loop
                                                    let mut error = error_result.lock();
                                                    *error = Some(format!("Error processing batch: {}", e));
                                                    break 'worker_loop;
                                                }
                                            }
                                            Err(error_msg) => {
                                                let mut error = error_result.lock();
                                                *error = Some(error_msg);
                                                break 'worker_loop;
                                            }
                                        },
                                        Err(_) => {
                                            // Channel is closed, exit
                                            break 'worker_loop;
                                        }
                                    }
                                }
                                Err(crossbeam_channel::TryRecvError::Disconnected) => {
                                    // Channel is closed, exit
                                    break 'worker_loop;
                                }
                            }
                        }
                        Steal::Retry => {
                            // Retry stealing from global queue
                            continue;
                        }
                    }
                }
                log::debug!("Worker thread {} finished", worker_id);
            })
        })
        .collect();

    // Create a thread to feed batches from the reader into the worker pool
    let feeder = thread::spawn(move || {
        for batch_result in reader {
            match batch_result {
                Ok(batch) => {
                    // Send the batch for processing
                    if sender.send(Ok(batch)).is_err() {
                        log::error!("Failed to send batch to worker threads - channel closed");
                        break;
                    }
                }
                Err(e) => {
                    let error_msg = format!("Failed to read batch: {}", e);
                    // Signal error to workers
                    let _ = sender.send(Err(error_msg.clone()));
                    break;
                }
            }
        }
        // Drop the sender to signal to the workers that we're done
        drop(sender);
    });

    // Wait for feeder to finish
    feeder.join().expect("Feeder thread panicked");
    
    // Wait for all workers to finish
    for (i, handle) in worker_handles.into_iter().enumerate() {
        if let Err(e) = handle.join() {
            log::error!("Worker thread {} panicked: {:?}", i, e);
        }
    }

    // Check for errors
    {
        let error_lock = error_result.lock();
        if let Some(error_msg) = &*error_lock {
            if let Some(pb) = pb {
                pb.finish_with_message("Error");
            }
            return Err(IdsError::invalid_format(error_msg.clone()));
        }
    }

    // Get the results
    let batches = {
        let batches_lock = batches_result.lock();
        batches_lock.clone()
    };

    if let Some(pb) = pb {
        pb.finish_with_message("Complete");
    }
    
    log::info!("Successfully read {} batches from {}", batches.len(), path.display());
    Ok(batches)
}

/// Helper function to process a single batch with optional PNR filtering
fn process_batch(
    batch: RecordBatch,
    utils: &ArrowUtils,
    pb: &Option<ProgressBar>,
    batches_result: &Arc<Mutex<Vec<RecordBatch>>>,
    _error_result: &Arc<Mutex<Option<String>>>,
    pnr_filter: &Option<HashSet<String>>,
) -> Result<(), IdsError> {
    // Filter by PNR if needed
    let batch = if let Some(filter) = pnr_filter {
        // Apply PNR filtering
        match filter_batch_by_pnr(&batch, filter) {
            Some(filtered) => filtered,
            None => return Ok(()), // Skip this batch if it has no matching rows
        }
    } else {
        batch
    };

    // Validate batch
    if let Err(e) = utils.validate_batch(&batch) {
        log::warn!("Parquet batch validation warning: {}", e);
    }

    // Optimize memory layout
    #[allow(clippy::unnecessary_mut_passed)]
    let aligned_batch = ArrowUtils::align_batch_buffers(&batch)?;

    if let Some(pb) = pb {
        pb.inc(aligned_batch.get_array_memory_size() as u64);
    }

    // Add to results
    let mut batches = batches_result.lock();
    batches.push(aligned_batch);
    
    Ok(())
}

/// Filter a RecordBatch by a set of PNRs
fn filter_batch_by_pnr(batch: &RecordBatch, pnr_set: &HashSet<String>) -> Option<RecordBatch> {
    use arrow::array::{Array, StringArray};
    use arrow::compute::filter_record_batch;
    
    // Find PNR column in batch
    let pnr_column_names = ["PNR", "pnr", "child_pnr", "child_id"];
    let pnr_idx = pnr_column_names.iter()
        .find_map(|name| batch.schema().index_of(name).ok());
        
    if let Some(idx) = pnr_idx {
        if let Some(array) = batch.column(idx).as_any().downcast_ref::<StringArray>() {
            // Create a filter mask
            let mut mask_builder = arrow::array::BooleanBuilder::new();
            let mut match_count = 0;
            
            for i in 0..array.len() {
                let matches = if !array.is_null(i) {
                    let pnr = array.value(i);
                    pnr_set.contains(pnr)
                } else {
                    false
                };
                
                if matches {
                    match_count += 1;
                }
                
                mask_builder.append_value(matches);
            }
            
            // If no rows match, return None to skip this batch
            if match_count == 0 {
                return None;
            }
            
            let mask = mask_builder.finish();
            
            // Apply the filter to the batch
            match filter_record_batch(batch, &mask) {
                Ok(filtered) => {
                    log::debug!("PNR filtering kept {}/{} rows", filtered.num_rows(), batch.num_rows());
                    return Some(filtered);
                },
                Err(e) => {
                    log::error!("Error applying PNR filter: {}", e);
                    return Some(batch.clone());
                }
            }
        }
    }
    
    // If we couldn't filter (no PNR column found), just return the original batch
    Some(batch.clone())
}

/// Load multiple parquet files in parallel
///
/// # Arguments
///
/// * `files` - List of paths to parquet files
/// * `schema` - Optional schema for column selection
/// * `progress` - Optional progress tracking
/// * `pnr_filter` - Optional PNR filtering set
///
/// # Returns
///
/// A map from file names to record batches
pub fn load_parquet_files_parallel(
    files: &[PathBuf],
    schema: Option<&Schema>,
    progress: Option<&LoaderProgress>,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<HashMap<String, Vec<RecordBatch>>, IdsError> {
    if files.is_empty() {
        return Ok(HashMap::new());
    }
    
    log::info!("Loading {} parquet files in parallel", files.len());
    
    // Create progress tracker for overall operation
    let main_progress = progress.map(|p| 
        p.create_main_progress(files.len() as u64, "Loading files".to_string())
    );
    
    // Use rayon to process files in parallel
    let results: HashMap<String, Vec<RecordBatch>> = files.par_iter()
        .map(|path| {
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
                
            match read_parquet(path, schema, progress, pnr_filter) {
                Ok(batches) => {
                    log::info!("Successfully loaded {} with {} batches", filename, batches.len());
                    if let Some(p) = &main_progress {
                        p.inc(1);
                    }
                    Ok((filename, batches))
                },
                Err(e) => {
                    log::error!("Failed to load {}: {}", filename, e);
                    if let Some(p) = &main_progress {
                        p.inc(1);
                    }
                    Err(e)
                }
            }
        })
        .filter_map(Result::ok)
        .collect();
        
    if let Some(p) = main_progress {
        p.finish_with_message(format!("Loaded {}/{} files", results.len(), files.len()));
    }
    
    Ok(results)
}


/// Filter a list of batches by a date range
/// 
/// # Arguments
///
/// * `batches` - The list of batches to filter
/// * `date_column` - The name of the date column
/// * `start_date` - The start date (inclusive)
/// * `end_date` - The end date (inclusive, optional)
///
/// # Returns
///
/// A Result containing filtered batches or an `IdsError`
pub fn filter_batches_by_date_range(
    batches: &[RecordBatch],
    date_column: &str,
    start_date: chrono::NaiveDate,
    end_date: Option<chrono::NaiveDate>,
) -> Result<Vec<RecordBatch>, IdsError> {
    // Early return for empty input
    if batches.is_empty() {
        return Ok(Vec::new());
    }

    // Use rayon's parallel iterator for better performance and simpler code
    // Process all batches in parallel with optimal thread utilization
    let filtered_batches: Result<Vec<_>, IdsError> = batches
        .par_iter()
        .map(|batch| {
            let utils = ArrowUtils;
            
            // Validate the batch
            if let Err(e) = utils.validate_batch(batch) {
                log::warn!("Batch validation warning before filtering: {}", e);
            }
            
            // Apply the date filter
            match utils.filter_batch_by_date_range(batch, date_column, start_date, end_date) {
                Ok(Some(filtered)) => {
                    // Optimize memory layout
                    #[allow(clippy::unnecessary_mut_passed)]
                    let optimized = ArrowUtils::align_batch_buffers(&filtered);
                    Ok(Some(optimized))
                }
                Ok(None) => Ok(None), // No rows matched the filter
                Err(e) => Err(e),
            }
        })
        .try_fold(
            Vec::new,                   // Initialize an empty vector for each thread
            |mut acc, batch_result| {   // Accumulate results within each thread
                match batch_result {
                    Ok(Some(batch)) => {
                        acc.push(batch);
                        Ok(acc)
                    }
                    Ok(None) => Ok(acc), // Skip empty batches
                    Err(e) => Err(e),
                }
            },
        )
        .try_reduce(
            Vec::new,                 // Initialize an empty vector for the final reduction
            |mut a, mut b| {          // Combine results from all threads
                a.append(&mut b);
                Ok(a)
            },
        );
    
    // Flatten the result to handle the nested Results
    filtered_batches.map(|batches_with_results| {
        // Collect and propagate any errors
        let mut flattened = Vec::with_capacity(batches_with_results.len());
        for batch_result in batches_with_results {
            flattened.push(batch_result?);
        }
        Ok(flattened)
    })?
}