//! IDS-RS: Integrated Data System for Research in Rust
//!
//! This crate provides utilities for working with registry data, matching samples,
//! analyzing covariates balance, and generating population datasets.

mod error;
pub use error::{IdsError, Result};

pub mod algorithm;
pub mod cli;
pub mod commands;
pub mod core;
pub mod model;
pub mod registry;
pub mod schema;
pub mod store;
pub mod utils;

// Re-export commonly used items
pub use core::date::DateExtensions;
pub use core::string::StringExtensions;
pub use model::pnr::Pnr;
