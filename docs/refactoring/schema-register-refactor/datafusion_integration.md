# DataFusion Integration for Registry and Schema Modules

After examining Apache DataFusion's examples and understanding our refactoring plan, I've identified several ways to leverage DataFusion to make our codebase more lean, maintainable, and potentially better performing.

## Benefits of DataFusion

1. **Query Optimization**: DataFusion includes a sophisticated query optimizer that can significantly improve performance.
2. **Predicate Pushdown**: Filter operations are pushed down to the data source level, improving performance.
3. **Statistics-Based Pruning**: DataFusion can use statistics to skip reading parquet files that don't match query filters.
4. **Memory Efficiency**: More efficient memory usage compared to loading full datasets into memory.
5. **Parallel and Async Execution**: Built-in support for parallel and asynchronous query execution.
6. **SQL Support**: DataFusion provides native SQL support for querying data.
7. **Custom Data Sources**: Flexible APIs for creating custom data sources.
8. **Query Planning**: Logical and physical query planning with optimization.

## Integration Strategy

### 1. Replace Direct Parquet Operations with DataFusion

Instead of directly using Parquet readers, we can use DataFusion's SessionContext to create and execute more efficient queries:

```rust
// Current approach
pub fn read_parquet(
    path: &Path,
    schema: Option<&Schema>,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>> {
    // Direct parquet file reading...
}

// DataFusion approach
pub async fn read_parquet(
    path: &Path, 
    schema: Option<&Schema>,
    pnr_filter: Option<&HashSet<String>>
) -> Result<Vec<RecordBatch>> {
    let ctx = SessionContext::new();
    
    // Register schema if provided
    let read_options = if let Some(schema) = schema {
        ParquetReadOptions::default().with_schema(schema.clone())
    } else {
        ParquetReadOptions::default()
    };
    
    // Create DataFrame
    let mut df = ctx.read_parquet(
        path.to_string_lossy().to_string(), 
        read_options
    ).await?;
    
    // Apply PNR filter if provided
    if let Some(pnr_filter) = pnr_filter {
        // Convert HashSet to a list for IN expression
        let pnr_list: Vec<Expr> = pnr_filter
            .iter()
            .map(|pnr| lit(pnr.clone()))
            .collect();
        
        // Create filter: PNR IN (pnr1, pnr2, ...)
        df = df.filter(col("PNR").in_list(pnr_list))?;
    }
    
    // Collect and return results
    let result = df.collect().await?;
    Ok(result)
}
```

### 2. Implement Registry Loaders with DataFusion

Reimagine registry loaders to use DataFusion's capabilities:

```rust
// src/data/registry/traits.rs
pub trait RegisterLoader {
    type SchemaType: RegistrySchema;
    
    fn register_name() -> &'static str;
    
    // Async function for loading data
    async fn load(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>
    ) -> Result<Vec<RecordBatch>>;
    
    // Create a DataFusion SessionContext with this registry
    async fn create_context(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>
    ) -> Result<SessionContext> {
        let ctx = SessionContext::new();
        let schema = Self::SchemaType::schema_arc();
        
        // Register source as a table
        ctx.register_parquet(
            Self::register_name().to_lowercase(),
            base_path,
            ParquetReadOptions::default().with_schema(schema)
        ).await?;
        
        // Apply PNR filter if provided
        if let Some(filter) = pnr_filter {
            // Apply filter logic to context...
        }
        
        Ok(ctx)
    }
    
    // Default implementation of load using DataFusion
    async fn default_load(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>
    ) -> Result<Vec<RecordBatch>> {
        let ctx = self.create_context(base_path, pnr_filter).await?;
        let table_name = Self::register_name().to_lowercase();
        
        let df = ctx.table(&table_name).await?;
        let result = df.collect().await?;
        Ok(result)
    }
}

// Implementation example for AKM
impl RegisterLoader for AkmRegister {
    type SchemaType = AkmSchema;
    
    fn register_name() -> &'static str {
        "AKM"
    }
    
    async fn load(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>
    ) -> Result<Vec<RecordBatch>> {
        self.default_load(base_path, pnr_filter).await
    }
}
```

### 3. Pruning for Efficient File Filtering

Use DataFusion's pruning capabilities to avoid reading files that don't match filters:

