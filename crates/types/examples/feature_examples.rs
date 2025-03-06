//! Examples demonstrating the use of feature flags and the new API.
//!
//! This example shows how to use the types crate with different feature flags enabled.
//! To run with specific features, use:
//!
//! ```bash
//! cargo run --example feature_examples --features arrow-integration,serde-support,logging
//! ```

use types::prelude::*;
use types::error::Result;

fn main() -> Result<()> {
    println!("Types crate feature examples\n");

    // Core functionality (always available)
    demonstrate_core_features();

    // Arrow integration (feature-gated)
    #[cfg(feature = "arrow-integration")]
    demonstrate_arrow_integration()?;

    // Serde support (feature-gated)
    #[cfg(feature = "serde-support")]
    demonstrate_serde_support()?;

    // Logging (feature-gated)
    #[cfg(feature = "logging")]
    demonstrate_logging()?;

    println!("\nAll examples completed successfully!");
    Ok(())
}

/// Demonstrates core features that are always available
fn demonstrate_core_features() {
    println!("=== Core Features ===");

    // Create and work with covariates
    let education = EducationBuilder::new("higher")
        .with_years(16.0)
        .build();

    let demographics = DemographicsBuilder::new(2, 101, "nuclear")
        .with_age(42)
        .with_gender("M")
        .build();

    // Combine education and demographics into separate covariates
    let education_covariate = Covariate::education("higher")
        .with_years(16.0)
        .build();
    
    let demographics_covariate = Covariate::demographics(2, 101, "nuclear")
        .with_age(42)
        .with_gender("M")
        .build();

    // Use match to extract the education level
    let level = match &education_covariate.value {
        CovariateValue::Education { level, .. } => level,
        _ => "Unknown"
    };
    
    println!("Created education covariate with level: {:?}", level);
    
    // Use the DataStore
    let store = DataStore::new_arrow().expect("Failed to create arrow data store");
    println!("Created empty arrow data store");

    println!("Core features demonstration complete\n");
}

/// Demonstrates Arrow integration (requires 'arrow-integration' feature)
#[cfg(feature = "arrow-integration")]
fn demonstrate_arrow_integration() -> Result<()> {
    use types::storage::arrow::backend::ArrowBackend;
    
    println!("=== Arrow Integration ===");
    
    // Create an Arrow backend
    let backend = ArrowBackend::new_empty();
    println!("Created Arrow backend");
    
    // Access data (will be empty since it's a new backend)
    let years = Vec::<i32>::new();  // Empty for a new backend
    let fields = Vec::<String>::new();  // Empty for a new backend
    
    println!("Backend has {} years and {} fields", years.len(), fields.len());
    
    println!("Arrow integration demonstration complete\n");
    Ok(())
}

/// Demonstrates Serde support (requires 'serde-support' feature)
#[cfg(feature = "serde-support")]
fn demonstrate_serde_support() -> Result<()> {
    use serde_json;
    
    println!("=== Serde Support ===");
    
    // Create a covariate
    let education = EducationBuilder::new("bachelor")
        .with_years(16.0)
        .build();
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&education)?;
    println!("Serialized education covariate to JSON:\n{}", json);
    
    // Deserialization is more complex and would need adapters
    // for the full structure that we'll skip for this example
    println!("(Skipping deserialization for simplicity)");
    
    println!("Serde support demonstration complete\n");
    Ok(())
}

/// Demonstrates logging functionality (requires 'logging' feature)
#[cfg(feature = "logging")]
fn demonstrate_logging() -> Result<()> {
    use types::utils::logging;
    
    println!("=== Logging ===");
    
    // Initialize the logger
    logging::init_logger()?;
    println!("Logger initialized");
    
    // Use logging macros
    log_info!("This is an info message");
    log_debug!("This is a debug message (may not be visible at default log level)");
    log_warn!("This is a warning message");
    
    println!("Logging demonstration complete\n");
    Ok(())
}