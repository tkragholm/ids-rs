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
pub use crate::error::{IdsError, Result, ErrorContext};
pub use crate::{bail, ensure, try_with_context};

// Core data models
pub use crate::models::{
    Covariate, 
    CovariateType, 
    CovariateValue, 
    TimeVaryingValue,
};

// Builder types
pub use crate::models::covariate::builders::{
    EducationBuilder,
    IncomeBuilder,
    OccupationBuilder,
    DemographicsBuilder,
};

// PNR and Family
pub use crate::models::{Pnr, PnrPool, PersonInfo, ParentPair, FamilyInfo};
pub use crate::models::family::{FamilyRelations};

// Storage and backends
pub use crate::store::DataStore;
pub use crate::traits::access::Backend;

// Arrow integration (only when arrow-integration feature is enabled)
#[cfg(feature = "arrow-integration")]
pub use crate::storage::arrow::{ArrowBackend, ArrowAccess, ArrowValue};

// TimeVaryingBackend (only when serde-support feature is enabled)
pub use crate::store::TimeVaryingBackend;

// Traits
pub use crate::traits::{
    Store, 
    DateHelpers, 
    FamilyAccess, 
    CovariateProcessor,
    Cacheable,
};

// Legacy traits (for backward compatibility)
#[doc(hidden)]
#[allow(deprecated)]
pub use crate::traits::{
    access::LegacyStore,
    LegacyStoreExt,
    LegacyFamilyAccess,
    LegacyTimeVaryingAccess
};

// Utilities
pub use crate::utils::date::{format_date, parse_date, parse_year};
pub use crate::utils::string::{sanitize_identifier, truncate};

// Logging utilities (only when logging feature is enabled)
#[cfg(feature = "logging")]
pub use crate::{log_debug, log_info, log_warn, log_error};

// Common external types
pub use hashbrown::HashMap;
pub use chrono::{NaiveDate, Datelike};

/// Commonly used modules namespace.
///
/// This allows users to access specific modules directly through the prelude:
/// ```
/// use types::prelude::*;
/// use types::prelude::models::covariate::builders::EducationBuilder;
/// ```
pub mod modules {
    pub use crate::models;
    pub use crate::storage;
    pub use crate::traits;
    pub use crate::utils;
    pub use crate::error;
}