```rust
// src/data/io/pruning.rs
pub struct RegistryPruningStatistics {
    schema: SchemaRef,
    min_values: HashMap<String, ArrayRef>,
    max_values: HashMap<String, ArrayRef>,
    row_counts: Option<ArrayRef>,
}

impl PruningStatistics for RegistryPruningStatistics {
    fn min_values(&self, column: &Column) -> Option<ArrayRef> {
        self.min_values.get(column.name.as_str()).cloned()
    }
    
    fn max_values(&self, column: &Column) -> Option<ArrayRef> {
        self.max_values.get(column.name.as_str()).cloned()
    }
    
    fn row_counts(&self, _column: &Column) -> Option<ArrayRef> {
        self.row_counts.clone()
    }
    
    fn null_counts(&self, _column: &Column) -> Option<ArrayRef> {
        None
    }
    
    fn num_containers(&self) -> usize {
        // Return number of files in the statistics
        self.min_values
            .values()
            .next()
            .map_or(0, |arr| arr.len())
    }
    
    fn contained(
        &self,
        _column: &Column,
        _values: &HashSet<ScalarValue>,
    ) -> Option<BooleanArray> {
        None
    }
}

// Function to create pruning predicate for PNR filter
pub fn create_pnr_pruning_predicate(
    pnrs: &HashSet<String>,
    schema: SchemaRef
) -> Result<PruningPredicate> {
    // Create IN expression for PNR
    let pnr_values: Vec<Expr> = pnrs
        .iter()
        .map(|pnr| lit(pnr.clone()))
        .collect();
    
    let expr = col("PNR").in_list(pnr_values);
    
    // Create pruning predicate
    let df_schema = DFSchema::try_from(schema.as_ref().clone())?;
    let props = ExecutionProps::new();
    let physical_expr = create_physical_expr(&expr, &df_schema, &props)?;
    
    Ok(PruningPredicate::try_new(physical_expr, schema)?)
}
```

### 4. Unified Transform Pipeline with DataFusion

Replace the current transformation functions with DataFusion's DataFrame operations:

```rust
// src/data/transform/mod.rs
pub struct TransformPipeline {
    operations: Vec<Box<dyn Fn(DataFrame) -> Result<DataFrame>>>,
}

impl TransformPipeline {
    pub fn new() -> Self {
        Self { operations: Vec::new() }
    }
    
    pub fn add_filter(mut self, expr: Expr) -> Self {
        self.operations.push(Box::new(move |df| df.filter(expr.clone())));
        self
    }
    
    pub fn add_select(mut self, columns: Vec<&str>) -> Self {
        self.operations.push(Box::new(move |df| {
            df.select_columns(&columns)
        }));
        self
    }
    
    pub fn add_aggregate(
        mut self,
        group_by: Vec<Expr>,
        aggregates: Vec<Expr>
    ) -> Self {
        self.operations.push(Box::new(move |df| {
            df.aggregate(group_by.clone(), aggregates.clone())
        }));
        self
    }
    
    pub async fn apply(&self, ctx: &SessionContext, table_name: &str) -> Result<DataFrame> {
        let mut df = ctx.table(table_name).await?;
        
        for op in &self.operations {
            df = op(df)?;
        }
        
        Ok(df)
    }
}

// Example usage
// Transform that filters by date range and adds a year column
pub fn transform_date_range_pipeline(
    start_date: Date,
    end_date: Date
) -> TransformPipeline {
    TransformPipeline::new()
        .add_filter(col("DATE").gt_eq(lit(start_date)).and(col("DATE").lt_eq(lit(end_date))))
        .add_select(vec!["PNR", "DATE", "VALUE"])
}
```

### 5. Custom Data Sources for Complex Registries

For complex registries like LPR, implement custom TableProvider to handle their specifics:

```rust
// src/data/registry/loaders/lpr/lpr_provider.rs
pub struct LprTableProvider {
    version: LprVersion,
    paths: LprPaths,
    schema: SchemaRef,
}

#[async_trait]
impl TableProvider for LprTableProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
    
    fn table_type(&self) -> TableType {
        TableType::Base
    }
    
    async fn scan(
        &self,
        state: &dyn Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        // Custom logic to handle LPR's multiple files and directories
        // using the version and paths information
        
        // Example: for LPR v2 Admin files
        match self.version {
            LprVersion::V2 => {
                // Implementation for LPR v2
                let object_store_url = ObjectStoreUrl::parse("file://")?;
                let source = Arc::new(ParquetSource::default());
                let mut file_scan_config_builder = FileScanConfigBuilder::new(
                    object_store_url, 
                    self.schema.clone(), 
                    source
                )
                .with_projection(projection.cloned())
                .with_limit(limit);
                
                // Add all the files from the paths
                for file_path in self.find_lpr_files()? {
                    let canonical_path = fs::canonicalize(file_path)?;
                    file_scan_config_builder = file_scan_config_builder.with_file(
                        PartitionedFile::new(canonical_path.display().to_string(), 0)
                    );
                }
                
                Ok(DataSourceExec::from_data_source(file_scan_config_builder.build()))
            },
            LprVersion::V3 => {
                // Implementation for LPR v3
                // Similar to V2 but with different file handling
                // ...
            }
        }
    }
}
```

