pub mod paths;
pub mod reports;
pub mod setup;

// Re-export the most commonly used functionality
pub use paths::normalize_path;
pub use reports::generate_structured_reports;
pub use setup::directories::setup_directories;
pub use setup::logging::configure_logging_with_dir;
