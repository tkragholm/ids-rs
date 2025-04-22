//! # IDS Types
//!
//! Core type definitions and foundational abstractions for the IDS-RS workspace.
//!
//! This crate serves as the foundation for the entire IDS-RS system, providing:
//!
//! - Common data types and models for epidemiological research
//! - Trait definitions used across the codebase
//! - Error handling patterns and standardization
//! - Storage abstractions for different data backends
//! - Utilities for working with Arrow data
//!
//! ## Getting Started
//!
//! The easiest way to get started is to import the prelude module, which
//! provides all commonly used types and traits:
//!
//! ```rust
//! use types::prelude::*;
//!
//! // Create a data store
//! let mut store = DataStore::new();
//!
//! // Work with covariates
//! let education = EducationBuilder::new("higher")
//!     .with_years(16.0)
//!     .build();
//!
//! let demographics = DemographicsBuilder::new(2, 101, "nuclear")
//!     .with_age(42)
//!     .with_gender("M")
//!     .build();
//!
//! // Combine them using the builder pattern
//! let combined_covariate = CovariateBuilder::new()
//!     .with_education(education)
//!     .with_demographics(demographics)
//!     .build();
//! ```
//!
//! ## Core Components
//!
//! - **Models**: Data structures for demographic, health, and registry data
//! - **Traits**: Interfaces for covariate processing, data access, and storage
//! - **Error Handling**: Standardized error types and propagation patterns
//! - **Storage**: Abstractions for data storage and retrieval
//! - **Arrow Utilities**: Helpers for working with Apache Arrow data format
//!
//! ## Feature Flags
//!
//! The following feature flags will be available in future releases:
//!
//! - `arrow-integration` - Integration with Apache Arrow (enabled by default)
//! - `serde-support` - Serialization/deserialization via serde (enabled by default)
//! - `chrono-nightly` - Enables nightly chrono features for improved date handling
//! - `polars-integration` - Integration with the polars DataFrame library
//! - `logging` - Logging functionality (enabled by default)

// Core public modules
pub mod error;
pub mod models;
pub mod prelude;
pub mod storage;
pub mod traits;
pub mod utils;

// Internal modules - considered implementation details
// Only public for backward compatibility
// To maintain backward compatibility while encouraging use of the newer APIs,
// these modules are exported with #[doc(hidden)] to discourage their use.
#[doc(hidden)]
pub mod arrow;
#[doc(hidden)]
pub mod config;
#[doc(hidden)]
pub mod family;
#[doc(hidden)]
pub mod store;
#[doc(hidden)]
pub mod translation;

// Re-export essential types at the crate root
// These are the most commonly used types that users will need
pub use self::error::{IdsError, Result};
pub use self::models::{Covariate, CovariateType, CovariateValue, TimeVaryingValue};
pub use self::models::{Pnr, PnrPool};
pub use self::store::DataStore;
#[cfg(feature = "arrow-integration")]
pub use self::storage::arrow::ArrowBackend;
pub use self::traits::{Store, DateHelpers};

// Type aliases for backward compatibility
#[doc(hidden)]
pub type OldFamilyRelations = family::relations::FamilyRelations;