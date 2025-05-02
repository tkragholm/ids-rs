# DataFusion Performance Benefits

This document outlines the expected performance improvements when integrating DataFusion into the registry and schema modules.

## Performance Comparison

The following table summarizes expected performance improvements for key operations:

| Operation | Current Implementation | DataFusion Implementation | Expected Improvement |
|-----------|------------------------|--------------------------|---------------------|
| Parquet File Loading | Direct file reading with manual projection and filtering | Optimized reading with predicate pushdown | 2-5x faster |
| PNR Filtering | Post-loading filtering in memory | Predicate pushdown to data source | 3-10x faster |
| Multiple File Processing | Parallel loading with Rayon | DataFusion parallel execution with optimized reading | 2-4x faster |
| Complex Queries | Manual implementation with multiple operations | SQL execution with optimized query plan | 5-15x faster |
| Memory Usage | Complete data loaded into memory | Streamed execution with memory management | 50-80% less memory |
| LPR Complex Operations | Custom code for file discovery and joining | Unified TableProvider with statistics-based pruning | 3-8x faster |

## Key Performance Optimizations

### 1. Predicate Pushdown

**Current Implementation:**
```rust
// 1. Read entire files into memory
let batches = read_parquet(path, schema, None)?;

// 2. Filter in memory after loading
let filtered_batches = batches.into_iter()
    .map(|batch| filter_batch_by_pnr(&batch, pnr_filter))
    .filter(|batch| batch.num_rows() > 0)
    .collect();
```

**DataFusion Implementation:**
```rust
// Pushes filter to parquet reader, only loading matching rows
let df = ctx.read_parquet(path, read_options)
    .await?
    .filter(col("PNR").in_list(pnr_list))?
    .collect()
    .await?;
```

**Benefits:**
- Reduces I/O by only reading relevant data from disk
- Reduces memory usage by not loading unnecessary data
- Leverages Parquet's internal optimizations for filtering
- Can use row group level statistics to skip entire sections of files

### 2. Statistics-Based Pruning

**Current Implementation:**
```rust
// Must read all files in a directory
for entry in dir.read_dir()? {
    let path = entry?.path();
    if path.extension() == Some("parquet".as_ref()) {
        let batches = read_parquet(&path, schema, pnr_filter)?;
        all_batches.extend(batches);
    }
}
```

**DataFusion Implementation:**
```rust
// Use statistics to skip files that can't match the predicate
let file_meta = RegistryFileMetadata::new(schema);
// Populate metadata from parquet files...

// Use predicate to determine which files to read
let matching_files = file_meta.get_files_for_predicate(&predicate)?;

// Only read files that might contain matching data
for file in matching_files {
    let df = ctx.read_parquet(file, read_options).await?;
    // Process file...
}
```

**Benefits:**
- Can completely skip files that don't match query predicates
- Uses min/max statistics from Parquet file metadata
- No need to open or read files that won't contribute to results
- Scales well with large numbers of files

### 3. Parallel Query Execution

**Current Implementation:**
```rust
// Uses Rayon for parallel file reading only
let batches: Vec<Vec<RecordBatch>> = files.par_iter()
    .map(|file| read_parquet(file, schema, pnr_filter))
    .collect::<Result<_>>()?;
```

**DataFusion Implementation:**
```rust
// DataFusion optimizes the entire query execution
let df = ctx.read_parquet(path, read_options)
    .await?
    .filter(col("PNR").in_list(pnr_list))?
    .select(vec![col("PNR"), col("SOCIO")])?
    .aggregate(vec![col("SOCIO")], vec![count(col("PNR"))])?
    .sort(vec![col("SOCIO").sort(true, true)])?
    .collect()
    .await?;
```

**Benefits:**
- Parallel execution of the entire query plan, not just file loading
- Optimized parallel aggregations and joins
- Better utilization of multiple CPU cores
- Adaptive to available system resources

### 4. Memory Efficiency

**Current Implementation:**
```rust
// All data must be loaded into memory
let all_batches = load_parquet_files_parallel(path, schema, pnr_filter)?;

// Process all batches at once
let result = process_batches(all_batches)?;
```

**DataFusion Implementation:**
```rust
// Streaming execution with controlled memory usage
let df = ctx.read_parquet(path, read_options)
    .await?
    .filter(col("PNR").in_list(pnr_list))?;
    
// Execute using streaming
let batches = df.execute_stream().await?;
while let Some(batch) = batches.next().await {
    // Process one batch at a time
    process_batch(batch?)?;
}
```

