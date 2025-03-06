// Core modules
pub mod arrow;
pub mod config;
pub mod error;
pub mod family;
pub mod models;
pub mod prelude;
pub mod store;
pub mod traits;
pub mod translation;

// New refactored modules
pub mod storage;

// Re-export commonly used types
pub use self::traits::access::ArrowAccess;
pub use self::error::IdsError;
pub use self::family::FamilyRelations;
pub use self::models::{Covariate, CovariateType, CovariateValue, TimeVaryingValue};
pub use self::models::pnr::{PnrPool, PersonInfo, ParentPair, FamilyInfo};
pub use self::traits::{Cacheable, Store};

// Re-exports for backwards compatibility
pub use self::arrow as arrow_utils;
pub use self::error::prelude::ErrorContext as Context;
pub use self::store as storage_old;

// Type aliases for backward compatibility during transition
pub type OldFamilyRelations = family::relations::FamilyRelations;
pub type SamplingError = error::IdsError;
pub type IdsResult<T> = error::Result<T>;
