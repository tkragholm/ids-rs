# IDS Loader Crate

The `loader` crate provides efficient data loading capabilities for the Danish register data, supporting both sequential and parallel loading strategies. It handles various data formats and register types, with a focus on performance and memory efficiency.

## Implementation Status

### Current State

All registry loaders have been implemented, following consistent patterns:
- `akm.rs`, `bef.rs`, `ind.rs`, `uddf.rs` follow similar implementation patterns with variations as needed
- `family.rs` has a specialized implementation for single file loading with relationship parsing

### Known Issues and Limitations

1. **Type System Issues**:
   - There are inconsistencies between `ArrowBackend` vs `Result<ArrowBackend, IdsError>` in:
     - `loaders/sequential.rs`
     - `loaders/parallel.rs`
   - The correct approach to using `ArrowStore` in these contexts needs resolution

2. **PNR Filtering**:
   - Current implementation in `formats/parquet.rs` is a workaround
   - Proper filtering directly on RecordBatch level is planned

3. **Parallel Loader Challenges**:
   - MutexGuard issues with `ArrowBackend` in `loaders/parallel.rs` need resolution
   - Architectural improvements may be needed for result combination

### Next Steps

1. **Fix Type System Issues**:
   - Resolve the correct patterns for `ArrowBackend` usage
   - Address MutexGuard issues in parallel loading

2. **Implement Proper Filtering**:
   - Replace the temporary PNR filtering approach with proper RecordBatch filtering

3. **Testing & Documentation**:
   - Add comprehensive tests for each loader
   - Improve documentation of common usage patterns

## Architecture

The crate is organized into domain-specific modules, each with a clearly defined responsibility:

```
loader/
├── config/      - Configuration types and environment settings
├── formats/     - File format handling (parquet, etc.)
├── loaders/     - Core loader implementations (sequential, parallel)
├── readers/     - Data reader abstractions
├── registry/    - Register-specific loading logic
├── schema/      - Data schemas for different registers
└── ui/          - User interface components (progress tracking, console output)
```

### Key Components

#### Configuration (`config/`)

Handles all configuration-related code for the data loading process:

- `config/env.rs` - Environment variable handling
- `config/path.rs` - Path resolution and validation
- `config/loader_config.rs` - Main configuration types

```rust
// Example: Creating a loader configuration
let config = LoaderConfig::new("/path/to/data")
    .with_custom_path("akm", "/path/to/akm")
    .with_batch_size(65536)
    .with_max_threads(8);
```

#### File Formats (`formats/`)

Manages the reading and processing of different file formats:

- `formats/parquet.rs` - Parquet file handling with optimized reading

```rust
// Example: Reading a parquet file
let batches = read_parquet(&path, Some(&schema), Some(&progress), pnr_filter)?;
```

#### Loaders (`loaders/`)

Core implementations for loading data into the store:

- `loaders/base.rs` - `StoreLoader` trait definition
- `loaders/sequential.rs` - Sequential loading implementation
- `loaders/parallel.rs` - Parallel loading implementation

```rust
// Example: Loading data with the parallel loader
let store = ParallelLoader::load_from_path("/path/to/data")?;
```

#### Readers (`readers/`)

Abstractions for reading data from different sources:

- `readers/file.rs` - File-based reader
- `readers/custom_path.rs` - Custom path reader

```rust
// Example: Using a file reader
let reader = FileReader::new("/path/to/data");
let batches = reader.read_akm(2020)?;
```

#### Registry (`registry/`)

Register-specific loading logic:

- `registry/akm.rs` - Annual Register (AKM) loading (employment data)
- `registry/bef.rs` - Population Register (BEF) loading (demographic data)
- `registry/family.rs` - Family relations loading (parent-child relationships)
- `registry/ind.rs` - Individual Register (IND) loading (income data)
- `registry/uddf.rs` - Education Register (UDDF) loading (education data)

Each registry loader implements specialized handling for its respective register format, with common patterns:
- Year-based filtering
- PNR filtering optimization
- Progress tracking
- Automatic schema application
- Proper error handling and reporting

```rust
// Example: Loading AKM data
let akm_data = load_akm("/path/to/data", Some(&pnr_filter))?;
```

#### Schema (`schema/`)

Defines the data schemas for different register types:

- `schema/akm.rs` - AKM schema
- `schema/bef.rs` - BEF schema
- `schema/family.rs` - Family schema
- `schema/ind.rs` - IND schema
- `schema/uddf.rs` - UDDF schema
- `schema/utils.rs` - Schema utility functions

```rust
// Example: Getting the AKM schema
let schema = akm_schema();
```

