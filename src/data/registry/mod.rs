//! Registry loaders for various Danish registry data sources
//!
//! This module contains registry loaders for various Danish registry data sources.

pub mod traits;
pub mod loaders;
pub mod factory;

// Re-export registry traits
pub use traits::{RegisterLoader, PnrFilter};

// Re-export registry loaders
pub use loaders::*;