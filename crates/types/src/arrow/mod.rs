//! Arrow integration utilities and types (legacy module).
//!
//! This module has been deprecated. Please use types from `crate::storage::arrow` directly.
//!
//! This empty module is kept for backward compatibility with existing imports.
//! It will be removed in a future version.

#[cfg(feature = "arrow-integration")]
pub use crate::storage::arrow;

#[cfg(not(feature = "arrow-integration"))]
compile_error!("The Arrow functionality requires the 'arrow-integration' feature to be enabled");