//! Schema definitions for the IDS-RS library
//!
//! This module contains schema definitions for various data sources.

pub mod akm;
pub mod bef;
pub mod ind;
pub mod uddf;
pub mod parquet;

// Re-export parquet utilities
pub use parquet::{read_parquet, load_parquet_files_parallel};