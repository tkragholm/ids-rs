use super::memory::MemoryTier;

/// Controls how data is processed during balance checking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptimizationStrategy {
    /// Safe mode: no date grouping, linear processing only
    /// Best for systems with limited memory or when stability is critical
    #[default]
    Safe,

    /// Balanced mode: uses date grouping only for small datasets
    /// Good default for most systems
    Balanced,

    /// Performance mode: uses date grouping optimization extensively
    /// Best for high-memory systems (32+ GB) when speed is critical
    Performance,
}

impl OptimizationStrategy {
    /// Maps memory tier to optimization strategy
    #[must_use] pub fn from_memory_tier(tier: MemoryTier) -> Self {
        match tier {
            MemoryTier::VeryHigh => OptimizationStrategy::Performance,
            MemoryTier::High => OptimizationStrategy::Performance,
            MemoryTier::Medium => OptimizationStrategy::Balanced,
            MemoryTier::Low => OptimizationStrategy::Safe,
        }
    }
}
