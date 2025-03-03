mod checker;
pub mod legacy_cache;
mod memory;
mod metrics;
mod processor;
pub mod results;
mod stats;

pub use checker::BalanceChecker;
pub use memory::{memory_manager, MemoryGuard, MemoryTier};
pub use processor::OptimizationStrategy;
pub use results::{BalanceResults, MatchedPairSummary};
