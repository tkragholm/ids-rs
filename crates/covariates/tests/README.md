# Covariates Crate Tests

This directory contains tests and diagnostic tools for the covariates crate.

## Recent Improvements

1. **Moved Diagnostic Code to Tests**: The diagnostic functionality has been moved from the source code to the test module, improving code organization and separating test utilities from production code.

2. **Re-enabled Parallel Processing**: Parallel iteration in the paired_analysis.rs file has been restored, which should significantly improve performance.

## Diagnostic Module

The `diagnostic.rs` file contains testing utilities that were previously part of the core `BalanceChecker` implementation but have been moved to the test module for better separation of concerns. This module provides:

1. **BalanceCheckerDiagnostics Trait**: Extension trait that adds diagnostic capabilities to the `BalanceChecker` type, including:
   - Creating diagnostic checkers with simulated data
   - Analyzing cache performance
   - Generating performance metrics
   - Logging diagnostic information

2. **Test Data Generation**: Helper functions to create realistic test data for:
   - Demographics
   - Income
   - Education
   - Occupation

3. **Performance Analysis**: Tools to measure and analyze caching efficiency and memory usage

## Using the Diagnostic Tools

To use the diagnostic utilities in tests:

```rust
use covariates::balance::BalanceChecker;
use crate::BalanceCheckerDiagnostics; // Import the extension trait

#[test]
fn test_balance_analysis() {
    // Create a diagnostic checker with simulated data
    let checker = BalanceChecker::new_diagnostic();
    
    // Or create one with specific PNRs
    let pnrs = vec!["010101-1234".to_string(), "020202-5678".to_string()];
    let checker_with_pnrs = BalanceChecker::new_diagnostic_with_pnrs(pnrs);
    
    // Perform analysis
    let metrics = checker.analyze_cache_performance();
    assert!(metrics.total_entries > 0);
    
    // Log diagnostic information
    checker.log_diagnostic_information();
}
```

## Compatibility Changes

For backward compatibility, the `ids` crate now includes a simplified version of the diagnostic functionality. This ensures that existing code that relies on the diagnostic mode (like error recovery) continues to work, but without exposing internal implementation details.

## Test Organization

As the test suite grows, additional test modules may be added here following similar patterns:

1. **Unit Tests**: Focus on testing individual functions and methods
2. **Integration Tests**: Test interactions between multiple components
3. **Performance Tests**: Measure performance characteristics of critical operations
4. **Regression Tests**: Ensure previously fixed bugs don't reappear