use tempfile::TempDir;

use datagen::{GeneratorConfig, RegisterGenerator};
use loader::{ParallelLoader, SequentialLoader, StoreLoader, RegisterPathConfig};

// Import test helpers
mod test_helpers;
use test_helpers::setup;

// Helper to generate test data in a temporary directory
fn generate_test_data() -> Result<TempDir, Box<dyn std::error::Error>> {
    // Create a temporary directory for the test data
    let temp_dir = TempDir::new()?;
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Configure the generator with minimal data for quick tests
    let config = GeneratorConfig::new(100, 25, output_path)
        .with_year_range(2020, 2023)
        .with_seed(42); // Use seed for reproducible tests
    
    // Generate the test data
    let mut generator = RegisterGenerator::new(config)?;
    generator.generate_all()?;
    
    Ok(temp_dir)
}

#[test]
fn test_sequential_loader() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test sequential loader
    let loader = SequentialLoader::new();
    let _store = loader.load_from_path(base_path)?;
    
    // Store was created successfully
    assert!(true);
    
    Ok(())
}

#[test]
fn test_parallel_loader() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test parallel loader
    let loader = ParallelLoader::new();
    let _store = loader.load_from_path(base_path)?;
    
    // Store was created successfully
    assert!(true);
    
    Ok(())
}

#[test]
fn test_custom_paths_loader() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path();
    let base_path_str = base_path.to_str().unwrap().to_string();
    
    // Create a custom path configuration
    let config = RegisterPathConfig::new(base_path_str);
    
    // Register specific paths
    let config = config.with_custom_path("akm", base_path.join("akm").to_str().unwrap())
        .with_custom_path("bef", base_path.join("bef").to_str().unwrap())
        .with_custom_path("ind", base_path.join("ind").to_str().unwrap())
        .with_custom_path("uddf", base_path.join("uddf").to_str().unwrap());
    
    // Test both loaders with custom paths
    let sequential = SequentialLoader::new();
    let parallel = ParallelLoader::new();
    
    let _store_seq = sequential.load_with_custom_paths(config.clone())?;
    let _store_par = parallel.load_with_custom_paths(config)?;
    
    // Verify stores were created successfully
    assert!(true);
    
    Ok(())
}

#[test]
fn test_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Create a non-existent path
    let non_existent_path = "/tmp/non_existent_directory_for_test";
    
    // Test both loaders with non-existent paths
    let sequential = SequentialLoader::new();
    let parallel = ParallelLoader::new();
    
    // Both should handle errors gracefully
    let seq_result = sequential.load_from_path(non_existent_path.to_string());
    let par_result = parallel.load_from_path(non_existent_path.to_string());
    
    // We expect them to still return stores
    match seq_result {
        Ok(_) => {
            // Successfully created an empty store
        }
        Err(e) => {
            panic!("Expected SequentialLoader to handle missing directory gracefully, got error: {}", e);
        }
    }
    
    match par_result {
        Ok(_) => {
            // Successfully created an empty store
        }
        Err(e) => {
            panic!("Expected ParallelLoader to handle missing directory gracefully, got error: {}", e);
        }
    }
    
    Ok(())
}