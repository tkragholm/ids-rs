//! SCD (Severe Chronic Disease) command implementation
//!
//! This module provides the implementation for the SCD command, which
//! analyzes health data to identify patients with severe chronic diseases.

pub mod config;
pub mod handler;

pub use config::ScdCommandConfig;
pub use handler::handle_scd_command;