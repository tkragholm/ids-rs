pub mod arrow;
pub mod backends;

// Re-exports
// We can't directly use data_store since it's private, will fix in Phase 2
// pub use crate::store::data_store::DataStore;

// Legacy re-exports
pub use crate::store::arrow_backend::ArrowBackend as OldArrowBackend;
pub use crate::store::time_varying_backend::TimeVaryingBackend;

// Future imports that will replace the legacy ones
pub use arrow::backend::ArrowBackend;