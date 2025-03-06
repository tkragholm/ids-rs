//! Storage abstractions for different data backends.
//!
//! This module provides storage abstractions for working with different data backends,
//! including Arrow, in-memory storage, and time-varying data.
//!
//! The main types in this module are:
//!
//! - `DataStore`: The central data store that manages multiple backends
//! - `Backend`: Trait for implementing different storage backends
//! - `ArrowBackend`: Arrow-based storage backend implementation
//! - `MemoryBackend`: Simple in-memory storage backend
//! - `TimeVaryingBackend`: Backend for time-varying data

pub mod arrow;
pub mod backends;

// Re-export public types
pub use crate::store::DataStore;
pub use crate::traits::Store as Backend;

// Legacy re-exports
#[doc(hidden)]
pub use crate::store::arrow_backend::ArrowBackend as OldArrowBackend;
#[doc(hidden)]
pub use crate::store::time_varying_backend::TimeVaryingBackend;

// Future imports that will replace the legacy ones
pub use arrow::backend::ArrowBackend;