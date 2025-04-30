# Code Examples for Proposed Structure

This document provides concrete code examples to illustrate the key components of the proposed restructuring of the registry and schema modules.

## 1. Schema Traits and Implementation

```rust
// src/data/schema/traits.rs

use arrow::datatypes::{Schema, Field, DataType};
use std::sync::Arc;

/// Trait for registry schemas defining common functionality
pub trait RegistrySchema {
    /// Get the schema name
    fn name() -> &'static str;
    
    /// Get the standard schema
    fn schema() -> Schema;
    
    /// Get the schema wrapped in an Arc for sharing
    fn schema_arc() -> Arc<Schema> {
        Arc::new(Self::schema())
    }
    
    /// Get the column names
    fn column_names() -> Vec<&'static str>;
    
    /// Get the primary key column name (e.g., PNR)
    fn primary_key_column() -> &'static str;
    
    /// Check if a column exists in the schema
    fn has_column(name: &str) -> bool {
        Self::column_names().contains(&name)
    }
}

// src/data/schema/registry/akm.rs

use super::super::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};

pub struct AkmSchema;

impl RegistrySchema for AkmSchema {
    fn name() -> &'static str {
        "AKM"
    }
    
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
    
    fn primary_key_column() -> &'static str {
        "PNR"
    }
}

// Legacy compatibility function
pub fn akm_schema() -> Schema {
    AkmSchema::schema()
}

pub fn akm_schema_arc() -> Arc<Schema> {
    AkmSchema::schema_arc()
}
```

## 2. Registry Loader Traits and Implementation

```rust
// src/data/registry/traits.rs

use arrow::record_batch::RecordBatch;
use std::path::Path;
use std::sync::Arc;
use crate::data::schema::traits::RegistrySchema;
use crate::error::Result;
use crate::data::io::filtering::PnrFilter;

/// Base trait for registry loaders
pub trait RegisterLoader {
    /// Associated schema type
    type Schema: RegistrySchema;
    
    /// Get the name of the register
    fn register_name(&self) -> &'static str {
        Self::Schema::name()
    }
    
    /// Get the schema for this register
    fn schema(&self) -> Arc<arrow::datatypes::Schema> {
        Self::Schema::schema_arc()
    }
    
    /// Load records from the register
    fn load(&self, base_path: &str, pnr_filter: Option<&PnrFilter>) -> Result<Vec<RecordBatch>>;
    
    /// Check if this register can handle a given path
    fn can_handle_path(&self, path: &Path) -> bool;
}

// src/data/registry/loaders/akm.rs

use std::path::Path;
use arrow::record_batch::RecordBatch;
use crate::data::schema::registry::akm::AkmSchema;
use crate::data::registry::traits::RegisterLoader;
use crate::data::io::filtering::PnrFilter;
use crate::data::io::parquet::ParquetReader;
use crate::error::Result;

pub struct AkmRegister;

impl RegisterLoader for AkmRegister {
    type Schema = AkmSchema;
    
    fn load(&self, base_path: &str, pnr_filter: Option<&PnrFilter>) -> Result<Vec<RecordBatch>> {
        let path = Path::new(base_path);
        
        let reader = ParquetReader::new(path)
            .with_schema(self.schema())
            .parallel(true);
            
        match pnr_filter {
            Some(filter) => reader.read_with_pnr_filter(filter),
            None => reader.read(),
        }
    }
    
    fn can_handle_path(&self, path: &Path) -> bool {
        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            file_name.to_lowercase().contains("akm")
        } else {
            false
        }
    }
}
```

## 3. Registry Factory Pattern

```rust
// src/data/registry/factory.rs

use std::collections::HashMap;
use std::path::Path;
use crate::data::registry::traits::RegisterLoader;
use crate::data::registry::loaders::akm::AkmRegister;
use crate::data::registry::loaders::bef::BefRegister;
use crate::data::registry::loaders::lpr::lpr2::LprAdmRegister;
// ... other imports
use crate::error::{IdsError, Result};

pub struct RegistryFactory;

impl RegistryFactory {
    /// List of all available registry loaders
    fn all_loaders() -> Vec<Box<dyn RegisterLoader>> {
        vec![
            Box::new(AkmRegister),
            Box::new(BefRegister),
            Box::new(LprAdmRegister),
            // ... other loaders
        ]
    }
    
    /// Create a registry loader from a registry name
    pub fn from_name(name: &str) -> Result<Box<dyn RegisterLoader>> {
        let name = name.to_lowercase();
        
        for loader in Self::all_loaders() {
            if loader.register_name().to_lowercase() == name {
                return Ok(loader);
            }
        }
        
        Err(IdsError::Validation(format!("Unknown registry: {name}")))
    }
    
    /// Create a registry loader based on a path
    pub fn from_path(path: &str) -> Result<Box<dyn RegisterLoader>> {
        let path = Path::new(path);
        
        for loader in Self::all_loaders() {
            if loader.can_handle_path(path) {
                return Ok(loader);
            }
        }
        
        Err(IdsError::Validation(format!(
            "Could not determine registry type from path: {}",
            path.display()
        )))
    }
    
    /// Create a map of all available registry loaders
    pub fn create_all() -> HashMap<&'static str, Box<dyn RegisterLoader>> {
        let mut map = HashMap::new();
        
        for loader in Self::all_loaders() {
            let name = loader.register_name();
            map.insert(name, loader);
        }
        
        map
    }
}
```

