//! Registry schema definitions
//!
//! This module contains schema definitions for various Danish registry data sources.

pub mod akm;
pub mod bef;
pub mod dod;
pub mod dodsaarsag;
pub mod idan;
pub mod ind;
pub mod lpr;
pub mod mfr;
pub mod uddf;
pub mod vnds;

// Re-export registry schemas
pub use akm::AkmSchema;
pub use bef::BefSchema;
pub use dod::{DodSchema, DodStandardizedSchema};
pub use dodsaarsag::{DodsaarsagSchema, DodsaarsagStandardizedSchema};
pub use idan::IdanSchema;
pub use ind::IndSchema;
pub use lpr::lpr2::Lpr2Schema;
pub use lpr::lpr3::Lpr3Schema;
pub use lpr::{LprSchema, LprVersion};
pub use mfr::MfrSchema;
pub use uddf::UddfSchema;
pub use vnds::{VndsSchema, VndsStandardizedSchema, MigrationType};