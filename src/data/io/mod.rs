//! Input/Output utilities for registry data
//!
//! This module contains utilities for reading and writing registry data
//! with optimized access patterns using `DataFusion`.

pub mod async_utils;
pub mod base_provider;
pub mod datafusion;
pub mod datafusion_utils;
pub mod parquet;

// // Legacy modules - these are now deprecated
// // New code should use the modules in src/data/filter and src/data/pruning
// #[deprecated(
//     since = "0.2.0",
//     note = "Use data::filter instead, which provides a unified filtering API"
// )]
// pub mod filtering;

// #[deprecated(
//     since = "0.2.0",
//     note = "Use data::pruning instead, which provides a more modular pruning implementation"
// )]
// pub mod pruning;

pub use async_utils::*;
pub use base_provider::*;
// Use explicit imports to avoid ambiguous re-exports
pub use datafusion_utils::create_listing_table;
pub use datafusion_utils::create_mem_table;
pub use datafusion_utils::create_optimized_context;
pub use datafusion_utils::find_parquet_files;
pub use datafusion_utils::filter_by_pnrs;
pub use datafusion_utils::register_listing_table;
pub use datafusion_utils::register_tables_from_directories;
pub use parquet::*;

// Re-export from the new modules to maintain backward compatibility
pub use crate::data::filter::PnrFilter;
pub use crate::data::pruning::{create_pnr_pruning_predicate, create_pruning_predicate};
pub use crate::data::pruning::{FileStatistics, PrunableTableProvider, RegistryPruningStatistics};