## 4. Unified PNR Filtering

```rust
// src/data/io/filtering.rs

use arrow::array::{Array, StringArray};
use arrow::compute;
use arrow::record_batch::RecordBatch;
use std::collections::HashSet;
use crate::error::{IdsError, Result};

/// Unified PNR filtering mechanism
pub struct PnrFilter {
    pnrs: HashSet<String>,
    column: String,
    indirect: Option<IndirectMapping>,
}

/// Describes an indirect PNR relationship
pub struct IndirectMapping {
    /// Column to use for the relation (e.g., RECNUM)
    relation_column: String,
    /// Related batches containing the mapping
    related_batches: Vec<RecordBatch>,
    /// Column with the PNR in the related batches
    pnr_column: String,
    /// Column with the relation key in the related batches
    key_column: String,
}

impl PnrFilter {
    /// Create a new filter for the default "PNR" column
    pub fn new(pnrs: HashSet<String>) -> Self {
        Self {
            pnrs,
            column: "PNR".to_string(),
            indirect: None,
        }
    }
    
    /// Create a filter using a different column name
    pub fn with_column(pnrs: HashSet<String>, column: impl Into<String>) -> Self {
        Self {
            pnrs,
            column: column.into(),
            indirect: None,
        }
    }
    
    /// Set up an indirect mapping (for LPR diagnoses, etc.)
    pub fn with_indirect_mapping(
        mut self,
        relation_column: impl Into<String>,
        related_batches: Vec<RecordBatch>,
        pnr_column: impl Into<String>,
        key_column: impl Into<String>,
    ) -> Self {
        self.indirect = Some(IndirectMapping {
            relation_column: relation_column.into(),
            related_batches,
            pnr_column: pnr_column.into(),
            key_column: key_column.into(),
        });
        
        self
    }
    
    /// Apply the filter to a record batch
    pub fn apply(&self, batch: &RecordBatch) -> Result<RecordBatch> {
        if let Some(ref indirect) = self.indirect {
            self.apply_indirect_filter(batch, indirect)
        } else {
            self.apply_direct_filter(batch)
        }
    }
    
    /// Apply a direct PNR filter
    fn apply_direct_filter(&self, batch: &RecordBatch) -> Result<RecordBatch> {
        // Get PNR column
        let col_idx = batch
            .schema()
            .index_of(&self.column)
            .map_err(|_| IdsError::Validation(format!("Column not found: {}", self.column)))?;
        
        let col_array = batch.column(col_idx);
        let string_array = col_array
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                IdsError::Validation(format!("Column is not a string column: {}", self.column))
            })?;
        
        // Create mask for matching PNRs
        let mut mask_builder = arrow::array::BooleanBuilder::new();
        
        for i in 0..string_array.len() {
            if string_array.is_null(i) {
                mask_builder.append_value(false);
            } else {
                let value = string_array.value(i);
                mask_builder.append_value(self.pnrs.contains(value));
            }
        }
        
        let mask = mask_builder.finish();
        
        // Apply filter
        compute::filter_record_batch(batch, &mask)
            .map_err(|e| IdsError::ArrowError(e.to_string()))
    }
    
    /// Apply an indirect filter (for related tables)
    fn apply_indirect_filter(&self, batch: &RecordBatch, indirect: &IndirectMapping) -> Result<RecordBatch> {
        // Implementation for indirect filtering through related batches
        // This would build a set of valid relation keys from the related batches
        // Then filter the target batch based on those keys
        
        // ... (implementation details omitted for brevity)
        
        Ok(batch.clone()) // Placeholder
    }
}
```

## 5. Unified Parquet I/O

