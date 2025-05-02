# Proposed Restructuring of Registry and Schema Modules

After analyzing the current structure of the `src/registry` and `src/schema` modules, I've identified several opportunities for improvement. This document outlines a comprehensive proposal for restructuring these modules to be more intuitive, logical, and efficient.

## Current Issues

1. **Separation of Related Concerns**: Schema definitions and registry loaders are separated, despite their tight coupling.
2. **Code Duplication**: Similar patterns are repeated across registry loaders.
3. **Inconsistent PNR Filtering**: Different registries handle PNR filtering differently.
4. **Limited Abstraction**: Limited use of traits and generics to reduce code duplication.
5. **Complex LPR Implementation**: LPR has special handling scattered across different files.
6. **Verbose Error Handling**: Error handling patterns are repeated across files.
7. **Transformation Integration**: Transformations exist but aren't well integrated with loading.

## Proposed Structure

I propose a restructuring focused on consolidating related functionality, reducing duplication, and providing clearer abstractions:

```
src/
  data/                           # New top-level module for data concerns
    schema/                       # Schema definitions
      traits.rs                   # Schema trait definitions
      registry/                   # Registry schemas grouped by registry
        akm.rs
        bef.rs
        ...
        lpr/                      # Grouped LPR schemas
          mod.rs
          lpr2.rs                 # LPR version 2 schemas
          lpr3.rs                 # LPR version 3 schemas
      common/                     # Common schema utilities
        mod.rs
        field_types.rs            # Standard field type definitions
        metadata.rs               # Field metadata helpers
    registry/                     # Registry loaders
      traits.rs                   # Registry loader trait definitions
      loaders/                    # Registry loader implementations
        akm.rs
        bef.rs
        ...
        lpr/                      # Grouped LPR loaders
          mod.rs
          lpr2.rs                 # LPR version 2 loaders
          lpr3.rs                 # LPR version 3 loaders
      factory.rs                  # Registry loader factory
    transform/                    # Data transformations
      mod.rs
      filters.rs                  # Filter transformations
      aggregations.rs             # Aggregation transformations
      conversions.rs              # Data type conversions
      joins.rs                    # Record joining utilities
    io/                           # I/O utilities
      mod.rs
      parquet.rs                  # Unified parquet operations
      parallel.rs                 # Parallel loading utilities
      async.rs                    # Async loading utilities
      filtering.rs                # Filter expression implementation
```

## Key Design Principles

### 1. Registry and Schema Association

Create a clear association between registry loaders and schemas by organizing them with parallel structure. Each registry and its schema would follow the same organizational pattern.

### 2. Trait-Based Architecture

Introduce traits to standardize and abstract common patterns:

```rust
// src/data/schema/traits.rs
pub trait RegistrySchema {
    fn schema() -> Schema;
    fn schema_arc() -> Arc<Schema>;
    fn schema_with_metadata() -> Schema;  // Include additional metadata
    fn column_names() -> Vec<&'static str>;  // For easier column reference
}

// src/data/registry/traits.rs
pub trait RegisterLoader {
    type SchemaType: RegistrySchema;  // Associate with schema
    
    fn register_name() -> &'static str;
    fn load(&self, base_path: &str, pnr_filter: Option<&PnrFilter>) -> Result<Vec<RecordBatch>>;
    
    // Default implementations for common operations
    fn get_schema() -> Arc<Schema> {
        Self::SchemaType::schema_arc()
    }
}
```

### 3. Registry Factory Pattern

Improve the registry factory with a more extensible approach:

```rust
// src/data/registry/factory.rs
pub struct RegistryFactory;

impl RegistryFactory {
    pub fn from_name(name: &str) -> Result<Box<dyn RegisterLoader>> {
        // Factory implementation
    }
    
    pub fn from_path(path: &Path) -> Result<Box<dyn RegisterLoader>> {
        // Path-based factory implementation
    }
    
    pub fn create_all() -> HashMap<&'static str, Box<dyn RegisterLoader>> {
        // Create instances of all available loaders
    }
}
```

