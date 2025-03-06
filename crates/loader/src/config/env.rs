use std::env;

/// Get batch size from environment or use default
///
/// This function reads the IDS_BATCH_SIZE environment variable,
/// defaulting to 65536 (64K rows) if not defined.
#[must_use]
pub fn get_batch_size() -> usize {
    env::var("IDS_BATCH_SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(65536) // 64K rows default for better performance
}

/// Get max threads from environment or use system CPU count
///
/// This function reads the IDS_MAX_THREADS environment variable,
/// defaulting to the number of CPU cores if not defined.
#[must_use]
pub fn get_max_threads() -> usize {
    env::var("IDS_MAX_THREADS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(num_cpus::get)
        .max(2) // At least 2 threads for parallelism
}

/// Check if family filtering should be enabled
///
/// This function reads the IDS_USE_FAMILY_FILTERING environment variable,
/// defaulting to false if not defined.
#[must_use]
pub fn should_use_family_filtering() -> bool {
    env::var("IDS_USE_FAMILY_FILTERING")
        .ok()
        .map(|s| s.to_lowercase() == "true" || s == "1")
        .unwrap_or(false)
}

/// Check if a specific register should use parallel loading
///
/// # Arguments
/// * `register_name` - The name of the register to check
///
/// # Returns
/// `true` if parallel loading is enabled for this register
#[must_use]
pub fn use_parallel_loading(register_name: &str) -> bool {
    let env_var = format!("IDS_PARALLEL_{}", register_name.to_uppercase());
    env::var(env_var)
        .ok()
        .map(|s| s.to_lowercase() == "true" || s == "1")
        .unwrap_or(true) // Default to true for all registers
}