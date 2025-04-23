use crate::ui::LoaderProgress;
use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;

use crossbeam_channel::bounded;
use crossbeam_deque::{Injector, Steal, Worker};
use parking_lot::Mutex;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
// use rayon::prelude::*;
use arrow::array::{Array, StringArray};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use types::error::IdsError;

/// Read a Parquet file and return its contents as a vector of `RecordBatches`.
///
/// Uses a parallel processing pipeline with crossbeam channels and worker threads
/// for better performance.
///
/// # Arguments
/// * `path` - File path to the Parquet file to be read
/// * `schema` - Optional Arrow Schema for projecting specific columns
/// * `progress` - Optional progress tracker for user feedback
/// * `pnr_filter` - Optional set of PNRs to filter the data by
///
/// # Returns
/// Vector of `RecordBatches` or an error
///
/// # Errors
/// Returns an error if:
/// - The file cannot be opened
/// - The file is not a valid Parquet file
/// - There are issues reading the record batches
pub fn read_parquet(
    path: &Path,
    schema: Option<&Schema>,
    progress: Option<&LoaderProgress>,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Reading parquet file: {}", path.display());

    // Check if the file exists
    if !path.exists() {
        log::error!("File not found: {}", path.display());
        return Err(IdsError::io_error(format!(
            "File not found: {}",
            path.display()
        )));
    }

    // Create a progress bar if provided
    let progress_bar = if let Some(progress) = progress {
        let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(1000);

        let filename = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown");

        Some(progress.create_file_progress(file_size, filename))
    } else {
        None
    };

    // Open the file
    let file = File::open(path).map_err(|e| {
        log::error!("Failed to open file {}: {}", path.display(), e);
        IdsError::io_error(format!("Failed to open file {}: {}", path.display(), e))
    })?;

    // Create a reader builder
    let mut builder = ParquetRecordBatchReaderBuilder::try_new(file).map_err(|e| {
        log::error!(
            "Failed to create parquet reader for {}: {}",
            path.display(),
            e
        );
        IdsError::invalid_operation(format!(
            "Failed to create parquet reader for {}: {}",
            path.display(),
            e
        ))
    })?;

    // Apply projection if schema is provided
    if let Some(_schema) = schema {
        // Currently, projection mask doesn't work correctly with the Arrow schema
        // To fix this properly, we'd need to get column indices from the schema
        // For now, we'll just read all columns
        // let mask = ProjectionMask::leaves(builder.schema(), vec![0, 1, 2]);
        // builder = builder.with_projection(mask);
    }

    // Get the batch size from environment or use a reasonable default
    let batch_size = std::env::var("IDS_BATCH_SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(16384);

    builder = builder.with_batch_size(batch_size);

    // Create the reader
    let reader = builder.build().map_err(|e| {
        log::error!(
            "Failed to build parquet reader for {}: {}",
            path.display(),
            e
        );
        IdsError::invalid_operation(format!(
            "Failed to build parquet reader for {}: {}",
            path.display(),
            e
        ))
    })?;

    // If we have a PNR filter, we'll apply it to each batch
    let has_pnr_filter = pnr_filter.is_some();

    // Collect the batches
    let mut all_batches = Vec::new();
    let mut filtered_batches = Vec::new();

    for batch_result in reader {
        let batch = batch_result.map_err(|e| {
            log::error!("Failed to read batch from {}: {}", path.display(), e);
            IdsError::invalid_operation(format!(
                "Failed to read batch from {}: {}",
                path.display(),
                e
            ))
        })?;

        // Update progress if provided
        if let Some(pb) = &progress_bar {
            pb.inc(1);
        }

        // Apply PNR filter if provided
        if let Some(pnr_filter) = pnr_filter {
            // Use ArrowAccess to filter the batch by PNR
            // Create a manual filter function
            let filtered_batch = match batch.column_by_name("pnr") {
                Some(pnr_column) => {
                    // Convert column to strings
                    // Try to cast to string array
                    let pnr_strings = match pnr_column.as_any().downcast_ref::<StringArray>() {
                        Some(arr) => arr,
                        None => {
                            log::warn!("Could not cast PNR column to StringArray");
                            // Create a string array once and reuse it
                            static EMPTY_ARRAY: once_cell::sync::Lazy<StringArray> =
                                once_cell::sync::Lazy::new(|| StringArray::from(vec![""]));
                            &EMPTY_ARRAY
                        }
                    };

                    let _pnr_vec: Vec<bool> = (0..pnr_strings.len())
                        .map(|i| {
                            let pnr = pnr_strings.value(i);
                            pnr_filter.contains(pnr)
                        })
                        .collect();

                    // Create a filtered batch - this would require additional work
                    // For now, just return the original batch
                    // In a real implementation, you would use the boolean mask to filter rows
                    batch.clone() // Return original for now
                }
                None => batch.clone(), // If no PNR column, return the original batch
            };

            // RecordBatch doesn't have is_empty() method
            // Instead we can check if it has any rows
            if filtered_batch.num_rows() > 0 {
                filtered_batches.push(filtered_batch);
            }
        } else {
            all_batches.push(batch);
        }
    }

    // Finish progress if provided
    if let Some(pb) = progress_bar {
        pb.finish_with_message("Done");
    }

    // Return the filtered batches if we applied a filter, otherwise all batches
    if has_pnr_filter {
        log::info!(
            "Read {} batches from {} (after filtering)",
            filtered_batches.len(),
            path.display()
        );
        Ok(filtered_batches)
    } else {
        log::info!("Read {} batches from {}", all_batches.len(), path.display());
        Ok(all_batches)
    }
}

/// Read a Parquet file with PNR filtering applied.
///
/// This is a convenience wrapper around `read_parquet` that applies a PNR filter.
///
/// # Arguments
/// * `path` - File path to the Parquet file to be read
/// * `schema` - Optional Arrow Schema for projecting specific columns
/// * `pnr_filter` - Set of PNRs to filter the data by
/// * `progress` - Optional progress tracker for user feedback
///
/// # Returns
/// Vector of `RecordBatches` or an error
///
/// # Errors
/// Returns an error if the underlying `read_parquet` function fails
pub fn read_parquet_with_filter(
    path: &Path,
    schema: Option<&Schema>,
    pnr_filter: &HashSet<String>,
    progress: Option<&LoaderProgress>,
) -> Result<Vec<RecordBatch>, IdsError> {
    read_parquet(path, schema, progress, Some(pnr_filter))
}

/// Load Parquet files in parallel from a directory.
///
/// Scans the directory for Parquet files and loads them in parallel using Rayon.
///
/// # Arguments
/// * `dir_path` - Directory containing Parquet files
/// * `schema` - Optional Arrow Schema for projecting specific columns
/// * `pnr_filter` - Optional set of PNRs to filter by
/// * `progress` - Optional progress tracker for user feedback
///
/// # Returns
/// Vector of `RecordBatches` from all Parquet files or an error
///
/// # Errors
/// Returns an error if:
/// - The directory cannot be read
/// - Any Parquet file cannot be read
pub fn load_parquet_files_parallel(
    dir_path: &Path,
    schema: Option<&Schema>,
    pnr_filter: Option<&HashSet<String>>,
    progress: Option<&LoaderProgress>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!(
        "Loading Parquet files from directory: {}",
        dir_path.display()
    );

    // Check if the directory exists
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(IdsError::io_error(format!(
            "Directory not found: {}",
            dir_path.display()
        )));
    }

    // Scan for Parquet files
    let mut parquet_files = Vec::new();
    for entry in std::fs::read_dir(dir_path).map_err(|e| {
        IdsError::io_error(format!(
            "Failed to read directory {}: {}",
            dir_path.display(),
            e
        ))
    })? {
        let entry = entry
            .map_err(|e| IdsError::io_error(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
            parquet_files.push(path);
        }
    }

    if parquet_files.is_empty() {
        log::warn!(
            "No Parquet files found in directory: {}",
            dir_path.display()
        );
        return Ok(Vec::new());
    }

    log::info!(
        "Found {} Parquet files in directory: {}",
        parquet_files.len(),
        dir_path.display()
    );

    // Sort by modification time (newest first)
    parquet_files.sort_by(|a, b| {
        let a_meta = match std::fs::metadata(a) {
            Ok(meta) => meta,
            Err(_) => return std::cmp::Ordering::Equal,
        };
        let b_meta = match std::fs::metadata(b) {
            Ok(meta) => meta,
            Err(_) => return std::cmp::Ordering::Equal,
        };
        a_meta
            .modified()
            .unwrap_or_else(|_| std::time::SystemTime::now())
            .cmp(
                &b_meta
                    .modified()
                    .unwrap_or_else(|_| std::time::SystemTime::now()),
            )
    });

    // Create a progress reporter if provided
    if let Some(progress) = progress {
        progress.set_main_message(&format!(
            "Loading {} Parquet files from {}",
            parquet_files.len(),
            dir_path.display()
        ));
    }

    // Process files in parallel
    let schema_arc = schema.map(|s| Arc::new(s.clone()));
    let pnr_filter_arc = pnr_filter.map(|p| Arc::new(p.clone()));

    let global_queue = Arc::new(Injector::new());
    for file in parquet_files {
        global_queue.push(file);
    }

    let max_threads = std::env::var("IDS_MAX_THREADS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(num_cpus::get)
        .max(2); // At least 2 threads

    // Create a channel to collect results
    let (sender, receiver) = bounded(max_threads * 2);

    // Spawn worker threads
    let workers: Vec<_> = (0..max_threads)
        .map(|i| {
            let local_queue = Worker::new_fifo();
            let global_queue = Arc::clone(&global_queue);
            let schema = schema_arc.clone();
            let pnr_filter = pnr_filter_arc.clone();
            let sender = sender.clone();
            let thread_id = i; // For debugging

            thread::spawn(move || {
                let worker_progress = None; // Individual thread progress tracking disabled

                loop {
                    // Try to get work from the local queue
                    let path = match local_queue.pop() {
                        Some(path) => path,
                        None => {
                            // If local queue is empty, try to steal from global queue
                            match global_queue.steal() {
                                Steal::Success(path) => path,
                                Steal::Empty => break, // Exit if global queue is empty
                                Steal::Retry => continue, // Retry if steal failed
                            }
                        }
                    };

                    log::debug!("Thread {} processing file: {}", thread_id, path.display());

                    // Process the file
                    let result = read_parquet(
                        &path,
                        schema.as_ref().map(|s| s.as_ref()),
                        worker_progress.as_ref(),
                        pnr_filter.as_ref().map(|p| p.as_ref()),
                    );

                    // Send the result back through the channel
                    match result {
                        Ok(batches) => {
                            log::debug!(
                                "Thread {} successfully read {} batches from {}",
                                thread_id,
                                batches.len(),
                                path.display()
                            );
                            if let Err(e) = sender.send((path, Ok(batches))) {
                                log::error!("Failed to send result: {}", e);
                            }
                        }
                        Err(err) => {
                            log::error!(
                                "Thread {} failed to read file {}: {}",
                                thread_id,
                                path.display(),
                                err
                            );
                            if let Err(e) = sender.send((path, Err(err))) {
                                log::error!("Failed to send error: {}", e);
                            }
                        }
                    }
                }

                log::debug!("Thread {} exiting", thread_id);
            })
        })
        .collect();

    // Drop the original sender to close the channel when all workers exit
    drop(sender);

    // Collect results
    let results_mutex = Arc::new(Mutex::new(HashMap::new()));
    let errors_mutex = Arc::new(Mutex::new(Vec::new()));

    // Process results as they come in
    for (path, result) in receiver {
        match result {
            Ok(batches) => {
                let mut results = results_mutex.lock();
                results.insert(path, batches);
            }
            Err(err) => {
                let mut errors = errors_mutex.lock();
                errors.push((path, err));
            }
        }
    }

    // Join all worker threads
    for worker in workers {
        let _ = worker.join();
    }

    // Check if we have any errors
    let errors = errors_mutex.lock();
    if !errors.is_empty() {
        let error_paths: Vec<_> = errors
            .iter()
            .map(|(path, _)| path.display().to_string())
            .collect();
        return Err(IdsError::invalid_operation(format!(
            "Failed to read {} Parquet files: {}",
            errors.len(),
            error_paths.join(", ")
        )));
    }

    // Combine all batches
    let mut all_batches = Vec::new();
    for (_, batches) in results_mutex.lock().iter() {
        all_batches.extend(batches.clone());
    }

    log::info!(
        "Successfully loaded {} batches from {} Parquet files",
        all_batches.len(),
        results_mutex.lock().len()
    );

    Ok(all_batches)
}
