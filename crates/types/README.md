# IDS-RS Types

Core type definitions and foundational abstractions for the IDS-RS workspace.

## Overview

This crate serves as the foundation for the entire IDS-RS system, providing:

- Common data types and models for epidemiological research
- Trait definitions used across the codebase
- Error handling patterns and standardization
- Storage abstractions for different data backends
- Utilities for working with Arrow data

## Key Components

- **Models**: Data structures for demographic, health, and registry data
- **Traits**: Interfaces for covariate processing, data access, and storage
- **Error Handling**: Standardized error types and propagation patterns
- **Storage**: Abstractions for data storage and retrieval
- **Arrow Utilities**: Helpers for working with Apache Arrow data format

## Usage

This crate is primarily used as a dependency by other crates in the IDS-RS workspace. It defines the shared language and interfaces that enable all components to work together coherently.

```rust
use types::models::{Covariate, CovariateType};
use types::traits::CovariateProcessor;
```

## Dependencies

- Arrow ecosystem (arrow-array, arrow-schema, arrow-select)
- Serde for serialization
- Chrono for date handling
- Collection types (dashmap, hashbrown) for performant data structures

## Refactoring Plan

The following plan details a comprehensive restructuring of the types crate to create a cleaner, more logical organization.

### Current Issues

Based on analysis of the codebase, we've identified several organizational issues:

1. **Inconsistent Module Structure**: Related functionality is spread across multiple modules
2. **Poor Separation of Concerns**: Some modules mix multiple responsibilities
3. **Error Handling Duplication**: Multiple context-adding mechanisms exist
4. **Unclear Public API**: The public interface isn't clearly defined
5. **Trait Proliferation**: Too many similar traits with overlapping functionality
6. **Builder Pattern Inconsistency**: Variations in implementation style

### Proposed Directory Structure

```
types/
├── src/
│   ├── error/
│   │   ├── context.rs       # Error context trait and implementations
│   │   ├── conversion.rs    # From/TryFrom implementations for errors
│   │   └── mod.rs           # IdsError definition and re-exports
│   │
│   ├── models/
│   │   ├── covariate/
│   │   │   ├── builders.rs  # All builder implementations 
│   │   │   ├── types.rs     # CovariateType enum and implementations
│   │   │   ├── values.rs    # CovariateValue and implementations
│   │   │   └── mod.rs       # Covariate struct and re-exports
│   │   ├── family/
│   │   │   ├── relations.rs # Family relationships implementation
│   │   │   ├── store.rs     # Family data storage
│   │   │   └── mod.rs       # FamilyRelations and re-exports
│   │   ├── pnr.rs           # PNR handling moved under models
│   │   ├── time_varying.rs  # Time varying value implementations
│   │   └── mod.rs           # Re-exports and shared model code
│   │
│   ├── storage/
│   │   ├── arrow/
│   │   │   ├── access.rs    # ArrowAccess trait implementation
│   │   │   ├── backend.rs   # ArrowBackend implementation
│   │   │   ├── convert.rs   # Conversion between Arrow and domain types
│   │   │   ├── utils.rs     # Arrow utility functions
│   │   │   └── mod.rs       # Re-exports and configuration
│   │   ├── backends/
│   │   │   ├── memory.rs    # In-memory backend implementation
│   │   │   ├── time_varying.rs # Time varying data backend
│   │   │   └── mod.rs       # Backend trait and re-exports
│   │   ├── cache.rs         # Caching implementation
│   │   ├── store.rs         # DataStore implementation
│   │   └── mod.rs           # Re-exports and shared storage code
│   │
│   ├── traits/
│   │   ├── access.rs        # Data access traits
│   │   ├── cacheable.rs     # Cache-related traits
│   │   ├── processing.rs    # Data processing traits
│   │   ├── utils.rs         # Utility traits like DateHelpers
│   │   └── mod.rs           # Re-exports all traits
│   │
│   ├── utils/
│   │   ├── date.rs          # Date manipulation utilities
│   │   ├── translation.rs   # Translation maps and utilities 
│   │   ├── config.rs        # Configuration utilities
│   │   └── mod.rs           # Re-exports
│   │
│   ├── prelude.rs           # Convenient imports for users
│   └── lib.rs               # Crate root, module definitions
```

### Implementation Phases

#### Phase 1: Reorganize Directory Structure
- Move files to their new locations
- Update imports and re-exports
- Maintain backward compatibility via the prelude

#### Phase 2: Clean Up Interfaces
- Standardize trait implementations
- Consolidate similar functionality
- Remove redundant code

#### Phase 3: Improve Error Handling
- Standardize on a single error context mechanism
- Add better error documentation
- Implement more detailed error types

#### Phase 4: Optimize Public API
- Review public exports
- Ensure comprehensive documentation
- Create more examples for typical use cases

### Migration Notes

1. **Backward Compatibility**: The prelude module will maintain backward compatibility during refactoring
2. **Deprecation Markers**: Types that move will include deprecated attributes with guidance
3. **Testing Strategy**: Comprehensive tests will be created before and after refactoring
4. **Dependent Crates**: Update each dependent crate one at a time

### Benefits of New Structure

- **Improved Discoverability**: Logical organization makes it easier to find related code
- **Reduced Coupling**: Better separation of concerns between modules
- **Enhanced Maintainability**: Smaller, focused files are easier to understand and modify
- **Better Documentation**: Reorganization enables more comprehensive documentation
- **Clearer Abstractions**: Refined trait hierarchy makes the design more evident
- **Reduced Duplication**: Consolidated utility code reduces repeated patterns

This plan serves as a reference document for the refactoring process, helping to ensure that all changes are consistent with the overall design goals.