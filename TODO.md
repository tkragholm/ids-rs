# Refactoring Progress

## Completed Changes

### 1. ✅ Consolidated Backend Implementations
- Merged duplicate `ArrowBackend` implementations into a single optimized version in `/crates/types/src/storage/arrow/backend.rs`
- Removed redundant implementation from `/crates/types/src/store/arrow_backend.rs`
- Updated all imports and references to use the consolidated implementation

### 2. ✅ Created Shared Cache Key Type
- Moved `CacheKey` to the central storage module to avoid duplication
- Updated all references to use the shared type
- Set up proper exports to maintain compatibility

### 3. ✅ Simplified DataStore Abstraction
- Replaced complex `DataStore` with a cleaner enum-based approach
- Removed unnecessary `Arc<dyn Store>` indirection
- Made delegation to specific backends more direct and explicit
- Eliminated redundant caching layer

### 4. ✅ Fixed Compiler Errors
- Resolved type mismatch issues in `ArrowBackend` implementation
- Fixed mutability issues in various trait implementations
- Addressed threading/concurrency issues with the Store trait

### 5. ✅ Streamlined Concurrency Approach
- Created a unified `ThreadSafeStore<S>` wrapper for all store implementations
- Standardized on `parking_lot::{RwLock, Mutex}` primitives for better performance
- Replaced `std::sync::Mutex` with more efficient `RwLock` for read-heavy workloads
- Implemented a high-performance `ShardedCache` with minimal lock contention
- Updated all dependent code to use the new concurrency primitives
- Created a single `CovariateCache` implementation that's optimized for concurrent access

## Current Work in Progress

### 1. 🔄 Simplify Caching Strategy

We've made significant progress on streamlining our caching approach, but there are still some inconsistencies and redundancies in the codebase. Our goal is to standardize on a single caching solution for all data access patterns.

#### Current findings:
- We now have a well-optimized `CovariateCache` implementation in the `storage::concurrency` module
- Legacy caching mechanisms are still used in some parts of the codebase
- There are opportunities to further optimize cache parameters based on actual usage patterns
- The cache invalidation strategy needs to be more consistent across the codebase

#### Implementation plan:
1. Replace all remaining custom cache implementations with the new `CovariateCache`
2. Standardize cache key generation and lookup patterns
3. Implement a more sophisticated cache eviction strategy based on usage
4. Add cache statistics and monitoring capabilities

## Remaining Tasks

### 2. Remove Legacy Compatibility Code
- [ ] Remove deprecated traits and methods
- [ ] Simplify type hierarchies and trait bounds
- [ ] Eliminate redundant compatibility layers

### 3. Consolidate Utility Functions
- [ ] Identify common utility functions (e.g., period finding, PNR lookups)
- [ ] Create dedicated utility modules with optimized implementations
- [ ] Use shared implementations consistently throughout the codebase

### 4. Optimize Data Structure Usage
- [ ] Standardize on specific collections based on actual performance needs
- [ ] Consider replacing complex custom solutions with simpler alternatives
- [ ] Profile for actual performance benefits of optimizations

### 5. Streamline Error Handling
- [ ] Standardize on the `?` operator for error propagation
- [ ] Make error types more specific for better handling
- [ ] Avoid ignoring errors without good reason

### 6. Reduce Lock Contention
- [ ] Consider lock-free data structures where possible
- [ ] Use fine-grained locking instead of locking entire stores
- [ ] Explore immutable data sharing with `Arc` without mutexes