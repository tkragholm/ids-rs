//! Schema definitions for the IDS-RS library
//!
//! This module contains schema definitions for various data sources.

pub mod akm;
pub mod bef;
pub mod idan;
pub mod ind;
pub mod lpr_adm;
pub mod lpr_bes;
pub mod lpr_diag;
pub mod lpr3_diagnoser;
pub mod lpr3_kontakter;
pub mod uddf;
pub mod parquet;

// Re-export parquet utilities
pub use parquet::{read_parquet, load_parquet_files_parallel};