//! Balance analysis module for covariate analysis in matched studies
//!
//! This module provides functionality for analyzing covariate balance between
//! case and control groups, calculating standardized mean differences, and
//! generating detailed reports.

// Core functionality
pub mod checker; // Main balance checker implementation
mod memory; // Memory management for large datasets
pub mod metrics; // Balance metrics calculations
mod optimization; // Optimization strategies for processing
mod proc_impl; // Implementation details for processor
mod processor; // Main value processor
pub mod results; // Results data structures
mod stats; // Statistical calculations

// Public exports
pub use checker::{BalanceChecker, BalanceCheckerBuilder};
pub use memory::{memory_manager, MemoryGuard, MemoryTier};
pub use optimization::OptimizationStrategy;
pub use processor::ValueProcessor;
pub use results::{BalanceResults, MatchedPairSummary};
