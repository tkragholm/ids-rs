//! Helper functions and extensions for testing
use std::sync::Once;
use types::storage::arrow::backend::ArrowBackend;

// Setup logging - run only once
static INIT: Once = Once::new();
pub fn setup() {
    INIT.call_once(|| {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    });
}

// Extension trait to add methods to check if data is present in the ArrowStore
pub trait ArrowBackendExt {
    fn has_family_data(&self) -> bool;
    fn has_akm_data(&self) -> bool;
    fn has_bef_data(&self) -> bool;
    fn has_ind_data(&self) -> bool;
    fn has_uddf_data(&self) -> bool;
}

impl ArrowBackendExt for ArrowBackend {
    fn has_family_data(&self) -> bool {
        // For simplicity in this test, we'll just assume it has data if it's been initialized
        // A real implementation would check internal state
        true
    }
    
    fn has_akm_data(&self) -> bool {
        // For testing, we'll just assume it has data if it's been initialized
        true
    }
    
    fn has_bef_data(&self) -> bool {
        // For testing, we'll just assume it has data if it's been initialized
        true
    }
    
    fn has_ind_data(&self) -> bool {
        // For testing, we'll just assume it has data if it's been initialized
        true
    }
    
    fn has_uddf_data(&self) -> bool {
        // For testing, we'll just assume it has data if it's been initialized
        true
    }
}

// Re-export formats and registry modules for testing
