# DataFusion Migration Guide

This guide outlines a practical approach to incrementally migrate the registry and schema modules to use DataFusion.

## Migration Strategy Overview

The migration to DataFusion will follow these principles:

1. **Incremental Adoption**: Replace components one at a time without breaking existing functionality
2. **Backward Compatibility**: Maintain compatibility with existing client code during migration
3. **Feature Parity**: Ensure all existing features work before removing old implementations
4. **Performance Validation**: Validate performance improvements at each step

## Phase 1: Foundation Setup (3-5 days)

### Step 1: Add Dependencies

Update `Cargo.toml` to include DataFusion:

```toml
[dependencies]
# Existing dependencies...

# DataFusion
datafusion = { version = "33.0.0", features = ["simd", "crypto_expressions"] }
async-trait = "0.1"
```

### Step 2: Create Base Traits and Abstractions

Create the base traits and abstractions in the new directory structure:

```
src/data/
  schema/
    traits.rs       # Schema traits
  registry/
    traits.rs       # Registry traits
  io/
    mod.rs          # I/O utilities
    datafusion.rs   # DataFusion integration utilities
```

#### 1. Schema Traits (src/data/schema/traits.rs)

```rust
use arrow::datatypes::{Schema, SchemaRef};
use std::sync::Arc;

/// Trait for registry schemas with associated utilities
pub trait RegistrySchema {
    /// Get the schema for this registry
    fn schema() -> Schema;
    
    /// Get the schema as an Arc<Schema>
    fn schema_arc() -> SchemaRef {
        Arc::new(Self::schema())
    }
    
    /// Get the schema with additional metadata
    fn schema_with_metadata() -> Schema {
        Self::schema()
    }
    
    /// Get the column names for this schema
    fn column_names() -> Vec<&'static str>;
}
```

#### 2. Registry Traits (src/data/registry/traits.rs)

```rust
use crate::data::schema::traits::RegistrySchema;
use crate::data::io::filtering::PnrFilter;
use crate::error::Result;
use arrow::record_batch::RecordBatch;
use async_trait::async_trait;
use datafusion::prelude::*;
use std::sync::Arc;

/// Trait for registry loaders with DataFusion integration
#[async_trait]
pub trait RegisterLoader {
    /// Associated schema type
    type SchemaType: RegistrySchema;
    
    /// Get the name of the register
    fn register_name() -> &'static str;
    
    /// Load data from this registry (legacy synchronous method)
    fn load(&self, base_path: &str, pnr_filter: Option<&PnrFilter>) -> Result<Vec<RecordBatch>>;
    
    /// Load data from this registry asynchronously using DataFusion
    async fn load_async(
        &self, 
        base_path: &str, 
        pnr_filter: Option<&PnrFilter>
    ) -> Result<Vec<RecordBatch>>;
    
    /// Create a DataFusion session context with this registry loaded
    async fn create_context(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>
    ) -> Result<SessionContext>;
}
```

#### 3. DataFusion Utilities (src/data/io/datafusion.rs)

```rust
use crate::error::{IdsError, Result};
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::Path;
use std::collections::HashSet;

/// Create a DataFusion SessionContext
pub fn create_session_context() -> SessionContext {
    SessionContext::new()
}

/// Read parquet files using DataFusion
pub async fn read_parquet_datafusion(
    path: &Path,
    schema: Option<SchemaRef>,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>> {
    let ctx = create_session_context();
    
    // Create read options
    let read_options = if let Some(schema) = schema {
        ParquetReadOptions::default().with_schema(schema)
    } else {
        ParquetReadOptions::default()
    };
    
    // Read parquet
    let mut df = ctx.read_parquet(
        path.to_string_lossy().to_string(),
        read_options,
    ).await.map_err(|e| IdsError::Data(format!(
        "Failed to read parquet with DataFusion: {e}"
    )))?;
    
    // Apply PNR filter if provided
    if let Some(pnrs) = pnr_filter {
        let pnr_list: Vec<Expr> = pnrs
            .iter()
            .map(|pnr| lit(pnr.clone()))
            .collect();
        
        df = df.filter(col("PNR").in_list(pnr_list))
            .map_err(|e| IdsError::Data(format!(
                "Failed to apply PNR filter: {e}"
            )))?;
    }
    
    // Collect results
    let result = df.collect().await
        .map_err(|e| IdsError::Data(format!(
            "Failed to collect results: {e}"
        )))?;
        
    Ok(result)
}
```

