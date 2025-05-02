//! Registry loader implementations
//!
//! This module contains registry loader implementations for various Danish registry data sources.

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

// Re-export registry loaders
pub use akm::AkmRegister;
pub use bef::BefRegister;
pub use dod::DodRegister;
pub use dodsaarsag::DodsaarsagRegister;
pub use idan::IdanRegister;
pub use ind::IndRegister;
pub use lpr::{Lpr2Register, Lpr3Register, LprPaths, LprRegistry, LprVersion};
pub use mfr::MfrRegister;
pub use uddf::UddfRegister;
pub use vnds::VndsRegister;
