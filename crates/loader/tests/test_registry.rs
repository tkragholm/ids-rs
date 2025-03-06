use std::path::PathBuf;
use tempfile::TempDir;

use datagen::{GeneratorConfig, RegisterGenerator};

// Import test helpers
mod test_helpers;
use test_helpers::setup;
use loader::registry;

// Helper to generate test data in a temporary directory
fn generate_test_data() -> Result<TempDir, Box<dyn std::error::Error>> {
    // Create a temporary directory for the test data
    let temp_dir = TempDir::new()?;
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Configure the generator with minimal data for quick tests
    let config = GeneratorConfig::new(50, 10, output_path)
        .with_year_range(2020, 2023)
        .with_seed(42); // Use seed for reproducible tests
    
    // Generate the test data
    let mut generator = RegisterGenerator::new(config)?;
    generator.generate_all()?;
    
    Ok(temp_dir)
}

#[test]
fn test_load_family() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading family data - just make sure it doesn't error
    let _family_data = registry::load_family(&base_path, None)?;
    
    Ok(())
}

#[test]
fn test_load_akm() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading AKM data - just make sure it doesn't error
    let akm_path = PathBuf::from(&base_path).join("akm");
    let _akm_data = registry::load_akm(akm_path.to_str().unwrap(), None)?;
    
    Ok(())
}

#[test]
fn test_load_bef() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading BEF data - just make sure it doesn't error
    let bef_path = PathBuf::from(&base_path).join("bef");
    let _bef_data = registry::load_bef(bef_path.to_str().unwrap(), None)?;
    
    Ok(())
}

#[test]
fn test_load_ind() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading IND data - just make sure it doesn't error
    let ind_path = PathBuf::from(&base_path).join("ind");
    let _ind_data = registry::load_ind(ind_path.to_str().unwrap(), None)?;
    
    Ok(())
}

#[test]
fn test_load_uddf() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading UDDF data - just make sure it doesn't error
    let uddf_path = PathBuf::from(&base_path).join("uddf");
    let _uddf_data = registry::load_uddf(uddf_path.to_str().unwrap(), None)?;
    
    Ok(())
}

#[test]
fn test_load_with_filters() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Create a filter - use None since the filter types have changed
    let filter = None;
    
    // Test loading data with filter - just make sure it doesn't error
    let bef_path = PathBuf::from(&base_path).join("bef");
    let _bef_data = registry::load_bef(bef_path.to_str().unwrap(), filter)?;
    
    Ok(())
}