//! Data handling for registry data
//!
//! This module contains functionality for loading, transforming, and querying registry data.
//!
//! For convenience, the most commonly used types are re-exported in the `prelude` module.
//!
//! ```no_run
//! use ids_rs::data::prelude::*;
//! ```

pub mod io;
pub mod prelude;
pub mod query;
pub mod registry;
pub mod schema;
pub mod transform;
//pub mod examples;

// New modules with improved organization
pub mod filter;
pub mod pruning;

// Re-export key types
pub use query::RegistrySqlEngine;
pub use filter::PnrFilter; // Updated from registry::PnrFilter
pub use registry::RegisterLoader;
pub use schema::RegistrySchema;
pub use transform::TransformPipeline;
