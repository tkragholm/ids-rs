use chrono::Datelike;
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use types::error::IdsError;
use types::storage::arrow::backend::ArrowBackend as ArrowStore;

use crate::config::RegisterPathConfig;
use crate::registry;
use crate::ui::LoaderProgress;

pub trait ParallelLoader {
    fn load_from_path(base_path: &str, parallel: bool) -> Result<ArrowStore, IdsError>;
    fn load_with_custom_paths(config: RegisterPathConfig) -> Result<ArrowStore, IdsError>;
}

pub struct ParallelLoaderImpl;

impl ParallelLoader for ParallelLoaderImpl {
    fn load_from_path(base_path: &str, parallel: bool) -> Result<ArrowStore, IdsError> {
        log::info!("Loading register data from path: {}", base_path);

        // Create a progress tracker
        let progress = LoaderProgress::new();
        progress.set_main_message("Initializing data loading");

        // Determine which loaders to run in parallel
        let load_akm_parallel = parallel;
        let load_bef_parallel = parallel;
        let load_ind_parallel = parallel;
        let load_uddf_parallel = parallel;

        // Create a shared store to collect all data
        let store_result = ArrowStore::new();
        let store = Arc::new(Mutex::new(match store_result {
            Ok(store) => store,
            Err(e) => {
                log::error!("Failed to create ArrowBackend: {}", e);
                return Err(e);
            }
        }));

        // Create a channel for collecting results
        let (sender, receiver) = mpsc::channel();

        // Create thread handles
        let mut handles = Vec::new();

        // AKM data
        if load_akm_parallel {
            let base_path_clone = base_path.to_string();
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
                match registry::load_akm(&base_path_clone, None) {
                    Ok(data) => {
                        let _ = sender_clone.send(("akm", Ok(data)));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(("akm", Err(e)));
                    }
                }
            }));
        } else {
            // Load sequentially
            progress.set_main_message("Loading AKM data");
            if let Ok(akm_data) = registry::load_akm(base_path, None) {
                let mut store_guard = store.lock().unwrap();
                let current_year = chrono::Local::now().year();
                if let Err(e) = store_guard.add_akm_data(current_year, akm_data) {
                    log::error!("Failed to add AKM data: {}", e);
                }
                progress.inc_main();
            }
        }

        // BEF data
        if load_bef_parallel {
            let base_path_clone = base_path.to_string();
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
                match registry::load_bef(&base_path_clone, None) {
                    Ok(data) => {
                        let _ = sender_clone.send(("bef", Ok(data)));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(("bef", Err(e)));
                    }
                }
            }));
        } else {
            // Load sequentially
            progress.set_main_message("Loading BEF data");
            if let Ok(bef_data) = registry::load_bef(base_path, None) {
                let mut store_guard = store.lock().unwrap();
                if let Err(e) = store_guard.add_bef_data("current".to_string(), bef_data) {
                    log::error!("Failed to add BEF data: {}", e);
                }
                progress.inc_main();
            }
        }

        // IND data
        if load_ind_parallel {
            let base_path_clone = base_path.to_string();
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
                match registry::load_ind(&base_path_clone, None) {
                    Ok(data) => {
                        let _ = sender_clone.send(("ind", Ok(data)));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(("ind", Err(e)));
                    }
                }
            }));
        } else {
            // Load sequentially
            progress.set_main_message("Loading IND data");
            if let Ok(ind_data) = registry::load_ind(base_path, None) {
                let mut store_guard = store.lock().unwrap();
                let current_year = chrono::Local::now().year();
                if let Err(e) = store_guard.add_ind_data(current_year, ind_data) {
                    log::error!("Failed to add IND data: {}", e);
                }
                progress.inc_main();
            }
        }

        // UDDF data
        if load_uddf_parallel {
            let base_path_clone = base_path.to_string();
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
                match registry::load_uddf(&base_path_clone, None) {
                    Ok(data) => {
                        let _ = sender_clone.send(("uddf", Ok(data)));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(("uddf", Err(e)));
                    }
                }
            }));
        } else {
            // Load sequentially
            progress.set_main_message("Loading UDDF data");
            if let Ok(uddf_data) = registry::load_uddf(base_path, None) {
                let mut store_guard = store.lock().unwrap();
                if let Err(e) = store_guard.add_uddf_data("current".to_string(), uddf_data) {
                    log::error!("Failed to add UDDF data: {}", e);
                }
                progress.inc_main();
            }
        }

        // Close the sender to signal no more messages
        drop(sender);

        // Process results from parallel loading
        for (register_type, result) in receiver {
            match (register_type, result) {
                ("akm", Ok(data)) => {
                    progress.set_main_message("Adding AKM data to store");
                    let mut store_guard = store.lock().unwrap();
                    let current_year = chrono::Local::now().year();
                    if let Err(e) = store_guard.add_akm_data(current_year, data) {
                        log::error!("Failed to add AKM data: {}", e);
                    }
                    progress.inc_main();
                }
                ("bef", Ok(data)) => {
                    progress.set_main_message("Adding BEF data to store");
                    let mut store_guard = store.lock().unwrap();
                    if let Err(e) = store_guard.add_bef_data("current".to_string(), data) {
                        log::error!("Failed to add BEF data: {}", e);
                    }
                    progress.inc_main();
                }
                ("ind", Ok(data)) => {
                    progress.set_main_message("Adding IND data to store");
                    let mut store_guard = store.lock().unwrap();
                    let current_year = chrono::Local::now().year();
                    if let Err(e) = store_guard.add_ind_data(current_year, data) {
                        log::error!("Failed to add IND data: {}", e);
                    }
                    progress.inc_main();
                }
                ("uddf", Ok(data)) => {
                    progress.set_main_message("Adding UDDF data to store");
                    let mut store_guard = store.lock().unwrap();
                    if let Err(e) = store_guard.add_uddf_data("current".to_string(), data) {
                        log::error!("Failed to add UDDF data: {}", e);
                    }
                    progress.inc_main();
                }
                (register_type, Err(e)) => {
                    log::error!("Error loading {} data: {}", register_type, e);
                }
                (unknown_type, Ok(_)) => {
                    log::warn!("Received data for unknown register type: {}", unknown_type);
                }
            }
        }

        // Wait for all threads to finish
        for handle in handles {
            let _ = handle.join();
        }

        progress.finish_main();

        // Return the store
        match Arc::try_unwrap(store) {
            Ok(mutex) => Ok(mutex.into_inner().unwrap()),
            Err(_) => Err(IdsError::invalid_operation(
                "Failed to unwrap store from Arc".to_string(),
            )),
        }
    }

    fn load_with_custom_paths(config: RegisterPathConfig) -> Result<ArrowStore, IdsError> {
        log::info!("Loading register data in parallel with custom paths");

        // Validate the config paths
        config.validate()?;

        // Resolve the paths
        let paths = config.resolve_paths()?;

        // We'll add the catch-all pattern for both match statements to fix the non-exhaustive pattern error

        // Create a progress tracker
        let progress = LoaderProgress::new();
        progress.set_main_message("Initializing parallel data loading");

        // Determine which loaders to run in parallel
        let load_akm_parallel = true; // Use hardcoded values or derive from custom_paths
        let load_bef_parallel = true;
        let load_ind_parallel = true;
        let load_uddf_parallel = true;

        // Get the paths
        let akm_path = paths
            .get("akm")
            .cloned()
            .unwrap_or_else(|| PathBuf::from(&config.base_path));
        let bef_path = paths
            .get("bef")
            .cloned()
            .unwrap_or_else(|| PathBuf::from(&config.base_path));
        let ind_path = paths
            .get("ind")
            .cloned()
            .unwrap_or_else(|| PathBuf::from(&config.base_path));
        let uddf_path = paths
            .get("uddf")
            .cloned()
            .unwrap_or_else(|| PathBuf::from(&config.base_path));

        // Create a shared store to collect all data
        let store_result = ArrowStore::new();
        let store = Arc::new(Mutex::new(match store_result {
            Ok(store) => store,
            Err(e) => {
                log::error!("Failed to create ArrowBackend: {}", e);
                return Err(e);
            }
        }));

        // Create a channel for collecting results
        let (sender, receiver) = mpsc::channel();

        // Create thread handles
        let mut handles = Vec::new();

        // AKM data
        if load_akm_parallel {
            let akm_path = akm_path.clone();
            let akm_path_str = akm_path.to_str().unwrap_or_default().to_string();
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
                match registry::load_akm(&akm_path_str, None) {
                    Ok(data) => {
                        let _ = sender_clone.send(("akm", Ok(data)));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(("akm", Err(e)));
                    }
                }
            }));
        } else {
            // Load sequentially
            progress.set_main_message("Loading AKM data");
            if let Ok(akm_data) = registry::load_akm(akm_path.to_str().unwrap_or_default(), None) {
                let mut store_guard = store.lock().unwrap();
                let current_year = chrono::Local::now().year();
                if let Err(e) = store_guard.add_akm_data(current_year, akm_data) {
                    log::error!("Failed to add AKM data: {}", e);
                }
                progress.inc_main();
            }
        }

        // BEF data
        if load_bef_parallel {
            let bef_path = bef_path.clone();
            let bef_path_str = bef_path.to_str().unwrap_or_default().to_string();
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
                match registry::load_bef(&bef_path_str, None) {
                    Ok(data) => {
                        let _ = sender_clone.send(("bef", Ok(data)));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(("bef", Err(e)));
                    }
                }
            }));
        } else {
            // Load sequentially
            progress.set_main_message("Loading BEF data");
            if let Ok(bef_data) = registry::load_bef(bef_path.to_str().unwrap_or_default(), None) {
                let mut store_guard = store.lock().unwrap();
                if let Err(e) = store_guard.add_bef_data("current".to_string(), bef_data) {
                    log::error!("Failed to add BEF data: {}", e);
                }
                progress.inc_main();
            }
        }

        // IND data
        if load_ind_parallel {
            let ind_path = ind_path.clone();
            let ind_path_str = ind_path.to_str().unwrap_or_default().to_string();
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
                match registry::load_ind(&ind_path_str, None) {
                    Ok(data) => {
                        let _ = sender_clone.send(("ind", Ok(data)));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(("ind", Err(e)));
                    }
                }
            }));
        } else {
            // Load sequentially
            progress.set_main_message("Loading IND data");
            if let Ok(ind_data) = registry::load_ind(ind_path.to_str().unwrap_or_default(), None) {
                let mut store_guard = store.lock().unwrap();
                let current_year = chrono::Local::now().year();
                if let Err(e) = store_guard.add_ind_data(current_year, ind_data) {
                    log::error!("Failed to add IND data: {}", e);
                }
                progress.inc_main();
            }
        }

        // UDDF data
        if load_uddf_parallel {
            let uddf_path = uddf_path.clone();
            let uddf_path_str = uddf_path.to_str().unwrap_or_default().to_string();
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
                match registry::load_uddf(&uddf_path_str, None) {
                    Ok(data) => {
                        let _ = sender_clone.send(("uddf", Ok(data)));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(("uddf", Err(e)));
                    }
                }
            }));
        } else {
            // Load sequentially
            progress.set_main_message("Loading UDDF data");
            if let Ok(uddf_data) = registry::load_uddf(uddf_path.to_str().unwrap_or_default(), None)
            {
                let mut store_guard = store.lock().unwrap();
                if let Err(e) = store_guard.add_uddf_data("current".to_string(), uddf_data) {
                    log::error!("Failed to add UDDF data: {}", e);
                }
                progress.inc_main();
            }
        }

        // Close the sender to signal no more messages
        drop(sender);

        // Process results from parallel loading
        for (register_type, result) in receiver {
            match (register_type, result) {
                ("akm", Ok(data)) => {
                    progress.set_main_message("Adding AKM data to store");
                    let mut store_guard = store.lock().unwrap();
                    let current_year = chrono::Local::now().year();
                    if let Err(e) = store_guard.add_akm_data(current_year, data) {
                        log::error!("Failed to add AKM data: {}", e);
                    }
                    progress.inc_main();
                }
                ("bef", Ok(data)) => {
                    progress.set_main_message("Adding BEF data to store");
                    let mut store_guard = store.lock().unwrap();
                    if let Err(e) = store_guard.add_bef_data("current".to_string(), data) {
                        log::error!("Failed to add BEF data: {}", e);
                    }
                    progress.inc_main();
                }
                ("ind", Ok(data)) => {
                    progress.set_main_message("Adding IND data to store");
                    let mut store_guard = store.lock().unwrap();
                    let current_year = chrono::Local::now().year();
                    if let Err(e) = store_guard.add_ind_data(current_year, data) {
                        log::error!("Failed to add IND data: {}", e);
                    }
                    progress.inc_main();
                }
                ("uddf", Ok(data)) => {
                    progress.set_main_message("Adding UDDF data to store");
                    let mut store_guard = store.lock().unwrap();
                    if let Err(e) = store_guard.add_uddf_data("current".to_string(), data) {
                        log::error!("Failed to add UDDF data: {}", e);
                    }
                    progress.inc_main();
                }
                (register_type, Err(e)) => {
                    log::error!("Error loading {} data: {}", register_type, e);
                }
                (unknown_type, Ok(_)) => {
                    log::warn!("Received data for unknown register type: {}", unknown_type);
                }
                (_, _) => {
                    // This should never happen but is needed for exhaustive pattern matching
                    log::warn!("Unexpected result combination from register loading");
                }
            }
        }

        // Wait for all threads to finish
        for handle in handles {
            let _ = handle.join();
        }

        progress.finish_main();

        // Return the store
        match Arc::try_unwrap(store) {
            Ok(mutex) => Ok(mutex.into_inner().unwrap()),
            Err(_) => Err(IdsError::invalid_operation(
                "Failed to unwrap store from Arc".to_string(),
            )),
        }
    }
}
