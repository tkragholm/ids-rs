//! Arrow integration utilities and types (legacy module).
//!
//! This module re-exports from storage/arrow for backward compatibility.
//!
//! Note: This module is only available when the `arrow-integration` feature is enabled.
//! New code should use types from `crate::storage::arrow` directly.

#[cfg(feature = "arrow-integration")]
pub use crate::storage::arrow::access;
#[cfg(feature = "arrow-integration")]
pub use crate::storage::arrow::convert;
#[cfg(feature = "arrow-integration")]
pub use crate::storage::arrow::utils;

// Re-export key types to maintain backward compatibility
#[cfg(feature = "arrow-integration")]
pub use access::ArrowAccess;
#[cfg(feature = "arrow-integration")]
pub use crate::storage::arrow::values::ArrowValue;
#[cfg(feature = "arrow-integration")]
pub use convert::{create_schema, RecordBatchConversion};
#[cfg(feature = "arrow-integration")]
pub use utils::ArrowUtils;

#[cfg(not(feature = "arrow-integration"))]
compile_error!("The Arrow functionality requires the 'arrow-integration' feature to be enabled");