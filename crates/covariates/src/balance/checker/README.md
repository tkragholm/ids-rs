# Balance Checker Module

This module implements the core functionality for analyzing covariate balance between case and control groups in matched studies. It provides tools to evaluate whether the matching process has resulted in comparable groups.

## Module Structure

The balance checker implementation is organized into these components:

- **mod.rs**: Exports the `BalanceChecker` struct and related types
- **builder.rs**: Implements the builder pattern for creating and configuring balance checkers
- **balance_calculation.rs**: Core logic for calculating balance metrics
- **paired_analysis.rs**: Functions for analyzing matched pairs
- **performance.rs**: Performance measurement and optimization utilities

## Key Components

### BalanceChecker

The main public interface for balance checking operations. It provides methods to:

- Check covariate balance between case and control groups
- Calculate standardized differences for numeric and categorical variables
- Generate detailed reports on balance metrics
- Fetch and cache covariates efficiently

### BalanceCheckerBuilder

A builder for creating and configuring `BalanceChecker` instances with specific settings:

```rust
let checker = BalanceCheckerBuilder::new()
    .with_store(data_store)
    .with_cache_capacity(200_000)
    .with_debug_mode(true)
    .build()?;
```

### Balance Calculation

The core logic for calculating balance metrics across different covariate types:

- **Demographics**: Age, gender, family size, geographic location
- **Income**: Earnings, employment status
- **Education**: Education level, years of education
- **Occupation**: Job type, industry sector

### Paired Analysis

Specialized analysis for matched pairs that calculates:

- Balance metrics for each case-control pair
- Standardized differences at the individual level
- Detailed statistics for matched pair quality

### Performance Optimization

The module uses several approaches to optimize performance:

- **Caching**: Cache frequently accessed covariates
- **Parallel Processing**: Process data in parallel using rayon
- **Batch Processing**: Group similar operations to improve throughput
- **Memory Management**: Adaptive memory usage based on system capabilities

## Usage Example

```rust
// Create a balance checker
let checker = BalanceCheckerBuilder::new()
    .with_store(data_store)
    .build()?;

// Run balance analysis
let results = checker.check_balance(&matched_pairs)?;

// View results
println!("Overall balance quality: {}", results.get_balance_quality());
println!("Imbalanced variables: {}", results.get_imbalanced_variables().len());

// Generate detailed report
let report = results.generate_report();
report.save_to_file("balance_report.html")?;
```

## Integration with Other Modules

The balance checker integrates with:

- **Processor**: For efficient data processing
- **Metrics**: For statistical calculations
- **Legacy Cache**: For optimized data access
- **Results**: For structured output generation

## Design Considerations

1. **Memory Efficiency**: The module is designed to handle large datasets efficiently
2. **Extensibility**: New covariate types and processors can be easily added
3. **Configurability**: Many aspects of the balance checking can be customized
4. **Reliability**: Comprehensive error handling and diagnostics