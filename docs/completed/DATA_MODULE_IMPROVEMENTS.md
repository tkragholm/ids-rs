# Data Module Organization Improvements

## Current Issues

After analyzing the codebase, the following organizational issues have been identified:

1. **Overlapping Responsibilities**:
   - The separation between `io` and `transform` modules isn't always clear
   - Similar filtering functionality exists in both modules
   - PnrFilter in `io/filtering.rs` vs filter operations in `transform/filters.rs`

2. **Large, Complex Files**:
   - `pruning.rs` is over 700 lines and handles multiple concerns
   - The TableProvider implementation could be better isolated

3. **Inconsistent Async Pattern Implementation**:
   - Async traits are handled differently across files
   - Some code uses direct async/await, others build runtimes to block

4. **Code Duplication**:
   - Filter creation and application logic appears in multiple places
   - Similar DataFusion context setup code in multiple locations

## Proposed Structure

Here's a proposed reorganization of the data module:

```
src/data/
├── io/                     # I/O operations
│   ├── mod.rs
│   ├── parquet.rs          # Parquet file I/O
│   ├── async_utils.rs      # Async utilities
│   └── datafusion.rs       # DataFusion-specific utilities
├── pruning/                # Pruning-specific code (extracted from io/pruning.rs)
│   ├── mod.rs
│   ├── statistics.rs       # File statistics
│   ├── predicate.rs        # Pruning predicates
│   └── provider.rs         # TableProvider implementation
├── filter/                 # Unified filtering (merge io/filtering.rs and transform/filters.rs)
│   ├── mod.rs
│   ├── predicates.rs       # Filter predicates
│   ├── pnr.rs              # PNR-specific filtering
│   └── builder.rs          # Filter builder pattern
├── transform/              # DataFrame transformations
│   ├── mod.rs
│   ├── aggregations.rs
│   ├── conversions.rs
│   └── joins.rs
├── query/                  # Query functionality
│   ├── mod.rs
│   └── sql.rs
├── registry/               # Registry-specific code
├── schema/                 # Schema definitions
└── mod.rs
```

## Specific Recommendations

### 1. Extract Pruning Code

The `pruning.rs` file is too large and handles multiple concerns. It should be split into:

- `pruning/statistics.rs`: File statistics and pruning statistics
- `pruning/predicate.rs`: Pruning predicate creation
- `pruning/provider.rs`: TableProvider implementation

### 2. Consolidate Filtering

Merge the filtering functionality from:
- `io/filtering.rs`
- `transform/filters.rs`

Into a unified filtering module with consistent patterns.

### 3. Standardize Async Implementation

- Use `#[async_trait::async_trait]` consistently
- Avoid creating tokio runtimes to block on futures
- Pass contexts explicitly where needed instead of trying to extract them

### 4. Clean Up Interfaces

- Make the TransformPipeline more ergonomic
- Standardize filter application
- Ensure consistent naming conventions across the module

### 5. Fix Clippy Warnings

- Address the unused imports and dead code
- Fix `manual_retain` and other clippy warnings
- Standardize error handling

## Implementation Strategy

1. First, fix the async implementation in TableProvider (already done)
2. Create the new directory structure
3. Move code to new locations with minimal changes
4. Consolidate duplicated functionality
5. Update imports and fix any resulting issues
6. Add deprecation notices for old APIs that should be migrated
7. Improve documentation to clarify the new organization

## Benefits

- Clearer separation of concerns
- More maintainable codebase
- More consistent async patterns
- Reduced code duplication
- Better developer experience