```rust
// src/data/io/parquet.rs

use arrow::record_batch::RecordBatch;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use arrow::datatypes::Schema;
use crate::error::Result;
use crate::data::io::filtering::PnrFilter;

pub const DEFAULT_BATCH_SIZE: usize = 1024;

/// Unified Parquet reader with various loading strategies
pub struct ParquetReader {
    path: PathBuf,
    schema: Option<Arc<Schema>>,
    batch_size: usize,
    parallel: bool,
    async_loading: bool,
}

impl ParquetReader {
    /// Create a new reader for a path
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            schema: None,
            batch_size: DEFAULT_BATCH_SIZE,
            parallel: false,
            async_loading: false,
        }
    }
    
    /// Set the schema for this reader
    pub fn with_schema(mut self, schema: Arc<Schema>) -> Self {
        self.schema = Some(schema);
        self
    }
    
    /// Set the batch size for reading
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }
    
    /// Enable or disable parallel loading
    pub fn parallel(mut self, enable: bool) -> Self {
        self.parallel = enable;
        self
    }
    
    /// Enable or disable async loading
    pub fn async_loading(mut self, enable: bool) -> Self {
        self.async_loading = enable;
        self
    }
    
    /// Read all records
    pub fn read(&self) -> Result<Vec<RecordBatch>> {
        if self.path.is_file() {
            self.read_file(&self.path)
        } else if self.path.is_dir() {
            self.read_directory(&self.path)
        } else {
            Err(crate::error::IdsError::Validation(format!(
                "Path doesn't exist: {}",
                self.path.display()
            )))
        }
    }
    
    /// Read with PNR filtering
    pub fn read_with_pnr_filter(&self, filter: &PnrFilter) -> Result<Vec<RecordBatch>> {
        let batches = self.read()?;
        
        let filtered_batches = batches
            .into_iter()
            .map(|batch| filter.apply(&batch))
            .collect::<Result<Vec<_>>>()?;
            
        Ok(filtered_batches)
    }
    
    /// Read records from a single file
    fn read_file(&self, file_path: &Path) -> Result<Vec<RecordBatch>> {
        // Choose appropriate loading strategy based on configuration
        if self.async_loading {
            // self.read_file_async(file_path)
            unimplemented!("Async loading not implemented in this example")
        } else {
            self.read_file_sync(file_path)
        }
    }
    
    /// Read records from a directory
    fn read_directory(&self, dir_path: &Path) -> Result<Vec<RecordBatch>> {
        // Find all parquet files in the directory
        let files = self.find_parquet_files(dir_path)?;
        
        // Choose appropriate loading strategy based on configuration
        if self.parallel {
            if self.async_loading {
                // self.read_files_parallel_async(&files)
                unimplemented!("Async parallel loading not implemented in this example")
            } else {
                self.read_files_parallel(&files)
            }
        } else {
            let mut all_batches = Vec::new();
            
            for file in files {
                let batches = self.read_file(&file)?;
                all_batches.extend(batches);
            }
            
            Ok(all_batches)
        }
    }
    
    /// Find all Parquet files in a directory
    fn find_parquet_files(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        // ... Implementation omitted for brevity
        Ok(Vec::new()) // Placeholder
    }
    
    /// Read a single file synchronously
    fn read_file_sync(&self, file: &Path) -> Result<Vec<RecordBatch>> {
        // ... Implementation omitted for brevity
        Ok(Vec::new()) // Placeholder
    }
    
    /// Read multiple files in parallel
    fn read_files_parallel(&self, files: &[PathBuf]) -> Result<Vec<RecordBatch>> {
        // ... Implementation omitted for brevity
        Ok(Vec::new()) // Placeholder
    }
}
```

## 6. LPR Registry Example

```rust
// src/data/registry/loaders/lpr/mod.rs

pub enum LprVersion {
    V2,
    V3,
}

/// Common trait for LPR registries
pub trait LprRegistry: RegisterLoader {
    /// Get the LPR version
    fn version(&self) -> LprVersion;
    
    /// Find LPR files in a directory
    fn find_lpr_files(&self, path: &Path) -> Result<Vec<PathBuf>>;
}

// src/data/registry/loaders/lpr/lpr2.rs

use super::{LprRegistry, LprVersion};
use std::path::{Path, PathBuf};
use arrow::record_batch::RecordBatch;
use crate::data::registry::traits::RegisterLoader;
use crate::data::schema::registry::lpr::lpr2::LprAdmSchema;
use crate::data::io::filtering::PnrFilter;
use crate::data::io::parquet::ParquetReader;
use crate::error::Result;

pub struct LprAdmRegister;

impl RegisterLoader for LprAdmRegister {
    type Schema = LprAdmSchema;
    
    fn load(&self, base_path: &str, pnr_filter: Option<&PnrFilter>) -> Result<Vec<RecordBatch>> {
        let path = Path::new(base_path);
        
        let reader = if path.is_file() {
            ParquetReader::new(path).with_schema(self.schema())
        } else {
            let files = self.find_lpr_files(path)?;
            if files.is_empty() {
                return Ok(Vec::new());
            }
            
            ParquetReader::new(path)
                .with_schema(self.schema())
                .parallel(true)
        };
        
        match pnr_filter {
            Some(filter) => reader.read_with_pnr_filter(filter),
            None => reader.read(),
        }
    }
    
    fn can_handle_path(&self, path: &Path) -> bool {
        if let Some(name) = path.file_name().and_then(|f| f.to_str()) {
            name.to_lowercase().contains("lpr_adm")
        } else {
            false
        }
    }
}

impl LprRegistry for LprAdmRegister {
    fn version(&self) -> LprVersion {
        LprVersion::V2
    }
    
    fn find_lpr_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
        // Common function for finding LPR files by pattern
        // ... Implementation omitted for brevity
        Ok(Vec::new()) // Placeholder
    }
}
```

