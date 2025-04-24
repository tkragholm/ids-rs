//! IDS-RS: Integrated Data System for Research in Rust
//! 
//! This crate provides utilities for working with registry data, matching samples,
//! and analyzing covariates balance.

mod error;
pub use error::{IdsError, Result};

pub mod core;
pub mod model;
pub mod store;
pub mod registry;
pub mod schema;
pub mod algorithm;
pub mod cli;

// Re-export commonly used items
pub use core::date::DateExtensions;
pub use core::string::StringExtensions;
pub use model::pnr::Pnr;