#### UI (`ui/`)

User interface components for tracking progress and displaying information:

- `ui/progress.rs` - Progress tracking
- `ui/console.rs` - Console output utilities

```rust
// Example: Tracking progress
let progress = LoaderProgress::new();
progress.set_main_message("Loading data...");
// ... perform work ...
progress.inc_main();
```

## Performance Optimization

The loader crate implements several performance optimizations:

1. **Parallel Loading** - Uses multiple threads to load data in parallel
2. **PNR Filtering** - Filters data by PNR at load time to reduce memory usage
3. **Batch Size Control** - Configurable batch sizes for optimized memory usage
4. **Thread Count Control** - Configurable thread count for different environments
5. **Year-based Loading** - Only loads data for specified years to reduce memory footprint
6. **Optimized Parquet Reading** - Leverages Arrow's predicate pushdown for efficient file reading

These optimizations can be controlled via environment variables:

- `IDS_BATCH_SIZE` - Controls the batch size (default: 65536)
- `IDS_MAX_THREADS` - Controls the maximum number of threads (default: number of CPU cores)
- `IDS_USE_FAMILY_FILTERING` - Controls whether to use family-based filtering (default: false)
- `IDS_PARALLEL_AKM` - Controls whether to load AKM data in parallel (default: true)
- `IDS_PARALLEL_BEF` - Controls whether to load BEF data in parallel (default: true)
- `IDS_PARALLEL_IND` - Controls whether to load IND data in parallel (default: true)
- `IDS_PARALLEL_UDDF` - Controls whether to load UDDF data in parallel (default: true)

## Usage Examples

### Basic Usage

```rust
use loader::{ParallelLoader, StoreLoader};

// Load data from a path
let store = ParallelLoader::load_from_path("/path/to/data")?;

// Access data from the store
let akm_data = store.get_akm_data();
```

### Custom Paths

```rust
use loader::{RegisterPathConfig, SequentialLoader, StoreLoader};

// Create a configuration with custom paths
let config = RegisterPathConfig::new("/base/path")
    .with_custom_path("akm", "/path/to/akm")
    .with_custom_path("bef", "/path/to/bef");

// Load data with custom paths
let store = SequentialLoader::load_with_custom_paths(config)?;
```

### PNR Filtering

```rust
use loader::{LoaderConfig, ParallelLoader};
use std::collections::HashSet;

// Create a set of PNRs to filter by
let mut pnr_set = HashSet::new();
pnr_set.insert("1234567890".to_string());

// Create a configuration with PNR filtering
let config = LoaderConfig::new("/path/to/data")
    .with_pnr_filter(pnr_set);

// Load data with the configuration
let store = ParallelLoader::load_with_custom_paths(config.path_config)?;
```

## Design Principles

The `loader` crate follows these design principles:

1. **Separation of Concerns** - Each module has a specific responsibility
2. **Modularity** - Components are modular and can be used independently
3. **Performance** - Optimized for speed and memory efficiency
4. **Flexibility** - Configurable to adapt to different environments and use cases
5. **Ergonomics** - Easy to use API with sensible defaults

## Development Notes

When working on the loader crate, be aware of these considerations:

1. **Arrow Type System** - The crate heavily relies on Apache Arrow for efficient data representation:
   - Understanding the Arrow RecordBatch model is essential
   - Arrow's type system may require careful handling, especially with date/time types

2. **Module Visibility** - Ensure that modules are properly exposed in mod.rs files:
   - Public modules should be re-exported at the appropriate level
   - Internal implementation details should be kept private

3. **Error Handling** - Use the `IdsError` type consistently:
   - Propagate errors with the `?` operator
   - Add context to errors to assist in debugging
   - Avoid panic points (`unwrap()`, `expect()`) in production code

4. **Configuration** - Respect the provided configuration:
   - Path resolution should follow the configuration hierarchy
   - Environment variables should override defaults but not explicit settings
   - Validate paths and other user inputs before use

## Contributing

When contributing to the `loader` crate:

1. Follow the existing architecture and module organization
2. Keep modules focused and files small
3. Write comprehensive documentation
4. Add tests for new functionality
5. Follow Rust best practices and idiomatic code style
6. Ensure performance is maintained or improved
7. Address the known issues mentioned in the "Implementation Status" section
8. When adding a new registry loader:
   - Follow the patterns in existing registry loaders
   - Ensure proper schema definition
   - Implement both sequential and parallel loading support
   - Add appropriate progress tracking
   - Handle PNR filtering consistently

## License

This project is licensed under the terms of the license included with this repository.