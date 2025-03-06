use chrono::Datelike;
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use types::error::IdsError;
use types::storage::arrow::backend::ArrowBackend as ArrowStore;

use crate::config::RegisterPathConfig;
use crate::loaders::StoreLoader;
use crate::registry;
use crate::ui::LoaderProgress;

/// Parallel Register Loader implementation
///
/// This loader loads registers in parallel, which is useful for:
/// 1. Performance - faster loading on multi-core systems
/// 2. Efficiency - takes advantage of multiple CPU cores
/// 3. Systems with adequate memory resources
pub struct ParallelLoader {
    /// Whether to load data in parallel
    parallel: bool,
}

impl Default for ParallelLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParallelLoader {
    /// Create a new ParallelLoader instance with parallel loading enabled
    #[must_use]
    pub const fn new() -> Self {
        Self { parallel: true }
    }
    
    /// Create a new ParallelLoader with specified parallel setting
    #[must_use]
    pub const fn with_parallel(parallel: bool) -> Self {
        Self { parallel }
    }
    
    /// Lock the shared store and handle mutex errors
    /// 
    /// # Arguments
    /// * `store` - The Arc<Mutex<ArrowStore>> to lock
    /// * `register_type` - The type of register being processed (for error messages)
    /// 
    /// # Returns
    /// A result containing the locked store guard or an error
    fn lock_store<'a>(
        &self,
        store: &'a Arc<Mutex<ArrowStore>>,
        register_type: &str,
    ) -> Result<std::sync::MutexGuard<'a, ArrowStore>, IdsError> {
        store.lock().map_err(|e| {
            IdsError::invalid_operation(format!(
                "Failed to lock store mutex for {} data: {}",
                register_type, e
            ))
        })
    }
    
    /// Handle receiver results with proper error handling
    /// 
    /// This consolidates common logic for handling register data from parallel processing
    /// 
    /// # Arguments
    /// * `store` - The shared store 
    /// * `register_type` - Type of register ("akm", "bef", etc.)
    /// * `data` - The loaded data
    /// * `progress` - Progress tracker
    /// * `add_fn` - Function to add data to store
    #[allow(dead_code)]
    fn handle_receiver_result<F>(
        &self,
        store: &Arc<Mutex<ArrowStore>>,
        register_type: &str,
        _data: Vec<arrow::record_batch::RecordBatch>, // Prefixed with underscore as it's passed to add_fn internally
        progress: &LoaderProgress,
        add_fn: F,
    ) where
        F: FnOnce(&mut ArrowStore) -> Result<(), IdsError>,
    {
        progress.set_main_message(&format!("Adding {} data to store", register_type.to_uppercase()));
        
        match self.lock_store(store, register_type) {
            Ok(mut store_guard) => {
                if let Err(e) = add_fn(&mut store_guard) {
                    log::error!("Failed to add {} data: {}", register_type.to_uppercase(), e);
                }
            },
            Err(e) => log::error!("{}", e),
        }
        
        progress.inc_main();
    }
    
    /// Safely unwrap store from Arc<Mutex<>> with comprehensive error handling
    ///
    /// # Arguments
    /// * `store` - The Arc<Mutex<ArrowStore>> to unwrap
    ///
    /// # Returns
    /// The unwrapped ArrowStore or an error with descriptive message
    #[allow(dead_code)]
    fn unwrap_store(store: Arc<Mutex<ArrowStore>>) -> Result<ArrowStore, IdsError> {
        // First try to unwrap from Arc
        match Arc::try_unwrap(store) {
            Ok(mutex) => {
                // Then try to get inner value from Mutex
                mutex.into_inner().map_err(|e| {
                    IdsError::invalid_operation(format!(
                        "Failed to unwrap store from mutex: {}",
                        e
                    ))
                })
            },
            Err(_) => Err(IdsError::invalid_operation(
                "Failed to unwrap store from Arc - still referenced by other threads".to_string(),
            )),
        }
    }
}

impl StoreLoader for ParallelLoader {
    fn load_from_path(&self, base_path: String) -> Result<ArrowStore, IdsError> {
        log::info!("Loading register data from path: {}", base_path);

        // Create a progress tracker
        let progress = LoaderProgress::new();
        progress.set_main_message("Initializing data loading");

        // Determine which loaders to run in parallel
        let load_akm_parallel = self.parallel;
        let load_bef_parallel = self.parallel;
        let load_ind_parallel = self.parallel;
        let load_uddf_parallel = self.parallel;

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
            let base_path_clone = base_path.clone();
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
            if let Ok(akm_data) = registry::load_akm(&base_path, None) {
                match self.lock_store(&store, "AKM") {
                    Ok(mut store_guard) => {
                        let current_year = chrono::Local::now().year();
                        if let Err(e) = store_guard.add_akm_data(current_year, akm_data) {
                            log::error!("Failed to add AKM data: {}", e);
                            // Continue with loading instead of returning error, to get as much data as possible
                        }
                    },
                    Err(e) => log::error!("{}", e),
                }
                progress.inc_main();
            }
        }

        // BEF data
        if load_bef_parallel {
            let base_path_clone = base_path.clone();
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
            if let Ok(bef_data) = registry::load_bef(&base_path, None) {
                match self.lock_store(&store, "BEF") {
                    Ok(mut store_guard) => {
                        if let Err(e) = store_guard.add_bef_data("current".to_string(), bef_data) {
                            log::error!("Failed to add BEF data: {}", e);
                            // Continue with loading instead of returning error, to get as much data as possible
                        }
                    },
                    Err(e) => log::error!("{}", e),
                }
                progress.inc_main();
            }
        }

        // IND data
        if load_ind_parallel {
            let base_path_clone = base_path.clone();
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
            if let Ok(ind_data) = registry::load_ind(&base_path, None) {
                match self.lock_store(&store, "IND") {
                    Ok(mut store_guard) => {
                        let current_year = chrono::Local::now().year();
                        if let Err(e) = store_guard.add_ind_data(current_year, ind_data) {
                            log::error!("Failed to add IND data: {}", e);
                            // Continue with loading instead of returning error, to get as much data as possible
                        }
                    },
                    Err(e) => log::error!("{}", e),
                }
                progress.inc_main();
            }
        }

        // UDDF data
        if load_uddf_parallel {
            let base_path_clone = base_path.clone();
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
            if let Ok(uddf_data) = registry::load_uddf(&base_path, None) {
                match self.lock_store(&store, "UDDF") {
                    Ok(mut store_guard) => {
                        if let Err(e) = store_guard.add_uddf_data("current".to_string(), uddf_data) {
                            log::error!("Failed to add UDDF data: {}", e);
                            // Continue with loading instead of returning error, to get as much data as possible
                        }
                    },
                    Err(e) => log::error!("{}", e),
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

    fn load_with_custom_paths(&self, config: RegisterPathConfig) -> Result<ArrowStore, IdsError> {
        log::info!("Loading register data in parallel with custom paths");

        // Validate the config paths
        config.validate()?;

        // Resolve the paths
        let paths = config.resolve_paths()?;

        // Create a progress tracker
        let progress = LoaderProgress::new();
        progress.set_main_message("Initializing parallel data loading");

        // Determine which loaders to run in parallel
        let load_akm_parallel = self.parallel;
        let load_bef_parallel = self.parallel;
        let load_ind_parallel = self.parallel;
        let load_uddf_parallel = self.parallel;

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
