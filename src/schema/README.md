# Parquet Filter Expression Engine

This module provides advanced functionality for filtering Parquet files using composable expressions with column projection.

## Key Features

- Expression-based filtering with composition (AND, OR, NOT)
- Support for string operations (equality, contains, starts with, ends with)
- Support for numeric comparisons (greater than, less than, equal to)
- Column projection for efficiency
- Parallel processing of multiple files
- Date/time field filtering

## Usage Examples

### Basic Filtering

```rust
use ids_rs::schema::{filter_expr::col, read_parquet_with_filter};
use std::path::Path;

// Create a filter expression
let expr = col("PNR").eq("0101010101");

// Read and filter a parquet file
let batches = read_parquet_with_filter(
    Path::new("/path/to/file.parquet"), 
    &expr, 
    None
)?;
```

### Composite Filters

```rust
use ids_rs::schema::{filter_expr::{col, Expr}, read_parquet_with_filter};

// Create multiple filter conditions
let age_filter = col("AGE").gt(40);
let region_filter = col("REGION").starts_with("101");

// Combine filters with AND
let filter = age_filter.and(region_filter);

// With column projection (only return specific columns)
let columns = Some(&["PNR", "AGE", "REGION", "NAME"][..]);

// Read and filter
let batches = read_parquet_with_filter(
    Path::new("/path/to/file.parquet"), 
    &filter, 
    columns
)?;
```

### Process Multiple Files in Parallel

```rust
use ids_rs::schema::{filter_expr::col, load_parquet_files_parallel_with_filter};

// Create a filter
let expr = col("DATE").gt(20200101);

// Process an entire directory of parquet files in parallel
let batches = load_parquet_files_parallel_with_filter(
    Path::new("/path/to/directory"), 
    &expr, 
    None
)?;
```

### Date Range Filter

```rust
use ids_rs::schema::{filter_expr::col, read_parquet_with_filter};

// Create date range filter
let after_start = col("DATE").gt(20200101);
let before_end = col("DATE").lt(20201231);
let date_range = after_start.and(before_end);

// Read and filter
let batches = read_parquet_with_filter(
    Path::new("/path/to/file.parquet"), 
    &date_range, 
    None
)?;
```

## Available Operations

### String Operations

- `.eq(value)` - Equal to
- `.neq(value)` - Not equal to
- `.contains(value)` - Contains substring
- `.starts_with(value)` - Starts with prefix
- `.ends_with(value)` - Ends with suffix

### Numeric Operations

- `.eq(value)` - Equal to
- `.neq(value)` - Not equal to
- `.gt(value)` - Greater than
- `.gte(value)` - Greater than or equal to
- `.lt(value)` - Less than
- `.lte(value)` - Less than or equal to

### Logical Operations

- `expr1.and(expr2)` - Logical AND
- `expr1.or(expr2)` - Logical OR
- `expr.not()` - Logical NOT