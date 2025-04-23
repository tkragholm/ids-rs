//! Adaptive memory management system for efficient resource utilization.
//!
//! This module provides a dynamic memory management system that adapts cache sizes,
//! batch sizes, and parallelism based on system resources. It automatically detects
//! available memory and adjusts settings to maximize performance without exhausting
//! system resources.

use log::{debug, info};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

/// Memory usage tier for adaptive memory management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTier {
    /// Low memory tier (< 8GB available)
    Low,
    /// Medium memory tier (8-16GB available)
    Medium,
    /// High memory tier (16-32GB available)
    High,
    /// Very high memory tier (> 32GB available)
    VeryHigh,
}

impl MemoryTier {
    /// Get the cache size scaling factor for this tier
    #[inline]
    fn cache_size_factor(&self) -> f64 {
        match self {
            Self::Low => 0.5,      // Smaller caches for low memory
            Self::Medium => 1.0,   // Base size for medium memory
            Self::High => 2.0,     // Larger caches for high memory
            Self::VeryHigh => 4.0, // Much larger caches for very high memory
        }
    }

    /// Get the batch size scaling factor for this tier
    #[inline]
    fn batch_size_factor(&self) -> f64 {
        match self {
            Self::Low => 0.5,      // Smaller batches for low memory
            Self::Medium => 1.0,   // Base size for medium memory
            Self::High => 2.0,     // Larger batches for high memory
            Self::VeryHigh => 3.0, // Much larger batches for very high memory
        }
    }

    /// Get the parallelism scaling factor for this tier
    #[inline]
    fn parallelism_factor(&self) -> f64 {
        match self {
            Self::Low => 0.5,      // Less parallelism for low memory
            Self::Medium => 0.75,  // Moderate parallelism for medium memory
            Self::High => 1.0,     // Full CPU parallelism for high memory
            Self::VeryHigh => 1.5, // Potentially higher parallelism for I/O bound tasks
        }
    }
}

