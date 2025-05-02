//! Filtering functionality for `DataFrames`
//!
//! This module provides unified filtering for `DataFrames` and registry data.
//! It includes specialized filters for PNR/CPR, dates, and categorical values.

mod predicates;
mod pnr;
mod builder;

pub use predicates::*;
pub use pnr::*;
pub use builder::*;