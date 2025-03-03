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