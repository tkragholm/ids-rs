# Covariates Crate

The `covariates` crate provides functionality for managing, processing, and analyzing covariate data in matched case-control studies. It handles the preprocessing, balance checking, and reporting of covariate balance between treatment and control groups.

## Recent Refactoring Improvements

The codebase has undergone significant modularization to improve maintainability and readability:

1. **Processor Module Refactoring**:
   - Split the monolithic 700+ line `processor.rs` into modular components
   - Created dedicated modules for numeric and categorical processing
   - Separated configuration, date grouping, and progress reporting into reusable components
   - Improved memory management with specialized handlers

2. **Optimization Strategies**:
   - Extracted optimization strategies into a separate module
   - Added memory-tier based strategy selection
   - Re-enabled parallel processing capabilities

3. **Better Type Organization**:
   - Added builder patterns for complex types
   - Created better parameter structures to reduce function argument counts
   - Improved reuse of common patterns

## Structure

The crate is organized into logical modules based on functionality:

### Core (`src/core/`)

Contains the fundamental components for configuration and processor management.

- **core/config.rs** - Configuration structures and functionality for covariate processing
- **core/registry.rs** - Registry for managing covariate processors
- **core/mod.rs** - Core module exports and error type definitions

### Data (`src/data/`)

Handles data storage and management of matched pairs.

- **data/storage.rs** - Storage mechanisms for covariate data
- **data/matched_pairs/** - Functionality for working with matched case-control pairs
  - **matched_pairs/loader.rs** - Loading matched pairs from files
  - **matched_pairs/record.rs** - Data structures for matched pair records
  - **matched_pairs/mod.rs** - Matched pairs module exports

### Processing (`src/processing/`)

Implements the processing of covariates by various domains.

- **processing/processor.rs** - Base processor interfaces
- **processing/factory.rs** - Factory for creating processor instances
- **processing/mod.rs** - Processing module exports

Domain-specific processors:
- **processing/demographic/** - Demographics-related covariate processors
- **processing/education/** - Education-related covariate processors
- **processing/income/** - Income-related covariate processors
- **processing/occupation/** - Occupation-related covariate processors

### Balance (`src/balance/`)

Provides functionality for checking covariate balance between treatment and control groups.

- **balance/checker/** - Modularized balance checker implementation
  - **checker/builder.rs** - Builder pattern for balance checker
  - **checker/balance_calculation.rs** - Core balance calculation logic
  - **checker/paired_analysis.rs** - Analysis for matched pairs
  - **checker/performance.rs** - Performance measurement and optimization
  - **checker/mod.rs** - Balance checker exports
- **balance/metrics.rs** - Balance metrics calculations
- **balance/optimization.rs** - Optimization strategies for processing
- **balance/proc_impl/** - Modularized processor implementation
  - **proc_impl/numeric.rs** - Numeric value processing
  - **proc_impl/categorical.rs** - Categorical value processing
  - **proc_impl/config.rs** - Processor configuration
  - **proc_impl/date_grouping.rs** - Date-based grouping for optimization
  - **proc_impl/progress.rs** - Progress reporting utilities
  - **proc_impl/mod.rs** - Processor implementation exports
- **balance/processor.rs** - Main processor facade
- **balance/results.rs** - Data structures for balance check results
- **balance/stats.rs** - Statistical functions for balance checking
- **balance/memory.rs** - Memory management for large balance datasets
- **balance/legacy_cache.rs** - Legacy caching mechanisms
- **balance/mod.rs** - Balance module exports

### Reporting (`src/reporting/`)

Handles the generation of reports from analysis results.

- **reporting/balance_report.rs** - Balance report generation
- **reporting/comprehensive_report.rs** - Comprehensive analysis reports
- **reporting/csv_report.rs** - CSV report generation
- **reporting/structured_output.rs** - Structured output management
- **reporting/mod.rs** - Reporting module exports

### Other Components

- **models.rs** - Domain models for covariate data
- **prelude.rs** - Convenient imports for common functionality
- **lib.rs** - Crate root and public API exports

## Usage

The crate provides a simple API for working with covariates:

```rust
use covariates::prelude::*;

// Create a configuration
let config = CovariatesConfig::default_config();

// Create a processor registry
let registry = CovariateProcessorRegistry::from_config(config);

// Load matched pairs
let matched_pairs = load_matched_pairs("path/to/matched_pairs.csv")?;

// Create a balance checker and run balance analysis
let checker = BalanceCheckerBuilder::new()
    .with_store(data_store)
    .build()?;
let results = checker.check_balance(&matched_pairs)?;

// Generate reports
let report = BalanceReport::new(&results);
report.save_to_file("balance_report.html")?;
```

## Features

- Support for various covariate types (demographics, income, education, occupation)
- Balance analysis with standardized mean difference calculation
- Configurable processing with custom variables
- Comprehensive reporting capabilities
- Efficient data storage for large datasets
- Parallel processing for performance

## Design Philosophy

The crate follows these design principles:

1. **Modularity**: Clear separation of concerns with focused modules
2. **Configurability**: Flexible configuration options for different use cases
3. **Performance**: Efficient processing of large datasets using parallel computation
4. **Type Safety**: Strong typing to prevent errors at compile time
5. **Extensibility**: Easy to add new covariate types and processors
6. **Maintainability**: Small, focused components with clear responsibilities