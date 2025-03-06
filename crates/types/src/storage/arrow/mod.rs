//! Arrow integration utilities and types.
//!
//! This module provides integration with the Apache Arrow format,
//! enabling efficient columnar data storage and retrieval.
//!
//! Note: This module is only available when the `arrow-integration` feature is enabled.

#[cfg(feature = "arrow-integration")]
pub mod access;
#[cfg(feature = "arrow-integration")]
pub mod convert;
#[cfg(feature = "arrow-integration")]
pub mod utils;
#[cfg(feature = "arrow-integration")]
pub mod backend;
#[cfg(feature = "arrow-integration")]
pub mod values;

// Re-export key types
#[cfg(feature = "arrow-integration")]
pub use access::ArrowAccess;
#[cfg(feature = "arrow-integration")]
pub use values::ArrowValue;
#[cfg(feature = "arrow-integration")]
pub use convert::{create_schema, RecordBatchConversion};
#[cfg(feature = "arrow-integration")]
pub use utils::ArrowUtils;
#[cfg(feature = "arrow-integration")]
pub use backend::ArrowBackend;

#[cfg(not(feature = "arrow-integration"))]
compile_error!("The Arrow functionality requires the 'arrow-integration' feature to be enabled");