### 4. Unified PNR Filtering

Create a consistent approach to PNR filtering:

```rust
// src/data/io/filtering.rs
pub struct PnrFilter {
    pnrs: HashSet<String>,
    direct_filter: bool,
    relation_column: Option<String>,
}

impl PnrFilter {
    pub fn new(pnrs: HashSet<String>) -> Self { ... }
    
    pub fn with_relation(pnrs: HashSet<String>, relation_column: &str) -> Self { ... }
    
    pub fn apply(&self, batch: &RecordBatch) -> Result<RecordBatch> { ... }
}
```

### 5. Transformations Pipeline

Introduce a pipeline pattern for transformations:

```rust
// src/data/transform/mod.rs
pub struct TransformPipeline {
    transforms: Vec<Box<dyn Transform>>,
}

impl TransformPipeline {
    pub fn new() -> Self { ... }
    
    pub fn add<T: Transform + 'static>(mut self, transform: T) -> Self { ... }
    
    pub fn apply(&self, batches: Vec<RecordBatch>) -> Result<Vec<RecordBatch>> { ... }
}

pub trait Transform {
    fn transform(&self, batch: &RecordBatch) -> Result<RecordBatch>;
    fn name(&self) -> &str;
}
```

### 6. LPR Version Handling

Handle LPR versions more cleanly:

```rust
// src/data/registry/loaders/lpr/mod.rs
pub enum LprVersion {
    V2,
    V3,
}

pub trait LprRegistry: RegisterLoader {
    fn version(&self) -> LprVersion;
    fn find_files(&self, path: &Path) -> Result<Vec<PathBuf>>;
}
```

### 7. Improved I/O Layer

Consolidate parquet operations into a unified API:

```rust
// src/data/io/parquet.rs
pub struct ParquetReader {
    path: PathBuf,
    schema: Option<Arc<Schema>>,
    batch_size: usize,
    parallel: bool,
    async_loading: bool,
}

impl ParquetReader {
    pub fn new(path: impl AsRef<Path>) -> Self { ... }
    
    pub fn with_schema(mut self, schema: Arc<Schema>) -> Self { ... }
    
    pub fn parallel(mut self, parallel: bool) -> Self { ... }
    
    pub fn async_loading(mut self, async_loading: bool) -> Self { ... }
    
    pub fn read(&self) -> Result<Vec<RecordBatch>> { ... }
    
    pub fn read_with_filter(&self, filter: impl Into<FilterExpr>) -> Result<Vec<RecordBatch>> { ... }
}
```

## Implementation Strategy

I recommend implementing this restructuring in phases:

1. **Phase 1**: Create the new directory structure and base trait definitions
2. **Phase 2**: Migrate schema definitions to the new structure
3. **Phase 3**: Implement the new registry loader architecture
4. **Phase 4**: Refactor the transformation system
5. **Phase 5**: Consolidate the I/O utilities
6. **Phase 6**: Update client code to use the new APIs
7. **Phase 7**: Add comprehensive documentation and examples

## Benefits

This restructuring offers several benefits:

1. **Reduced Code Duplication**: Through traits and shared implementations
2. **Improved Organization**: Clear hierarchy and responsibility boundaries
3. **Better Extensibility**: Adding new registries becomes more straightforward
4. **Performance Improvements**: Consolidated I/O layer can optimize operations
5. **Enhanced Readability**: Clearer structure makes code navigation easier
6. **Better Type Safety**: More type-driven design via associated types
7. **Flexible Transformations**: Pipeline pattern enables composition
8. **Improved Error Handling**: Consolidated error handling approaches

## Migration Considerations

To ensure a smooth migration:

1. Maintain backward compatibility during the transition
2. Implement and test changes incrementally
3. Update documentation to reflect new structure
4. Create adapters for existing client code if needed
5. Add comprehensive tests for the new structure

## Conclusion

This restructuring would significantly improve the organization, efficiency, and maintainability of the registry and schema modules. By consolidating related functionality, reducing duplication, and providing clearer abstractions, the code will be easier to understand, extend, and maintain.