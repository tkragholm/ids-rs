# Migration Missing Functionality

This document outlines key functionality that hasn't been fully migrated from the original multi-crate structure to the new single-crate structure.

## Critical Priority

### Advanced Balance Analysis
- **Original Path**: `/crates/covariates/src/balance/checker/`
- **Description**: Sophisticated balance checking with optimization, paired analysis, and performance monitoring
- **Missing Components**:
  - Memory management for large datasets (`memory.rs`)
  - Performance optimization strategies
  - Paired sample analysis
  - Advanced balance calculation algorithms

## High Priority

### Covariates Processing System
- **Original Path**: `/crates/covariates/src/processing/`
- **Description**: Advanced covariate processing modules for demographics, education, income, and occupation
- **Missing Components**:
  - Processor factory system
  - Specialized processors for different data domains
  - Configurable processor interfaces

### Time-Varying Data Support
- **Original Path**: `/crates/types/src/models/time_varying.rs` and `/crates/types/src/storage/backends/time_varying.rs`
- **Description**: Support for data that changes over time
- **Missing Components**:
  - Time-varying data models
  - Time-varying data storage backends
  - Temporal query capabilities

### Advanced Data Loading Patterns
- **Original Path**: `/crates/loader/src/loaders/`
- **Description**: Specialized loading patterns for different data sources
- **Missing Components**:
  - Parallel loader implementation
  - Sequential loader implementation
  - Base loader abstractions
  - Custom path readers

### Core Matching Quality Assessment
- **Original Path**: `/crates/core/src/matching_quality.rs`
- **Description**: Tools for assessing match quality
- **Missing Components**:
  - Match quality metrics
  - Match quality assessment tools
  - Validation capabilities

## Medium Priority

### Family Relations Processing
- **Original Path**: `/crates/types/src/family/` and `/crates/types/src/models/family/`
- **Description**: Specialized family relationship handling and storage
- **Missing Components**:
  - Family relationship modeling
  - Family store implementation
  - Relationship graph functionality

### Comprehensive Reporting System
- **Original Path**: `/crates/covariates/src/reporting/`
- **Description**: Advanced reporting capabilities beyond basic CSV exports
- **Missing Components**:
  - Comprehensive report generation
  - Structured output management
  - Multiple report format support

### Advanced Error Handling
- **Original Path**: `/crates/types/src/error/` 
- **Description**: Sophisticated error handling with context, macros and conversions
- **Missing Components**:
  - Error context system
  - Error conversion utilities
  - Error macros

### Caching and Performance Optimizations
- **Original Path**: `/crates/types/src/traits/cacheable.rs`
- **Description**: Caching system for improved performance
- **Missing Components**:
  - Cacheable trait implementation
  - Performance optimization traits

## Migration Plan

1. Focus on migrating critical components first
2. Progressively add high-priority features
3. Address medium-priority items once the core system is stable