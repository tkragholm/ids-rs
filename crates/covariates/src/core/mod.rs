pub mod config;
pub mod registry;

pub use config::{
    generate_default_config, CovariateTypeConfig, CovariateVariableConfig, CovariatesConfig,
};
pub use registry::CovariateProcessorRegistry;

// Re-export common error types that we use
pub use types::error::IdsError as Error;
// Create a type alias for Result to use with our Error type
pub type Result<T> = std::result::Result<T, Error>;