### 6. Async Loading with DataFusion

Replace the current async loading with DataFusion's async support:

```rust
// src/data/io/async.rs
pub async fn load_parquet_files_async(
    path: &Path,
    schema: Option<&SchemaRef>,
    batch_size: usize,
) -> Result<Vec<RecordBatch>> {
    let ctx = SessionContext::new();
    
    // Configure batch size
    ctx.runtime_env().with_batch_size(batch_size);
    
    // Create read options
    let read_options = match schema {
        Some(schema) => ParquetReadOptions::default().with_schema(schema.clone()),
        None => ParquetReadOptions::default(),
    };
    
    // Read parquet
    let df = ctx.read_parquet(path.to_string_lossy().to_string(), read_options).await?;
    
    // Collect and return
    Ok(df.collect().await?)
}
```

### 7. SQL Interface for Queries

Add SQL capabilities for querying the registries:

```rust
// src/data/query/sql.rs
pub struct RegistrySqlEngine {
    ctx: SessionContext,
}

impl RegistrySqlEngine {
    pub fn new() -> Self {
        Self { ctx: SessionContext::new() }
    }
    
    pub async fn register_registry<R: RegisterLoader>(
        &mut self,
        loader: &R,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<()> {
        // Register the registry as a table
        let ctx = loader.create_context(base_path, pnr_filter).await?;
        let table_name = R::register_name().to_lowercase();
        let table = ctx.table(&table_name).await?;
        
        // Register the table in our context
        self.ctx.register_table(&table_name, table.into_optimized_plan()?)?;
        
        Ok(())
    }
    
    pub async fn execute_sql(&self, query: &str) -> Result<Vec<RecordBatch>> {
        let df = self.ctx.sql(query).await?;
        Ok(df.collect().await?)
    }
}

// Example usage
pub async fn example_sql_query() -> Result<()> {
    let mut engine = RegistrySqlEngine::new();
    
    // Register registries
    engine.register_registry(
        &AkmRegister, 
        "/path/to/akm", 
        None
    ).await?;
    
    engine.register_registry(
        &BefRegister, 
        "/path/to/bef", 
        None
    ).await?;
    
    // Execute SQL query
    let results = engine.execute_sql(
        "SELECT a.PNR, a.SOCIO, b.GENDER 
         FROM akm a JOIN bef b ON a.PNR = b.PNR 
         WHERE a.SOCIO > 5"
    ).await?;
    
    Ok(())
}
```

## Recommended Changes to Cargo.toml

Add DataFusion as a dependency:

```toml
[dependencies]
# Existing dependencies...

# DataFusion
datafusion = { version = "33.0.0", features = ["simd", "crypto_expressions"] }
```

## Migration Strategy

1. **Phase 1**: Add DataFusion dependency and create basic abstractions
   - Add DataFusion to Cargo.toml
   - Create base traits with DataFusion integration
   - Implement utility functions for DataFusion integration

2. **Phase 2**: Implement DataFusion-based I/O layer
   - Create unified ParquetReader using DataFusion
   - Implement pruning-based filtering
   - Create parallel and async loading utilities

3. **Phase 3**: Refactor registry loaders
   - Update RegisterLoader trait with DataFusion capabilities
   - Implement default methods using DataFusion
   - Create custom TableProvider for complex registries

4. **Phase 4**: Implement transform pipeline
   - Create DataFusion-based transform pipeline
   - Implement common transformations as pipeline steps
   - Add customization capabilities

5. **Phase 5**: Add SQL interface
   - Implement RegistrySqlEngine
   - Add registry registration with SQL support
   - Create convenience methods for common operations

6. **Phase 6**: Update client code and documentation
   - Adapt client code to use new APIs
   - Create comprehensive documentation
   - Add examples showing advanced capabilities

## Conclusion

Integrating DataFusion into the registry and schema modules would bring significant benefits in terms of code maintainability, performance, and functionality. The most compelling advantages include:

1. **Code Reduction**: Leveraging DataFusion's built-in capabilities will reduce custom code by an estimated 50-60%.
2. **Performance**: Predicate pushdown, statistics-based pruning, and query optimization can significantly improve performance.
3. **SQL Support**: Adding SQL query capabilities provides a powerful interface for data exploration.
4. **Maintainability**: Using a well-supported framework reduces the need for custom implementations.
5. **Extensibility**: DataFusion's modular design makes adding new features easier.

This integration aligns perfectly with the refactoring plan's goals of creating a more intuitive, logical, and efficient structure for the registry and schema modules.