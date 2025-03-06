mod normalize;
mod resolver;

// Re-export the public API
pub use normalize::normalize_path;
pub use resolver::{check_path_exists, resolve_path};