## 7. Transform Pipeline

```rust
// src/data/transform/mod.rs

use arrow::record_batch::RecordBatch;
use crate::error::Result;

/// Trait for data transformations
pub trait Transform {
    /// Apply the transformation to a record batch
    fn transform(&self, batch: &RecordBatch) -> Result<RecordBatch>;
    
    /// Get the name of this transformation
    fn name(&self) -> &str;
}

/// Pipeline for applying a series of transformations
pub struct TransformPipeline {
    transforms: Vec<Box<dyn Transform>>,
}

impl TransformPipeline {
    /// Create a new empty pipeline
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
        }
    }
    
    /// Add a transformation to the pipeline
    pub fn add<T: Transform + 'static>(mut self, transform: T) -> Self {
        self.transforms.push(Box::new(transform));
        self
    }
    
    /// Apply the pipeline to a single batch
    pub fn apply_to_batch(&self, batch: &RecordBatch) -> Result<RecordBatch> {
        let mut current = batch.clone();
        
        for transform in &self.transforms {
            current = transform.transform(&current)?;
        }
        
        Ok(current)
    }
    
    /// Apply the pipeline to multiple batches
    pub fn apply(&self, batches: &[RecordBatch]) -> Result<Vec<RecordBatch>> {
        batches
            .iter()
            .map(|batch| self.apply_to_batch(batch))
            .collect()
    }
}

// src/data/transform/filters.rs

use super::Transform;
use arrow::record_batch::RecordBatch;
use arrow::array::{Array, StringArray, Int32Array, Date32Array};
use arrow::compute;
use arrow::datatypes::DataType;
use chrono::{NaiveDate, Datelike};
use crate::error::{IdsError, Result};

/// Filter records by date range
pub struct DateRangeFilter {
    column: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl DateRangeFilter {
    pub fn new(
        column: impl Into<String>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            column: column.into(),
            start_date,
            end_date,
        }
    }
}

impl Transform for DateRangeFilter {
    fn transform(&self, batch: &RecordBatch) -> Result<RecordBatch> {
        // Implementation omitted for brevity
        Ok(batch.clone()) // Placeholder
    }
    
    fn name(&self) -> &str {
        "DateRangeFilter"
    }
}

/// Filter out records with missing values in specified columns
pub struct MissingValueFilter {
    columns: Vec<String>,
}

impl MissingValueFilter {
    pub fn new(columns: Vec<String>) -> Self {
        Self { columns }
    }
}

impl Transform for MissingValueFilter {
    fn transform(&self, batch: &RecordBatch) -> Result<RecordBatch> {
        // Implementation omitted for brevity
        Ok(batch.clone()) // Placeholder
    }
    
    fn name(&self) -> &str {
        "MissingValueFilter"
    }
}
```

## 8. Example Usage

```rust
// Example of using the new API

use crate::data::registry::factory::RegistryFactory;
use crate::data::io::filtering::PnrFilter;
use crate::data::transform::{TransformPipeline, filters::DateRangeFilter};
use std::collections::HashSet;
use chrono::NaiveDate;

fn load_and_process_data() -> Result<()> {
    // Create a PNR filter
    let mut pnrs = HashSet::new();
    pnrs.insert("1234567890".to_string());
    let pnr_filter = PnrFilter::new(pnrs);
    
    // Load data from a registry
    let registry = RegistryFactory::from_name("bef")?;
    let batches = registry.load("/path/to/bef/data", Some(&pnr_filter))?;
    
    // Create a transformation pipeline
    let start_date = NaiveDate::from_ymd_opt(2020, 1, 1);
    let end_date = NaiveDate::from_ymd_opt(2022, 12, 31);
    
    let pipeline = TransformPipeline::new()
        .add(DateRangeFilter::new("DATE", start_date, end_date));
        
    // Apply the transformations
    let transformed = pipeline.apply(&batches)?;
    
    // Do something with the transformed data
    println!("Loaded {} batches", transformed.len());
    
    Ok(())
}
```

These examples illustrate how the proposed restructuring would create a more intuitive, logical, and efficient organization of the registry and schema modules. The approach uses Rust's type system to provide better abstractions, reduce duplication, and increase maintainability.