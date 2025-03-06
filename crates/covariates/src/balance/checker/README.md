# Balance Checker Module

This directory contains the implementation of the `BalanceChecker` which is responsible for analyzing covariate balance between case and control groups in matched studies.

## Module Structure

The balance checker functionality has been split into several focused modules:

- **mod.rs**: Core `BalanceChecker` struct definition and public API
- **builder.rs**: Builder pattern implementation for flexible checker construction
- **diagnostic.rs**: Diagnostic and performance analysis capabilities
- **balance_calculation.rs**: Core balance calculation logic for different covariate types
- **paired_analysis.rs**: Matched pair analysis functionality
- **performance.rs**: Performance optimization utilities

## Flow of Functionality

1. **Initialization**: The checker is created either directly or using the builder pattern
2. **Data Loading**: Covariates are loaded and cached from the underlying data store
3. **Balance Calculation**: Domain-specific calculations for demographics, income, education, and occupation
4. **Paired Analysis**: Detailed analysis of individual matched pairs
5. **Result Generation**: Creation of comprehensive balance results

## Key Components

### BalanceChecker

The main entry point for balance checking functionality. It orchestrates the overall process and delegates to specialized components.

```rust
// Creating a balance checker
let checker = BalanceChecker::new(data_store);

// Or using the builder pattern
let checker = BalanceChecker::builder()
    .with_store(data_store)
    .with_cache_capacity(200_000)
    .build()?;

// Running a balance check
let results = checker.calculate_balance(cases, controls)?;
```

### Balance Calculation

The balance calculation process:

1. Calculates standardized mean differences for numeric variables
2. Calculates proportional differences for categorical variables
3. Handles missing data appropriately
4. Provides detailed metrics for each variable

### Optimization Features

- **Memory Management**: Efficient handling of large datasets
- **Caching**: Sophisticated caching system to avoid redundant data loading
- **Parallel Processing**: Utilizes parallel computation for performance
- **Prefetching**: Proactively loads data to improve performance

## Usage Examples

Basic usage:

```rust
use covariates::prelude::*;

// Create a balance checker
let checker = BalanceChecker::new(data_store);

// Calculate balance
let results = checker.calculate_balance(&cases, &controls)?;

// Generate reports
let report = BalanceReport::new(&results);
report.save_to_file("balance_report.html")?;
```

With prefetching for better performance:

```rust
// Prefetch data for better performance
let covariate_types = [
    CovariateType::Demographics,
    CovariateType::Income,
    CovariateType::Education,
    CovariateType::Occupation,
];

checker.prefetch_data(&all_pnrs, &covariate_types, &unique_dates);

// Then calculate balance
let results = checker.calculate_balance(&cases, &controls)?;
```