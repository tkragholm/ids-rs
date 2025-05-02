//! Registry loaders for various Danish registry data sources
//!
//! This module contains registry loaders for various Danish registry data sources.

pub mod factory;
pub mod helper;
pub mod loaders;
pub mod traits;

// Re-export registry traits
pub use traits::{PnrFilter, RegisterLoader};

// Re-export registry loaders
pub use loaders::*;

// Re-export helper functions
pub use helper::{load_registry_data, load_registry_data_sync};
