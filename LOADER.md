# IDS-RS Register Loading Optimization Strategy

This document outlines a comprehensive approach to optimize register data loading in the ids-rs project, particularly for Windows servers with slower I/O but abundant memory resources.

## Performance Challenges

The current register loading process faces several challenges:

1. **I/O Bottlenecks**: Loading large parquet files from slower storage can be a bottleneck
2. **Thread Limitations**: The current implementation caps thread usage at 16 threads
3. **Memory Inefficiency**: Loading all register data when only a subset is needed
4. **Windows-Specific Issues**: Performance characteristics differ on Windows servers

## Phase 1: Quick-Win Optimizations

These changes provide significant performance improvements with minimal code changes.

### 1. Increase Batch Size and Thread Count

```rust
// In crates/loader/src/parquet.rs

// Change this line (around line 134):
let batch_size = 16384;

// To a configurable version:
let batch_size = std::env::var("IDS_BATCH_SIZE")
    .ok()
    .and_then(|s| s.parse::<usize>().ok())
    .unwrap_or(65536); // 4x larger default

log::info!("Using batch size of {} rows for Parquet loading", batch_size);

// Change this line (around line 200):
let num_workers = num_cpus::get().clamp(2, 16); // At least 2, at most 16

// To remove the upper limit:
let num_workers = std::env::var("IDS_MAX_THREADS")
    .ok()
    .and_then(|s| s.parse::<usize>().ok())
    .unwrap_or_else(|| num_cpus::get())
    .max(2); // At least 2 workers for parallelism

log::info!("Using {} worker threads for Parquet batch processing", num_workers);
```

Benefits:
- Reduces I/O operations through larger batches
- Utilizes all available CPU cores on high-core servers
- Configurable via environment variables for easy tuning

### 2. Implement Family-Based PNR Filtering

Create a new module for the optimized loader (`optimized_loader.rs`) that:

1. Loads only the family relations file first
2. Extracts all relevant PNRs (children, mothers, fathers)
3. Uses these PNRs to filter the remaining register files

This approach:
- Dramatically reduces memory usage by only loading relevant records
- Preserves all required family relationships
- Improves processing speed by reducing the dataset size

Key implementation components:
```rust
/// Extract PNRs from family data
fn extract_pnrs_from_family_batches(
    family_batches: &[arrow::record_batch::RecordBatch]
) -> Result<HashSet<String>, IdsError> {
    let mut pnr_set = HashSet::new();
    
    // Extract child PNRs
    // Extract mother PNRs
    // Extract father PNRs
    
    Ok(pnr_set)
}

/// Load with family-based PNR filtering
pub fn load_with_family_filtering(
    base_path: &str
) -> Result<ArrowStore, IdsError> {
    // 1. Find and load family file
    // 2. Extract PNRs from family data
    // 3. Load other register files with PNR filtering
    
    Ok(store)
}
```

### 3. Add CLI Support for Optimized Loading

Add command-line flags to the IDS CLI:

```rust
/// Check balance with optimized loading (uses more memory but faster)
#[arg(long, help = "Use optimized loading with increased batch size and thread count")]
optimized_loading: bool,

/// Use family-based filtering for optimized loading
#[arg(long, help = "Filter register data based on family relationships to reduce memory")]
family_based_filtering: bool,
```

## Phase 2: Advanced Optimizations

For even greater performance, consider these advanced optimizations:

### 1. Implement Rayon-based Parallel File Loading

Load multiple register files simultaneously using Rayon:

```rust
/// Load multiple register files in parallel
pub fn load_files_in_parallel(
    file_paths: &[PathBuf],
    schema: Option<&arrow::datatypes::Schema>,
    pnr_filter: Option<&HashSet<String>>
) -> Result<HashMap<String, Vec<arrow::record_batch::RecordBatch>>, IdsError> {
    use rayon::prelude::*;
    
    // Process all files in parallel using Rayon
    let results = file_paths.par_iter()
        .map(|path| {
            // Load file with PNR filtering
            // Return (filename, batches)
        })
        .collect();
    
    Ok(results)
}
```

### 2. Add Memory-Mapping for Better Windows Performance

For Windows-specific optimizations:

```rust
// In a Windows-specific utility module
pub fn open_with_memory_mapping(path: &Path) -> Result<File, std::io::Error> {
    use std::os::windows::fs::OpenOptionsExt;
    use std::fs::{File, OpenOptions};
    use winapi::um::winbase::FILE_FLAG_SEQUENTIAL_SCAN;
    
    OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_SEQUENTIAL_SCAN) // Optimize for sequential reads
        .open(path)
}
```

### 3. Implement Chunked Processing for Huge Files

For extremely large files:

```rust
/// Process a large file in chunks to avoid memory issues
pub fn process_large_file_in_chunks(
    path: &Path,
    chunk_size: usize,
    process_fn: impl Fn(&[arrow::record_batch::RecordBatch]) -> Result<(), IdsError>
) -> Result<(), IdsError> {
    // Create a reader with large buffer
    // Process batches in chunks
    // Call process_fn on each chunk
    
    Ok(())
}
```

## Windows Server Optimization

Special considerations for Windows server environments:

1. **Memory Mapping**: Use memory-mapped files for better performance on Windows
2. **I/O Buffering**: Increase buffer sizes for Windows I/O operations
3. **Sequential Scan Hints**: Add flags to optimize for sequential file access
4. **Reduced File Locking**: Minimize file locking to prevent contention

## Usage Instructions

### Environment Variable Configuration

```bash
# Thread configuration
export IDS_MAX_THREADS=64      # Number of worker threads (no limit)
export RAYON_NUM_THREADS=64    # Global thread pool size

# Memory and batch configuration
export IDS_BATCH_SIZE=131072   # 128K rows per batch (adjust based on memory)
```

### Windows Command Example

```batch
:: Set environment variables for optimized family-based loading
SET IDS_MAX_THREADS=64
SET IDS_BATCH_SIZE=131072
SET RAYON_NUM_THREADS=64

:: Run with optimized loading using CLI flags
ids.exe check-balance --matches-file matches.csv --covariate-dir C:\path\to\registers --optimized-loading --family-based-filtering
```

## Implementation Strategy

1. Start with the Phase 1 quick wins (batch size and thread count)
2. Add family-based filtering for efficient memory usage 
3. Test thoroughly with realistic data volumes
4. Implement Phase 2 optimizations if needed

## Expected Benefits

- **Reduced Loading Time**: Faster loading through parallelism and efficient I/O
- **Lower Memory Usage**: Family-based filtering reduces memory requirements by only loading relevant records
- **Better Server Utilization**: Full use of available CPU cores and memory
- **Windows Optimization**: Special optimizations for Windows server environments

This approach delivers significant performance improvements while maintaining compatibility with the existing codebase.