//! Registry loader implementations
//!
//! This module contains registry loader implementations for various Danish registry data sources.

pub mod akm;
pub mod lpr;

// Re-export registry loaders
pub use akm::AkmRegister;