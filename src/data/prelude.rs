//! Convenience re-exports for the data module
//!
//! This module re-exports commonly used items from the data module for convenience.

// Schema types
pub use super::schema::RegistrySchema;
pub use super::schema::registry;

// Registry types
pub use super::registry::RegisterLoader;
pub use super::registry::loaders;
pub use super::registry::factory::RegistryFactory;

// Filter types
pub use super::filter::PnrFilter;
pub use super::filter::{
    date_range_filter, 
    categorical_filter,
    numeric_range_filter,
    non_null_filter,
    prefix_filter,
    contains_filter,
    substring_filter,
    hash_set_filter,
    and_filters,
    or_filters,
    FilterBuilder
};

// Pruning types
pub use super::pruning::{
    FileStatistics,
    RegistryPruningStatistics,
    PrunableTableProvider,
    create_pnr_pruning_predicate,
    create_pruning_predicate
};

// IO utilities
pub use super::io::{
    ParquetReader, 
    load_parquet_directory,
    GenericTableProvider,
    BaseTableProvider
};

// Transform utilities
pub use super::transform::{TransformPipeline, date_range_transform, filter_missing_values};
pub use super::transform::aggregations::*;
pub use super::transform::conversions::*;
pub use super::transform::joins::*;

// Query utilities
pub use super::query::RegistrySqlEngine;