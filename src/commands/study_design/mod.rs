//! Study design module for running the full pipeline
//!
//! This module provides a command that combines population generation,
//! SCD identification, sampling/matching, and covariate balance checking.

pub mod config;
pub mod handler;

pub use config::StudyDesignCommandConfig;
pub use handler::handle_study_design_command;
