//! Helper functions and extensions for testing
use std::sync::Once;

// Setup logging - run only once
static INIT: Once = Once::new();
pub fn setup() {
    INIT.call_once(|| {
        env_logger::builder().filter_level(log::LevelFilter::Error).init();
    });
}

// Alternative setup with debug logging for when detailed logs are needed
pub fn setup_with_debug() {
    INIT.call_once(|| {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    });
}