**Benefits:**
- Controlled memory usage with streaming execution
- No need to load entire dataset into memory
- Better scalability for large datasets
- Reduced risk of out-of-memory errors

### 5. Query Optimization

**Current Implementation:**
```rust
// Manual implementation of each operation in sequence
let filtered = transform::filter_by_date_range(batches, start_date, end_date)?;
let with_year = transform::add_year_column(filtered)?;
let aggregated = transform::aggregate_by_column(with_year, "YEAR")?;
```

**DataFusion Implementation:**
```rust
// Optimized query plan with logical optimizations
let df = ctx.read_parquet(path, read_options)
    .await?
    .filter(col("DATE").between(lit(start_date), lit(end_date)))?
    // DateFusion can optimize operations like extraction
    .with_column("YEAR", col("DATE").cast(DataType::Int32))?
    .aggregate(vec![col("YEAR")], vec![count(lit(1))])?
    .collect()
    .await?;
```

**Benefits:**
- Automatic query optimization by DataFusion's optimizer
- Common subexpression elimination
- Constant folding
- Filter pushdown and reordering
- Join reordering for optimal execution

### 6. Partial Projection

**Current Implementation:**
```rust
// Manual projection specification that must be passed correctly
let projection = ProjectionMask::leaves(
    reader_builder.parquet_schema(), 
    projection_indices
);

let reader = reader_builder
    .with_projection(projection)
    .build()?;
```

**DataFusion Implementation:**
```rust
// Simple column selection that DataFusion optimizes
let df = ctx.read_parquet(path, read_options)
    .await?
    .select_columns(&["PNR", "DATE", "VALUE"])?
    .collect()
    .await?;
```

**Benefits:**
- Simplified column selection
- Automatic projection optimization
- Pushes projection to Parquet readers
- Reduces I/O by only reading required columns

## Memory Usage Comparison

The following chart illustrates expected memory usage for different dataset sizes:

| Dataset Size | Current Implementation | DataFusion Implementation | Memory Reduction |
|--------------|------------------------|--------------------------|------------------|
| 1GB          | ~1.5-2GB               | ~300-500MB               | 70-80%           |
| 10GB         | ~15-20GB               | ~1-3GB                   | 85-95%           |
| 100GB        | Out of memory          | ~3-8GB                   | Enables processing |

## Runtime Comparison for Common Operations

| Operation                   | Current Implementation | DataFusion Implementation | Speedup |
|-----------------------------|------------------------|--------------------------|---------|
| Loading 10 parquet files    | 100%                   | 40-60%                   | 1.7-2.5x |
| Filtering by PNR (1% match) | 100%                   | 10-30%                   | 3.3-10x |
| Joining two large registries | 100%                  | 15-40%                   | 2.5-6.7x |
| Date range + aggregation    | 100%                   | 20-40%                   | 2.5-5x  |
| Complex multi-step analysis | 100%                   | 5-20%                    | 5-20x   |

## When to Expect Best Performance Gains

The most significant performance improvements can be expected in the following scenarios:

1. **Selective Filtering**: When queries filter out a large percentage of the data
2. **Working with Large Datasets**: When datasets are larger than available memory
3. **Complex Joins**: When performing joins between multiple registries
4. **Aggregate Queries**: When performing grouping and aggregation operations
5. **Complex Multi-Step Analysis**: When chaining multiple operations together

## Disk I/O Reduction

DataFusion can significantly reduce disk I/O through:

1. **Row Group Pruning**: Using Parquet row group statistics to skip entire sections of files
2. **File-Level Pruning**: Using file-level statistics to skip entire files
3. **Partial Column Reading**: Only reading required columns from disk
4. **Optimized Predicate Pushdown**: Pushing filters all the way to storage

For a typical selective query that reads 5% of rows and 30% of columns, DataFusion could reduce I/O by up to 95% compared to the current implementation.

## Conclusion

Integrating DataFusion into the registry and schema modules is expected to provide substantial performance improvements across all operations. The most significant gains will come from:

1. Predicate pushdown and statistics-based pruning
2. Memory-efficient streaming execution
3. Parallel query execution
4. Query optimization
5. Reduced disk I/O

These improvements will enhance the scalability and responsiveness of the system, enabling it to handle larger datasets with fewer resources. The largest performance gains will be seen in complex analytical queries over large datasets with selective filtering conditions.