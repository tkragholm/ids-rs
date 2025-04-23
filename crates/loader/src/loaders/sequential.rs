use chrono::Datelike;
use types::error::IdsError;
use types::storage::arrow::backend::ArrowBackend as ArrowStore;

use crate::config::RegisterPathConfig;
use crate::loaders::StoreLoader;
use crate::registry;
use crate::ui::LoaderProgress;

/// Sequential Register Loader implementation
///
/// This loader loads registers sequentially (one at a time), which is useful for:
/// 1. Debugging - simpler execution flow makes issues easier to find
/// 2. Reduced memory usage - loads one register at a time
/// 3. Systems with limited resources - uses minimal threading
pub struct SequentialLoader;

impl Default for SequentialLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl SequentialLoader {
    /// Create a new `SequentialLoader` instance
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl StoreLoader for SequentialLoader {
    fn load_from_path(&self, base_path: String) -> Result<ArrowStore, IdsError> {
        log::info!("Loading register data sequentially from {base_path}");

        // Create a progress tracker
        let progress = LoaderProgress::new();
        progress.set_main_message("Initializing data store");

        // Create an empty store
        let mut store = ArrowStore::new()?;

        // Load family relations first (if they exist)
        progress.set_main_message("Loading family relations");
        if let Ok(families) = registry::load_family(&base_path, None) {
            if let Err(e) = store.add_family_data(families) {
                log::error!("Failed to add family data: {e}");
            }
            progress.inc_main();
        }

        // Load AKM data
        progress.set_main_message("Loading annual register (AKM) data");
        if let Ok(akm_data) = registry::load_akm(&base_path, None) {
            // Default AKM year to current year
            let current_year = chrono::Local::now().year();
            if let Err(e) = store.add_akm_data(current_year, akm_data) {
                log::error!("Failed to add AKM data: {e}");
            }
            progress.inc_main();
        }

        // Load BEF data
        progress.set_main_message("Loading population register (BEF) data");
        if let Ok(bef_data) = registry::load_bef(&base_path, None) {
            // Default BEF period to "current" if missing
            if let Err(e) = store.add_bef_data("current".to_string(), bef_data) {
                log::error!("Failed to add BEF data: {e}");
            }
            progress.inc_main();
        }

        // Load IND data
        progress.set_main_message("Loading individual register (IND) data");
        if let Ok(ind_data) = registry::load_ind(&base_path, None) {
            // Default IND year to current year
            let current_year = chrono::Local::now().year();
            if let Err(e) = store.add_ind_data(current_year, ind_data) {
                log::error!("Failed to add IND data: {e}");
            }
            progress.inc_main();
        }

        // Load UDDF data
        progress.set_main_message("Loading education register (UDDF) data");
        if let Ok(uddf_data) = registry::load_uddf(&base_path, None) {
            // Default UDDF period to "current" if missing
            if let Err(e) = store.add_uddf_data("current".to_string(), uddf_data) {
                log::error!("Failed to add UDDF data: {e}");
            }
            progress.inc_main();
        }

        progress.finish_main();
        Ok(store)
    }

    fn load_with_custom_paths(&self, config: RegisterPathConfig) -> Result<ArrowStore, IdsError> {
        log::info!("Loading register data sequentially with custom paths");

        // Validate the config paths
        config.validate()?;

        // Resolve the paths
        let paths = config.resolve_paths()?;

        // Create a progress tracker
        let progress = LoaderProgress::new();
        progress.set_main_message("Initializing data store");

        // Create an empty store
        let mut store = ArrowStore::new()?;

        // Load family relations first (if configured)
        if let Some(family_path) = paths.get("family") {
            progress.set_main_message("Loading family relations");
            if let Ok(families) =
                registry::load_family(family_path.to_str().unwrap_or_default(), None)
            {
                if let Err(e) = store.add_family_data(families) {
                    log::error!("Failed to add family data: {e}");
                }
                progress.inc_main();
            }
        }

        // Load AKM data
        if let Some(akm_path) = paths.get("akm") {
            progress.set_main_message("Loading annual register (AKM) data");
            if let Ok(akm_data) = registry::load_akm(akm_path.to_str().unwrap_or_default(), None) {
                // Default AKM year to current year
                let current_year = chrono::Local::now().year();
                if let Err(e) = store.add_akm_data(current_year, akm_data) {
                    log::error!("Failed to add AKM data: {e}");
                }
                progress.inc_main();
            }
        }

        // Load BEF data
        if let Some(bef_path) = paths.get("bef") {
            progress.set_main_message("Loading population register (BEF) data");
            if let Ok(bef_data) = registry::load_bef(bef_path.to_str().unwrap_or_default(), None) {
                // Default BEF period to "current" if missing
                if let Err(e) = store.add_bef_data("current".to_string(), bef_data) {
                    log::error!("Failed to add BEF data: {e}");
                }
                progress.inc_main();
            }
        }

        // Load IND data
        if let Some(ind_path) = paths.get("ind") {
            progress.set_main_message("Loading individual register (IND) data");
            if let Ok(ind_data) = registry::load_ind(ind_path.to_str().unwrap_or_default(), None) {
                // Default IND year to current year
                let current_year = chrono::Local::now().year();
                if let Err(e) = store.add_ind_data(current_year, ind_data) {
                    log::error!("Failed to add IND data: {e}");
                }
                progress.inc_main();
            }
        }

        // Load UDDF data
        if let Some(uddf_path) = paths.get("uddf") {
            progress.set_main_message("Loading education register (UDDF) data");
            if let Ok(uddf_data) = registry::load_uddf(uddf_path.to_str().unwrap_or_default(), None)
            {
                // Default UDDF period to "current" if missing
                if let Err(e) = store.add_uddf_data("current".to_string(), uddf_data) {
                    log::error!("Failed to add UDDF data: {e}");
                }
                progress.inc_main();
            }
        }

        progress.finish_main();
        Ok(store)
    }
}
