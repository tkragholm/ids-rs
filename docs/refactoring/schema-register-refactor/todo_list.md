# Schema Registry Refactoring Todo List

This document tracks the completed tasks for the schema registry refactoring project, organized by priority and component.

## High Priority Tasks

### ParquetReader Implementation
- [x] Update `src/data/io/parquet.rs` to use DataFusion for all Parquet operations
- [x] Implement Builder pattern for ParquetReader configuration
- [x] Add support for filtering at source level with predicate pushdown
- [x] Integrate with PnrFilter for efficient filtering
- [x] Support both synchronous and asynchronous loading
- [x] Add batch size and parallel loading configuration

### Transform Pipeline
- [x] Refactor `TransformPipeline` in `src/data/transform/mod.rs` to use DataFusion operations
- [x] Update join operations in `src/data/transform/joins.rs` for async usage
- [x] Implement transform operations as DataFusion plan modifications
- [x] Create common transformations (filter, select, aggregate, etc.)
- [x] Add support for operation chaining with proper DataFusion optimization

### Pruning Optimization
- [x] Implement statistics-based pruning in `src/data/pruning/mod.rs`
- [x] Create Parquet metadata readers for efficient file skipping
- [x] Add Bloom filter support for PNR filtering
- [x] Integrate pruning with DataFusion's scan operations
- [x] Create caching mechanism for pruning statistics

## Medium Priority Tasks


### Registry Loader Extensions
- [x] Add more registry types to the factory
- [x] Implement additional registry loaders using the new pattern
- [x] Update existing loaders to fully leverage DataFusion capabilities
- [x] Add batched loading support for large registries
- [x] Create caching mechanism for loaded registries

### Documentation and Examples
- [x] Add documentation for the new structure and components
- [x] Create examples for common operations
- [x] Write tutorials for extending the system
- [x] Add API documentation for public interfaces
- [x] Create usage guidelines for efficient querying

## Low Priority Tasks


### Error Handling and Logging
- [x] Improve error types and messages
- [x] Add structured logging throughout the codebase
- [x] Create debug utilities for DataFusion operations
- [x] Add tracing support for complex operations
- [x] Implement retry mechanisms for transient failures

### Performance Optimizations
- [x] Identify and optimize bottlenecks
- [x] Add memory usage tracking and optimization
- [x] Implement caching strategies for frequent operations
- [x] Add parallel execution for CPU-bound operations
- [x] Optimize I/O operations for large files

## Completed Tasks

### Foundation Setup
- [x] Create the new directory structure under `src/data/`
- [x] Define core traits (`RegistrySchema`, `RegisterLoader`, etc.)
- [x] Implement `PnrFilter` with direct and relation filtering
- [x] Create `RegistryFactory` for loader creation

### Schema Migration
- [x] Implement schema structure with parallel organization
- [x] Migrate LPR schemas (V2 and V3) to new structure
- [x] Create schema metadata handling
- [x] Implement `DynamicSchema` for runtime schema creation

### Registry Loaders
- [x] Update Registry Loaders to use DataFusion's API
- [x] Implement LPR-specific loader traits with version handling
- [x] Create `LprTableProvider` for DataFusion integration
- [x] Implement DataFusion scan method with file handling
- [x] Fix DataFusion 47.0.0 compatibility issues
- [x] Implement BEF (Population) registry loader and schema
- [x] Implement DOD (Death) registry loader and schema
- [x] Implement DODSAARSAG (Death Cause) registry loader and schema
- [x] Implement IDAN (Employment) registry loader and schema
- [x] Implement IND (Income) registry loader and schema
- [x] Implement UDDF (Education) registry loader and schema
- [x] Implement VNDS (Migration) registry loader and schema

### I/O Layer Foundation
- [x] Create DataFusion integration utilities
- [x] Implement optimized session context creation
- [x] Add PNR filter application to DataFrames
- [x] Implement batch to DataFrame conversion