/// Dynamic memory manager for adaptive resource allocation
pub struct MemoryManager {
    /// Current memory usage tier, determined dynamically
    tier: MemoryTier,
    /// Base cache size, scaled by tier
    base_cache_size: usize,
    /// Currently tracked allocations
    allocations: Mutex<HashMap<String, usize>>,
    /// Total currently allocated memory
    allocated_memory: AtomicUsize,
    /// Maximum memory to allocate
    max_memory: usize,
    /// CPU count for parallelism calculations
    cpu_count: usize,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryManager {
    /// Create a new memory manager with dynamic resource detection
    pub fn new() -> Self {
        // Detect system memory
        let system_memory = Self::detect_system_memory();

        // Determine tier based on available memory
        let tier = if system_memory < 8 * 1024 * 1024 * 1024 {
            MemoryTier::Low
        } else if system_memory < 16 * 1024 * 1024 * 1024 {
            MemoryTier::Medium
        } else if system_memory < 32 * 1024 * 1024 * 1024 {
            MemoryTier::High
        } else {
            MemoryTier::VeryHigh
        };

        // Determine maximum memory to use (25% of system memory)
        let max_memory = (system_memory / 4).max(1024 * 1024 * 1024); // At least 1GB

        // Base cache size - will be scaled by tier
        let base_cache_size = 500_000;

        // Get CPU count
        let cpu_count = num_cpus::get();

        info!(
            "Memory manager initialized: {:?} tier, {:.2}GB max memory, {} CPUs",
            tier,
            max_memory as f64 / (1024.0 * 1024.0 * 1024.0),
            cpu_count
        );

        Self {
            tier,
            base_cache_size,
            allocations: Mutex::new(HashMap::new()),
            allocated_memory: AtomicUsize::new(0),
            max_memory,
            cpu_count,
        }
    }

    /// Detect available system memory
    fn detect_system_memory() -> usize {
        #[cfg(target_os = "linux")]
        {
            if let Ok(mem_info) = std::fs::read_to_string("/proc/meminfo") {
                if let Some(mem_line) = mem_info.lines().find(|line| line.starts_with("MemTotal:"))
                {
                    if let Some(mem_kb_str) = mem_line.split_whitespace().nth(1) {
                        if let Ok(mem_kb) = mem_kb_str.parse::<usize>() {
                            return mem_kb * 1024; // Convert KB to bytes
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("sysctl").arg("-n").arg("hw.memsize").output() {
                if let Ok(mem_str) = String::from_utf8(output.stdout) {
                    if let Ok(mem_bytes) = mem_str.trim().parse::<usize>() {
                        return mem_bytes;
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            use std::mem::{size_of, zeroed};
            use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

            unsafe {
                let mut mem_status: MEMORYSTATUSEX = zeroed();
                mem_status.dwLength = size_of::<MEMORYSTATUSEX>() as u32;

                if GlobalMemoryStatusEx(&mut mem_status) != 0 {
                    return mem_status.ullTotalPhys as usize;
                }
            }
        }

        // Default if we can't detect: assume 16GB
        16 * 1024 * 1024 * 1024
    }

    /// Get the current memory tier
    pub fn get_tier(&self) -> MemoryTier {
        self.tier
    }

    /// Get recommended cache size based on memory tier and current allocations
    pub fn get_recommended_cache_size(&self) -> usize {
        let base_size = self.base_cache_size;
        let tier_factor = self.tier.cache_size_factor();

        // Calculate scaled size
        let scaled_size = (base_size as f64 * tier_factor) as usize;

        // Adjust based on current memory pressure
        let current_allocated = self.allocated_memory.load(Ordering::Relaxed);
        let memory_used_ratio = current_allocated as f64 / self.max_memory as f64;

        // If we're using more than 80% of our allocation, reduce the cache size
        if memory_used_ratio > 0.8 {
            ((scaled_size as f64 * (1.0 - (memory_used_ratio - 0.8) * 5.0)) as usize)
                .max(base_size / 4) // Don't go below 1/4 of base size
        } else {
            scaled_size
        }
    }

    /// Get the maximum parallel tasks to use based on CPU count and memory tier
    pub fn get_max_parallel_tasks(&self) -> usize {
        let parallelism_factor = self.tier.parallelism_factor();
        ((self.cpu_count as f64 * parallelism_factor).round() as usize).max(1) // At least 1 thread
    }

    /// Get optimal batch size for prefetching based on memory tier
    pub fn get_prefetch_batch_size(&self) -> usize {
        let base_size = 10_000;
        let tier_factor = self.tier.batch_size_factor();
        (base_size as f64 * tier_factor) as usize
    }

    /// Get the optimal chunk size for processing based on total items and memory tier
    pub fn get_optimal_chunk_size(&self, total_items: usize) -> usize {
        // Base calculation: 1/10 of total items
        let base_chunk = (total_items / 10).clamp(1000, 10_000);

        // Scale by memory tier
        let tier_factor = self.tier.batch_size_factor();
        (base_chunk as f64 * tier_factor) as usize
    }

    /// Track a memory allocation, returning a guard that will automatically untrack on drop
    pub fn track_allocation(&self, id: &str, size: usize) -> MemoryGuard {
        let allocation_id = id.to_string();

        // Update allocation tracking
        {
            let mut allocations = self.allocations.lock().unwrap();
            allocations.insert(allocation_id.clone(), size);
        }

        // Update total allocated memory
        self.allocated_memory.fetch_add(size, Ordering::SeqCst);

        debug!("Tracked allocation: {} ({} bytes)", id, size);

        // Return a guard that will automatically untrack on drop
        MemoryGuard {
            manager: Arc::new(self as *const MemoryManager as usize),
            id: allocation_id,
            size,
            start_time: Instant::now(),
        }
    }

    /// Internal method used by MemoryGuard to untrack an allocation on drop
    fn untrack_allocation(&self, id: &str, size: usize) {
        // Update allocation tracking
        {
            let mut allocations = self.allocations.lock().unwrap();
            allocations.remove(id);
        }

        // Update total allocated memory
        self.allocated_memory.fetch_sub(size, Ordering::SeqCst);

        debug!("Untracked allocation: {} ({} bytes)", id, size);
    }
}

// Use a thread-safe singleton pattern for the memory manager
static MEMORY_MANAGER: OnceLock<MemoryManager> = OnceLock::new();

/// Get the global memory manager instance
pub fn memory_manager() -> &'static MemoryManager {
    MEMORY_MANAGER.get_or_init(MemoryManager::new)
}

/// RAII guard for tracking memory allocations
///
/// When this guard is dropped, the allocation is automatically untracked
/// and resources are released.
pub struct MemoryGuard {
    /// Raw pointer to the memory manager as a usize for thread safety
    manager: Arc<usize>,
    /// ID of this allocation for tracking
    id: String,
    /// Size of this allocation in bytes
    size: usize,
    /// Time when this allocation was created
    start_time: Instant,
}

impl MemoryGuard {
    /// Create a new memory guard
    pub fn new(id: &str, size: usize) -> Self {
        memory_manager().track_allocation(id, size)
    }

    /// Get the size of this allocation
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the elapsed time since this allocation was created
    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

impl Drop for MemoryGuard {
    fn drop(&mut self) {
        // This is using Arc<usize> where the usize value is actually the address of
        // the MemoryManager. This is a workaround for sending the MemoryManager pointer
        // across threads.
        let raw_ptr = Arc::as_ptr(&self.manager);
        let ptr_val = unsafe { *raw_ptr };
        let manager_ptr = ptr_val as *const MemoryManager;
        
        unsafe {
            (*manager_ptr).untrack_allocation(&self.id, self.size);
        }
    }
}
