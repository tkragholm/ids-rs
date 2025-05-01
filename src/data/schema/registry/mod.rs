//! Registry schema definitions
//!
//! This module contains schema definitions for various Danish registry data sources.

pub mod akm;
pub mod lpr;

// Re-export registry schemas
pub use akm::AkmSchema;
pub use lpr::lpr2::Lpr2Schema;
pub use lpr::lpr3::Lpr3Schema;
pub use lpr::{LprSchema, LprVersion};