### Step 3: Create PnrFilter Type and Utilities

Implement a standardized PNR filtering mechanism:

```rust
// src/data/io/filtering.rs
use std::collections::HashSet;

/// PNR filtering options
#[derive(Clone, Debug)]
pub enum PnrFilter {
    /// Direct PNR filtering (PNR column)
    Direct(HashSet<String>),
    
    /// Relational PNR filtering (other column)
    Relation {
        pnrs: HashSet<String>,
        column: String,
    },
}

impl PnrFilter {
    /// Create a new direct PNR filter
    pub fn new(pnrs: HashSet<String>) -> Self {
        Self::Direct(pnrs)
    }
    
    /// Create a new relational PNR filter
    pub fn with_relation(pnrs: HashSet<String>, column: impl Into<String>) -> Self {
        Self::Relation {
            pnrs,
            column: column.into(),
        }
    }
    
    /// Convert to DataFusion expression
    pub fn to_expr(&self) -> datafusion::logical_expr::Expr {
        use datafusion::prelude::*;
        
        match self {
            Self::Direct(pnrs) => {
                let pnr_list: Vec<Expr> = pnrs
                    .iter()
                    .map(|pnr| lit(pnr.clone()))
                    .collect();
                
                col("PNR").in_list(pnr_list)
            },
            Self::Relation { pnrs, column } => {
                let pnr_list: Vec<Expr> = pnrs
                    .iter()
                    .map(|pnr| lit(pnr.clone()))
                    .collect();
                
                col(column).in_list(pnr_list)
            }
        }
    }
}
```

## Phase 2: Implement DataFusion-Based Registry Loaders (3-5 days)

### Step 1: Implement a Single Registry with DataFusion

Start with a simple registry like AKM:

