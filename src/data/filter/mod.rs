//! Filtering functionality for `DataFrames`
//!
//! This module provides unified filtering for `DataFrames` and registry data.
//! It includes specialized filters for PNR/CPR, dates, and categorical values.

pub mod builder;
pub mod pnr;
pub mod predicates;

pub use builder::*;
pub use pnr::*;
pub use predicates::*;
