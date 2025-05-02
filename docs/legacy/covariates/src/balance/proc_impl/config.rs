use super::super::memory::memory_manager;
use super::OptimizationStrategy;
use log;

/// Processor configuration to control performance characteristics
#[derive(Clone)]
pub struct ProcessorConfig {
    pub thread_count: usize,
    pub chunk_size_multiplier: usize,
    pub optimization_strategy: OptimizationStrategy,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        // Use the memory manager to determine optimal settings
        let mem_manager = memory_manager();

        Self {
            thread_count: mem_manager.get_max_parallel_tasks(),
            chunk_size_multiplier: 1,
            optimization_strategy: OptimizationStrategy::from_memory_tier(mem_manager.get_tier()),
        }
    }
}

impl ProcessorConfig {
    /// Create a new `ProcessorConfig` with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `ProcessorConfig` with custom configuration
    #[allow(dead_code)]
    pub fn with_config(
        thread_count: Option<usize>,
        chunk_size_multiplier: Option<usize>,
        optimization_strategy: Option<OptimizationStrategy>,
    ) -> Self {
        let mem_manager = memory_manager();

        Self {
            thread_count: thread_count.unwrap_or_else(|| mem_manager.get_max_parallel_tasks()),
            chunk_size_multiplier: chunk_size_multiplier.unwrap_or(1),
            optimization_strategy: optimization_strategy
                .unwrap_or_else(|| OptimizationStrategy::from_memory_tier(mem_manager.get_tier())),
        }
    }

    /// Configure optimization strategy
    #[allow(dead_code)]
    pub fn with_optimization_strategy(mut self, strategy: OptimizationStrategy) -> Self {
        self.optimization_strategy = strategy;
        self
    }

    /// Automatically select optimization strategy based on system resources
    #[allow(dead_code)]
    pub fn auto_configure(mut self) -> Self {
        // Get memory manager for system resources
        let mem_manager = memory_manager();

        // Select strategy based on available memory
        self.optimization_strategy = OptimizationStrategy::from_memory_tier(mem_manager.get_tier());

        log::info!(
            "Auto-configured optimization strategy: {:?} (detected memory tier: {:?})",
            self.optimization_strategy,
            mem_manager.get_tier()
        );

        self
    }

    /// Get the optimal chunk size based on workload and system capabilities
    pub fn get_optimal_chunk_size(&self, total_items: usize) -> usize {
        // Use memory manager for base chunk size determination
        let mem_manager = memory_manager();
        let base_chunk_size = mem_manager.get_optimal_chunk_size(total_items);

        // Apply the multiplier to allow tuning
        let chunk_size = base_chunk_size * self.chunk_size_multiplier;

        // Round to nearest hundred for cleaner numbers
        (chunk_size / 100) * 100
    }
}