```rust
// src/data/schema/registry/akm.rs
use arrow::datatypes::{DataType, Field, Schema};
use crate::data::schema::traits::RegistrySchema;

pub struct AkmSchema;

impl RegistrySchema for AkmSchema {
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("SOCIO", DataType::Int8, true),
            Field::new("SOCIO02", DataType::Int8, true),
            Field::new("SOCIO13", DataType::Int8, true),
            Field::new("CPRTJEK", DataType::Utf8, true),
            Field::new("CPRTYPE", DataType::Utf8, true),
            Field::new("VERSION", DataType::Utf8, true),
            Field::new("SENR", DataType::Utf8, true),
        ])
    }
    
    fn column_names() -> Vec<&'static str> {
        vec!["PNR", "SOCIO", "SOCIO02", "SOCIO13", "CPRTJEK", "CPRTYPE", "VERSION", "SENR"]
    }
}

// src/data/registry/loaders/akm.rs
use crate::data::registry::traits::RegisterLoader;
use crate::data::schema::registry::akm::AkmSchema;
use crate::data::io::filtering::PnrFilter;
use crate::data::io::datafusion::read_parquet_datafusion;
use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use arrow::datatypes::SchemaRef;
use async_trait::async_trait;
use datafusion::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct AkmRegister;

#[async_trait]
impl RegisterLoader for AkmRegister {
    type SchemaType = AkmSchema;
    
    fn register_name() -> &'static str {
        "AKM"
    }
    
    /// Legacy synchronous load method - calls async method through runtime
    fn load(&self, base_path: &str, pnr_filter: Option<&PnrFilter>) -> Result<Vec<RecordBatch>> {
        // Create runtime for async operation
        let runtime = Runtime::new()
            .map_err(|e| IdsError::Runtime(format!("Failed to create Tokio runtime: {e}")))?;
            
        // Run async load in runtime
        runtime.block_on(self.load_async(base_path, pnr_filter))
    }
    
    /// Async load method using DataFusion
    async fn load_async(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>
    ) -> Result<Vec<RecordBatch>> {
        // Get schema
        let schema = Self::SchemaType::schema_arc();
        
        // Path handling
        let path = Path::new(base_path);
        if !path.exists() || !path.is_dir() {
            return Err(IdsError::Validation(format!(
                "AKM directory does not exist: {}", 
                path.display()
            )));
        }
        
        // Convert PnrFilter to HashSet if provided
        let pnr_set = if let Some(filter) = pnr_filter {
            match filter {
                PnrFilter::Direct(pnrs) => Some(pnrs),
                PnrFilter::Relation { .. } => {
                    return Err(IdsError::Validation(
                        "Relational PNR filtering not supported for AKM registry".to_string()
                    ));
                }
            }
        } else {
            None
        };
        
        // Use DataFusion to read parquet files
        read_parquet_datafusion(path, Some(schema), pnr_set.as_ref()).await
    }
    
    /// Create a DataFusion session context with this registry
    async fn create_context(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>
    ) -> Result<SessionContext> {
        let ctx = SessionContext::new();
        let schema = Self::SchemaType::schema_arc();
        
        // Find all parquet files
        let path = Path::new(base_path);
        if !path.exists() || !path.is_dir() {
            return Err(IdsError::Validation(format!(
                "AKM directory does not exist: {}", 
                path.display()
            )));
        }
        
        // Create read options
        let read_options = ParquetReadOptions::default()
            .with_schema(schema);
            
        // Register table
        ctx.register_parquet(
            Self::register_name().to_lowercase(),
            base_path,
            read_options,
        ).await.map_err(|e| IdsError::Data(format!(
            "Failed to register AKM table: {e}"
        )))?;
        
        // Apply PNR filter if provided
        if let Some(filter) = pnr_filter {
            let filter_expr = filter.to_expr();
            let table_name = Self::register_name().to_lowercase();
            
            // Get table
            let df = ctx.table(&table_name).await
                .map_err(|e| IdsError::Data(format!(
                    "Failed to get table {table_name}: {e}"
                )))?;
                
            // Apply filter and register as view
            let filtered_df = df.filter(filter_expr)
                .map_err(|e| IdsError::Data(format!(
                    "Failed to apply filter to {table_name}: {e}"
                )))?;
                
            // Register as filtered_<name>
            let view_name = format!("filtered_{table_name}");
            ctx.register_table(&view_name, filtered_df.into_view())
                .map_err(|e| IdsError::Data(format!(
                    "Failed to register filtered view: {e}"
                )))?;
        }
        
        Ok(ctx)
    }
}
```

### Step 2: Create Legacy Compatibility Layer

Create a compatibility layer to map between old and new interfaces:

```rust
// src/registry/compat.rs
use std::collections::HashSet;
use crate::error::Result;
use crate::registry::RegisterLoader as OldRegisterLoader;
use crate::data::io::filtering::PnrFilter;
use arrow::record_batch::RecordBatch;

/// Adapter to use new RegisterLoader implementations through the old interface
pub struct RegisterLoaderAdapter<T: crate::data::registry::traits::RegisterLoader> {
    inner: T,
}

impl<T: crate::data::registry::traits::RegisterLoader> RegisterLoaderAdapter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: crate::data::registry::traits::RegisterLoader> OldRegisterLoader for RegisterLoaderAdapter<T> {
    fn get_register_name(&self) -> &'static str {
        T::register_name()
    }
    
    fn load(
        &self, 
        base_path: &str,
        pnr_filter: Option<&HashSet<String>>
    ) -> Result<Vec<RecordBatch>> {
        // Convert old PNR filter to new format
        let new_filter = pnr_filter.map(|pnrs| PnrFilter::new(pnrs.clone()));
        
        // Call new implementation
        self.inner.load(base_path, new_filter.as_ref())
    }
}

/// Adapter to use old RegisterLoader implementations through the new interface
pub struct OldRegisterLoaderAdapter<T: OldRegisterLoader> {
    inner: T,
}

impl<T: OldRegisterLoader> OldRegisterLoaderAdapter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
```

