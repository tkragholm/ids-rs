mod parquet;
mod parallel;
mod polars;

pub use parquet::ParquetLoader;
pub use parallel::ParallelLoader;
pub use polars::PolarsLoader;