//! Utility functions and types for the IDS-RS project.
//!
//! This crate provides various utility functions and types that are used
//! throughout the IDS-RS project, including:
//!
//! - Date utilities for working with dates, periods, and time ranges
//! - String utilities for string manipulation, parsing, and formatting
//! - PNR utilities for working with personal identification numbers
//! - File pattern utilities for finding and categorizing files
//! - Error handling utilities and types
//! - Logging utilities and configuration
//! - Rich console output and progress reporting
//!
//! Many of these utilities were previously scattered across different modules
//! and have been consolidated here for better organization and reuse.

// Core utilities
pub mod config;
pub mod error;
pub mod logging;
pub mod rich_console;

// Consolidated utilities
pub mod date_utils;
pub mod string_utils;
pub mod pnr_utils;
pub mod file_patterns;

// Older utilities (to be migrated)
pub mod date;
pub mod string;

// Re-export commonly used utilities for convenience
pub use crate::date_utils::core::{DateUtils, DateUtilsImpl, DateHelpers};
pub use crate::date_utils::periods::{DatePeriodUtils, DatePeriodUtilsImpl};
pub use crate::date_utils::parsing::{DateParsingUtils, DateParsingUtilsImpl};
pub use crate::date_utils::formatting::{DateFormattingUtils, DateFormattingUtilsImpl};

pub use crate::string_utils::case_conversion::{StringCaseUtils, StringCaseUtilsImpl};
pub use crate::string_utils::parsing::{StringParsingUtils, StringParsingUtilsImpl};
pub use crate::string_utils::formatting::{StringFormattingUtils, StringFormattingUtilsImpl};

pub use crate::pnr_utils::types::{PersonInfo, ParentPair, FamilyInfo, Gender};
pub use crate::pnr_utils::generation::{PnrPool, generate_pnr};
pub use crate::pnr_utils::validation::{PnrValidationUtils, PnrValidationUtilsImpl};