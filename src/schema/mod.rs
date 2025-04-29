//! Schema definitions for the IDS-RS library
//!
//! This module contains schema definitions for various data sources.

pub mod akm;
pub mod bef;
pub mod dod;             // Death Register (DOD)
pub mod dodsaarsag;      // Death Cause Register (DODSAARSAG)
pub mod examples;
pub mod filter_expr;
pub mod idan;
pub mod ind;
pub mod lpr3_diagnoser;
pub mod lpr3_kontakter;
pub mod lpr_adm;
pub mod lpr_bes;
pub mod lpr_diag;
pub mod mfr;
pub mod parquet_async;
pub mod parquet_utils;
pub mod uddf;
pub mod vnds;            // Migration Register (VNDS)

// Re-export parquet utilities
pub use parquet_utils::{
    load_parquet_files_parallel, load_parquet_files_parallel_with_filter, read_parquet,
    read_parquet_with_filter,
};

// Re-export async parquet utilities
pub use parquet_async::{
    load_parquet_files_parallel_async, load_parquet_files_parallel_with_filter_async,
    read_parquet_async, read_parquet_with_filter_async, read_parquet_with_pnr_filter_async,
    DEFAULT_BATCH_SIZE,
};