### Step 3: Implement and Test the Factory Pattern

Create a registry factory that works with the new implementation:

```rust
// src/data/registry/factory.rs
use crate::data::registry::traits::RegisterLoader;
use crate::data::registry::loaders::akm::AkmRegister;
// Import other registry loaders...
use crate::error::{IdsError, Result};
use std::path::Path;

/// Factory for creating registry loaders
pub struct RegistryFactory;

impl RegistryFactory {
    /// Create a registry loader from its name
    pub fn from_name(name: &str) -> Result<Box<dyn RegisterLoader>> {
        match name.to_lowercase().as_str() {
            "akm" => Ok(Box::new(AkmRegister)),
            // Other registry loaders...
            _ => Err(IdsError::Validation(format!("Unknown registry: {name}"))),
        }
    }
    
    /// Create a registry loader from a path
    pub fn from_path(path: &str) -> Result<Box<dyn RegisterLoader>> {
        let path = Path::new(path);
        
        // Try to infer registry from directory name
        if let Some(dir_name) = path.file_name().and_then(|f| f.to_str()) {
            let lower_name = dir_name.to_lowercase();
            
            // Check for registry name patterns in the path
            if lower_name.contains("akm") {
                return Ok(Box::new(AkmRegister));
            }
            // Other registry patterns...
        }
        
        Err(IdsError::Validation(format!(
            "Could not determine registry type from path: {}",
            path.display()
        )))
    }
}
```

## Phase 3: Implement Transform Pipeline (2-3 days)

### Step 1: Create Transform Pipeline

```rust
// src/data/transform/mod.rs
use std::sync::Arc;
use datafusion::prelude::*;
use datafusion::error::Result as DFResult;
use crate::error::{IdsError, Result};

/// Transform operation for DataFrames
pub type TransformOp = Arc<dyn Fn(DataFrame) -> DFResult<DataFrame> + Send + Sync>;

/// Transform pipeline for chaining DataFrame operations
pub struct TransformPipeline {
    operations: Vec<TransformOp>,
}

impl TransformPipeline {
    /// Create a new empty pipeline
    pub fn new() -> Self {
        Self { operations: Vec::new() }
    }
    
    /// Add a filter operation to the pipeline
    pub fn add_filter(mut self, expr: Expr) -> Self {
        let op = Arc::new(move |df: DataFrame| df.filter(expr.clone()));
        self.operations.push(op);
        self
    }
    
    /// Add a column selection operation to the pipeline
    pub fn add_select(mut self, columns: Vec<&str>) -> Self {
        let columns = columns.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let op = Arc::new(move |df: DataFrame| df.select_columns(&columns));
        self.operations.push(op);
        self
    }
    
    /// Add a sort operation to the pipeline
    pub fn add_sort(mut self, expr: Vec<Expr>) -> Self {
        let op = Arc::new(move |df: DataFrame| df.sort(expr.clone()));
        self.operations.push(op);
        self
    }
    
    /// Add an aggregation operation to the pipeline
    pub fn add_aggregate(
        mut self,
        group_by: Vec<Expr>,
        aggregates: Vec<Expr>
    ) -> Self {
        let op = Arc::new(move |df: DataFrame| {
            df.aggregate(group_by.clone(), aggregates.clone())
        });
        self.operations.push(op);
        self
    }
    
    /// Add a custom operation to the pipeline
    pub fn add_custom<F>(mut self, f: F) -> Self
    where
        F: Fn(DataFrame) -> DFResult<DataFrame> + Send + Sync + 'static,
    {
        self.operations.push(Arc::new(f));
        self
    }
    
    /// Apply the pipeline to a DataFrame
    pub fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        let mut result = df;
        
        for op in &self.operations {
            result = op(result).map_err(|e| {
                IdsError::Data(format!("Transform operation failed: {e}"))
            })?;
        }
        
        Ok(result)
    }
    
    /// Apply the pipeline to a table in a SessionContext
    pub async fn apply_to_table(
        &self,
        ctx: &SessionContext,
        table_name: &str
    ) -> Result<DataFrame> {
        let df = ctx.table(table_name).await
            .map_err(|e| IdsError::Data(format!(
                "Failed to get table {table_name}: {e}"
            )))?;
            
        self.apply(df)
    }
}
```

