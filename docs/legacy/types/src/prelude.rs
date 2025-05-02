//! Convenient imports for common types and traits.
//!
//! This module re-exports the most commonly used types and traits from the crate,
//! allowing users to import everything they need with a single import statement.
//!
//! # Examples
//!
//! ```
//! use types::prelude::*;
//!
//! // Create a data store
//! let mut store = DataStore::new();
//!
//! // Create and use covariates
//! let education = EducationBuilder::new("higher")
//!     .with_years(16.0)
//!     .build();
//! ```
//!
//! ## Feature-gated imports
//!
//! Some imports are only available when specific features are enabled:
//!
//! ```
//! use types::prelude::*;
//!
//! // Arrow integration (requires 'arrow-integration' feature)
//! # #[cfg(feature = "arrow-integration")]
//! let arrow_backend = ArrowBackend::new();
//!
//! // Logging utilities (requires 'logging' feature)
//! # #[cfg(feature = "logging")]
//! utils::logging::init_logger().unwrap();
//! ```

// Error handling
pub use crate::error::{ErrorContext, IdsError, Result};
pub use crate::{bail, ensure, try_with_context};

// Core data models
pub use crate::models::{Covariate, CovariateType, CovariateValue, TimeVaryingValue};

// Builder types
pub use crate::models::covariate::builders::{
    DemographicsBuilder, EducationBuilder, IncomeBuilder, OccupationBuilder,
};

// PNR and Family
pub use crate::models::family::FamilyRelations;
pub use crate::models::{FamilyInfo, ParentPair, PersonInfo, Pnr, PnrPool};

// Storage and backends
pub use crate::store::DataStore;
pub use crate::traits::access::Backend;

// Arrow integration (only when arrow-integration feature is enabled)
#[cfg(feature = "arrow-integration")]
pub use crate::storage::arrow::{ArrowAccess, ArrowBackend, ArrowValue};

// TimeVaryingBackend (only when serde-support feature is enabled)
pub use crate::store::TimeVaryingBackend;

// Traits
pub use crate::traits::{Cacheable, CovariateProcessor, DateHelpers, FamilyAccess, Store};

// Legacy traits have been removed in this version

// Utilities
pub use crate::utils::date::{format_date, parse_date, parse_year};
pub use crate::utils::string::{sanitize_identifier, truncate};

// Logging utilities (only when logging feature is enabled)
#[cfg(feature = "logging")]
pub use crate::{log_debug, log_error, log_info, log_warn};

// Common external types
pub use chrono::{Datelike, NaiveDate};
pub use hashbrown::HashMap;

/// Commonly used modules namespace.
///
/// This allows users to access specific modules directly through the prelude:
/// ```
/// use types::prelude::*;
/// use types::prelude::models::covariate::builders::EducationBuilder;
/// ```
pub mod modules {
    pub use crate::error;
    pub use crate::models;
    pub use crate::storage;
    pub use crate::traits;
    pub use crate::utils;
}
