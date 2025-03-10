mod config;
mod engine;
mod error;
mod filter;
mod loader;
mod optimize;
mod parallel;
mod result;
mod utils;

// Public API exports
pub use config::{ColumnSelection, ParquetConfig, ReaderMode};
pub use engine::ParquetReader;
pub use error::ParquetIntegrationError;
pub use filter::FilterEngine;
pub use loader::{read_parquet_file, load_directory, load_with_schema};
pub use parallel::ParallelParquetLoader;
pub use result::{ReadResult, ReadStatistics};

/// Version of the parquet-integration crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Creates an optimized ParquetReader with good defaults
pub fn create_optimized_loader() -> ParquetReader {
    ParquetReader::new()
        .with_batch_size(131072) // 128K batch size
        .with_mode(ReaderMode::Adaptive)
}