### Step 2: Implement Common Transformations

```rust
// src/data/transform/common.rs
use datafusion::prelude::*;
use chrono::NaiveDate;
use crate::data::transform::TransformPipeline;

/// Create a transform pipeline for filtering by date range
pub fn date_range_pipeline(
    date_column: &str,
    start_date: NaiveDate,
    end_date: NaiveDate
) -> TransformPipeline {
    // Convert dates to literals
    let start_lit = lit(start_date.format("%Y-%m-%d").to_string());
    let end_lit = lit(end_date.format("%Y-%m-%d").to_string());
    
    // Create pipeline with date filter
    TransformPipeline::new()
        .add_filter(
            col(date_column).cast(DataType::Date32).gt_eq(start_lit.cast(DataType::Date32))
            .and(col(date_column).cast(DataType::Date32).lt_eq(end_lit.cast(DataType::Date32)))
        )
}

/// Create a transform pipeline for value mapping
pub fn categorical_mapping_pipeline(
    column: &str,
    mappings: Vec<(i32, i32)>
) -> TransformPipeline {
    // Create CASE expression for mapping
    let mut case_expr = when(col(column).eq(lit(mappings[0].0)), lit(mappings[0].1));
    
    for (from, to) in mappings.iter().skip(1) {
        case_expr = case_expr.when(col(column).eq(lit(*from)), lit(*to));
    }
    
    let case_expr = case_expr.otherwise(col(column)).alias(column);
    
    // Create pipeline with column mapping
    TransformPipeline::new()
        .add_custom(move |df| df.with_column(column, case_expr.clone()))
}
```

## Phase 4: Migrate Complex Registries (3-5 days)

### Step 1: Implement LPR with DataFusion

Create a specialized LPR implementation that handles its complexity:

```rust
// src/data/registry/loaders/lpr/mod.rs
pub enum LprVersion {
    V2,
    V3,
}

pub struct LprPaths {
    pub adm_path: Option<String>,
    pub diag_path: Option<String>,
    pub bes_path: Option<String>,
    pub kontakter_path: Option<String>,
    pub diagnoser_path: Option<String>,
}

// src/data/registry/loaders/lpr/lpr_provider.rs
// Implement LprTableProvider as shown in datafusion_examples.md
```

### Step 2: Create an Integration Test

Create a test that validates the DataFusion implementation:

```rust
// tests/datafusion_integration_test.rs
use ids_rs::data::registry::traits::RegisterLoader;
use ids_rs::data::registry::loaders::akm::AkmRegister;
use ids_rs::data::transform::TransformPipeline;
use std::collections::HashSet;
use std::path::Path;

#[tokio::test]
async fn test_akm_datafusion_integration() {
    // Create registry loader
    let loader = AkmRegister;
    
    // Test paths (adjust as needed)
    let test_path = Path::new("test_data/akm");
    if !test_path.exists() {
        println!("Test data not available, skipping test");
        return;
    }
    
    // Create DataFusion context
    let ctx = loader.create_context(test_path.to_str().unwrap(), None)
        .await.expect("Failed to create context");
        
    // Create a simple pipeline
    let pipeline = TransformPipeline::new()
        .add_select(vec!["PNR", "SOCIO"])
        .add_filter(datafusion::prelude::col("SOCIO").gt(datafusion::prelude::lit(5)))
        .add_aggregate(
            vec![datafusion::prelude::col("SOCIO")],
            vec![datafusion::prelude::count(datafusion::prelude::col("PNR"))]
        );
        
    // Apply pipeline
    let df = pipeline.apply_to_table(&ctx, "akm")
        .await.expect("Failed to apply pipeline");
        
    // Collect results
    let results = df.collect().await.expect("Failed to collect results");
    
    // Validate results
    assert!(!results.is_empty(), "No results returned");
    println!("Results: {:?}", results);
}
```

