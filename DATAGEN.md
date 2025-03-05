# Rust Crate Refactoring: datagen

After analyzing the code structure, I've identified several areas for improvement to make this codebase more idiomatic and efficient. Here's my restructuring plan:

## Overall Structure Improvements

1. Reorganize the crate to better separate concerns
2. Improve error handling with more specific error types
3. Improve API ergonomics with better type usage
4. Make better use of generics and traits
5. Optimize performance-critical code

## File Structure Changes

### config.rs

- Keep the file but add `Default` implementation for `GeneratorConfig`
- Change the validation approach to return a proper error type instead of `String`
- Consider using a builder pattern more consistently

### error.rs

- Create a dedicated error module with a proper enum-based error type
- Replace `IntoDataGenError` trait with more idiomatic implementation
- Add proper `Error` and `Display` implementations
- Remove the reexports and use more direct error handling

### generators/mod.rs

- Split the functionality more logically
- Extract common code patterns into helper modules
- Add a trait for generators to standardize the interface

### writers

- Create a dedicated module for different output formats
- Refactor `ParquetWriter` to be more flexible
- Add common traits for different writer implementations

### Specific Implementation Changes

## Refactoring Plan

Here's my concrete refactoring plan organized by files:

### src/error.rs

```rust
// Create a proper error type hierarchy
pub enum DataGenError {
    Configuration(String),
    IoError(std::io::Error),
    ArrowError(arrow::error::ArrowError),
    ParquetError(parquet::errors::ParquetError),
    Generation(String),
}

// Implement standard error traits
impl_error_conversions!(DataGenError);
```

### src/config.rs

```rust
// Add a builder pattern
impl GeneratorConfig {
    pub fn builder() -> GeneratorConfigBuilder { ... }
}

// Add a proper builder
pub struct GeneratorConfigBuilder { ... }

// Add Default implementation
impl Default for GeneratorConfig { ... }

// Use proper error type for validation
pub fn validate(&self) -> Result<(), DataGenError> { ... }
```

### src/generators/mod.rs

```rust
// Add a trait for generators
pub trait Generator {
    fn generate(&mut self, config: &GeneratorConfig) -> Result<(), DataGenError>;
}

// Make specific generators implement this trait
pub struct AkmGenerator { ... }
impl Generator for AkmGenerator { ... }

// Create a more modular approach to generation
pub struct CompositeGenerator {
    generators: Vec<Box<dyn Generator>>,
}
```

### src/writer.rs

```rust
// Create a trait for writers
pub trait DataWriter {
    fn write_batch(&mut self, batch: RecordBatch, path: &Path) -> Result<(), DataGenError>;
}

// Make the ParquetWriter implement this trait
impl DataWriter for ParquetWriter { ... }

// Add a builder for ParquetWriter with configuration options
pub struct ParquetWriterBuilder { ... }
```

### src/generators/common.rs (New file)

```rust
// Extract common functionality used across generators
pub fn create_date_array(dates: &[Option<NaiveDate>]) -> Date32Array { ... }

// Add helper functions for data generation
pub fn create_schema(fields: &[(&str, DataType, bool)]) -> Schema { ... }

// Add utilities for common random operations
pub struct RandomUtils<R: Rng> { rng: R }
impl<R: Rng> RandomUtils<R> {
    pub fn random_date_between(&mut self, start: NaiveDate, end: NaiveDate) -> NaiveDate { ... }
    pub fn weighted_choice<T>(&mut self, choices: &[(T, f64)]) -> &T { ... }
}
```

### src/models.rs

```rust
// Add derive traits where appropriate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AkmRecord { ... }

// Add conversion methods from records to Arrow arrays
impl AkmRecord {
    pub fn to_arrow_arrays(records: &[Self]) -> Vec<Arc<dyn Array>> { ... }
}

// Add From implementations for Row conversion
impl From<AkmRecord> for Vec<arrow::array::ArrayRef> { ... }
```

## Implementation Priorities

1. First, refactor error handling to have a proper error type
2. Implement generator traits and shared utilities
3. Refactor individual generators to use the new abstractions
4. Improve the writer interface
5. Add tests for the refactored code

This approach will make the code more maintainable, more idiomatic, and easier to extend in the future.