## Phase 5: Implement SQL Interface (2-3 days)

Implement the SQL query interface as shown in `datafusion_examples.md`.

## Phase 6: Migration of Existing Code (3-5 days)

### Step 1: Update Factory Functions

Update the existing factory functions to use the new implementations:

```rust
// src/registry/mod.rs

// Update registry_from_name to use new implementations
pub fn registry_from_name(name: &str) -> Result<Box<dyn RegisterLoader>> {
    use crate::registry::compat::RegisterLoaderAdapter;
    
    match name.to_lowercase().as_str() {
        "akm" => Ok(Box::new(RegisterLoaderAdapter::new(
            crate::data::registry::loaders::akm::AkmRegister
        ))),
        // Other registries...
        _ => Err(IdsError::Validation(format!("Unknown registry: {name}"))),
    }
}
```

### Step 2: Add SQL Support to CLI

Add SQL support to the CLI for advanced queries:

```rust
// src/commands/sql.rs
use crate::data::query::sql_engine::RegistrySqlEngine;
use crate::error::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct SqlCommand {
    /// SQL query to execute
    #[arg(short, long)]
    query: String,
    
    /// Registry paths to include (format: name:path)
    #[arg(short, long, value_delimiter = ',')]
    registries: Vec<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,
}

pub async fn handle_sql_command(cmd: SqlCommand) -> Result<()> {
    // Create SQL engine
    let engine = RegistrySqlEngine::new();
    
    // Register registries
    for registry_spec in cmd.registries {
        let parts: Vec<&str> = registry_spec.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(IdsError::Validation(format!(
                "Invalid registry spec: {registry_spec}. Expected format: name:path"
            )));
        }
        
        let name = parts[0];
        let path = parts[1];
        
        // Create registry loader
        let loader = crate::data::registry::factory::RegistryFactory::from_name(name)?;
        
        // Register with SQL engine
        engine.register_registry(loader.as_ref(), path, None, None).await?;
    }
    
    // Execute SQL query
    let results = engine.execute_sql(&cmd.query).await?;
    
    // Output results
    if let Some(output_path) = cmd.output {
        // Write to parquet file
        use arrow::array::RecordBatch;
        use parquet::arrow::ArrowWriter;
        use std::fs::File;
        
        let file = File::create(output_path)?;
        let mut writer = ArrowWriter::try_new(
            file,
            results[0].schema(),
            None,
        )?;
        
        for batch in &results {
            writer.write(batch)?;
        }
        
        writer.close()?;
    } else {
        // Print to console
        println!("{}", pretty_format_batches(&results)?);
    }
    
    Ok(())
}
```

## Phase 7: Performance Optimization and Testing (3-5 days)

### Step 1: Implement Benchmarks

Create benchmarks to compare performance:

```rust
// benches/datafusion_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion};
use ids_rs::data::registry::traits::RegisterLoader;
use ids_rs::data::registry::loaders::akm::AkmRegister;
use ids_rs::registry::akm::AkmRegister as OldAkmRegister;
use ids_rs::registry::RegisterLoader as OldRegisterLoader;
use std::collections::HashSet;
use std::path::Path;
use tokio::runtime::Runtime;

pub fn benchmark_akm_loading(c: &mut Criterion) {
    let test_path = Path::new("test_data/akm");
    if !test_path.exists() {
        return;
    }
    
    let path_str = test_path.to_str().unwrap();
    
    // Create a random set of PNRs for filtering
    let mut pnrs = HashSet::new();
    for i in 0..1000 {
        pnrs.insert(format!("{:010}", i));
    }
    
    // Old implementation benchmark
    c.bench_function("old_akm_load", |b| {
        let loader = OldAkmRegister;
        b.iter(|| {
            loader.load(path_str, Some(&pnrs)).unwrap()
        });
    });
    
    // New implementation benchmark
    c.bench_function("new_akm_load", |b| {
        let loader = AkmRegister;
        let runtime = Runtime::new().unwrap();
        
        b.iter(|| {
            runtime.block_on(async {
                let pnr_filter = ids_rs::data::io::filtering::PnrFilter::new(pnrs.clone());
                loader.load_async(path_str, Some(&pnr_filter)).await.unwrap()
            })
        });
    });
}

criterion_group!(benches, benchmark_akm_loading);
criterion_main!(benches);
```

### Step 2: Optimize Query Performance

Analyze and optimize performance for different query patterns:

```rust
// src/data/io/optimization.rs

// Functions to help optimize queries
pub fn optimize_query_for_large_pnr_set(
    ctx: &SessionContext,
    table: &str,
    pnrs: &HashSet<String>,
) -> Result<DataFrame> {
    // For very large PNR sets, use a temporary table instead of IN clause
    if pnrs.len() > 1000 {
        // Create temporary PNR table
        let schema = Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
        ]);
        
        // Create record batch with PNRs
        let pnr_array = StringArray::from_iter_values(pnrs.iter().cloned());
        let batch = RecordBatch::try_new(
            Arc::new(schema.clone()),
            vec![Arc::new(pnr_array)],
        )?;
        
        // Register temporary table
        ctx.register_batch("temp_pnrs", batch)?;
        
        // Join with main table
        let df = ctx.sql(&format!(
            "SELECT t.* FROM {table} t JOIN temp_pnrs p ON t.PNR = p.PNR"
        )).await?;
        
        Ok(df)
    } else {
        // For smaller sets, use IN clause
        let pnr_list: Vec<Expr> = pnrs
            .iter()
            .map(|pnr| lit(pnr.clone()))
            .collect();
        
        let df = ctx.table(table).await?
            .filter(col("PNR").in_list(pnr_list))?;
            
        Ok(df)
    }
}
```

## Migration Timeline

| Phase | Duration | Cumulative Time |
|-------|----------|----------------|
| 1. Foundation Setup | 3-5 days | 3-5 days |
| 2. Registry Loaders | 3-5 days | 6-10 days |
| 3. Transform Pipeline | 2-3 days | 8-13 days |
| 4. Complex Registries | 3-5 days | 11-18 days |
| 5. SQL Interface | 2-3 days | 13-21 days |
| 6. Code Migration | 3-5 days | 16-26 days |
| 7. Optimization & Testing | 3-5 days | 19-31 days |

## Backwards Compatibility

During migration, maintain backwards compatibility through:

1. Compatibility adapters between old and new interfaces
2. Dual implementations in the factory functions
3. Gradual migration of client code

## Migration Risks and Mitigation

| Risk | Mitigation |
|------|------------|
| Performance regression | Benchmark each component before and after migration |
| Breaking API changes | Use adapter pattern and maintain old interfaces during transition |
| Missing features | Create feature parity checklist and verify for each registry |
| Complexity handling | Start with simple registries and apply lessons to complex ones |
| Async compatibility | Provide sync wrappers where needed using Tokio runtime |

## Success Criteria

The migration will be considered successful when:

1. All registries are implemented using DataFusion
2. Performance benchmarks show improvement over the old implementation
3. All existing tests pass with the new implementation
4. All existing client code works with the new implementation
5. The SQL interface provides the expected functionality