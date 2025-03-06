use std::path::PathBuf;
use tempfile::TempDir;

use datagen::{GeneratorConfig, RegisterGenerator};

// Import test helpers
mod test_helpers;
use test_helpers::{setup, registry};

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
    
    // Test loading family data
    let family_data = registry::load_family(&base_path, None)?;
    
    // Verify data was loaded
    assert!(!family_data.is_empty(), "Family data should not be empty");
    
    // Verify first relation has valid data
    let first_relation = &family_data[0];
    assert!(!first_relation.child_pnr.is_empty(), "Child PNR should not be empty");
    
    Ok(())
}

#[test]
fn test_load_akm() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading AKM data
    let akm_path = PathBuf::from(&base_path).join("akm");
    let akm_data = registry::load_akm(akm_path.to_str().unwrap(), None)?;
    
    // Verify data was loaded and has the expected columns
    assert!(akm_data.column("pnr").is_some(), "AKM data should have a PNR column");
    assert!(akm_data.column("year").is_some(), "AKM data should have a year column");
    
    Ok(())
}

#[test]
fn test_load_bef() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading BEF data
    let bef_path = PathBuf::from(&base_path).join("bef");
    let bef_data = registry::load_bef(bef_path.to_str().unwrap(), None)?;
    
    // Verify data was loaded and has the expected columns
    assert!(bef_data.column("pnr").is_some(), "BEF data should have a PNR column");
    assert!(bef_data.column("birthyear").is_some(), "BEF data should have a birthyear column");
    
    Ok(())
}

#[test]
fn test_load_ind() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading IND data
    let ind_path = PathBuf::from(&base_path).join("ind");
    let ind_data = registry::load_ind(ind_path.to_str().unwrap(), None)?;
    
    // Verify data was loaded and has the expected columns
    assert!(ind_data.column("pnr").is_some(), "IND data should have a PNR column");
    
    Ok(())
}

#[test]
fn test_load_uddf() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Test loading UDDF data
    let uddf_path = PathBuf::from(&base_path).join("uddf");
    let uddf_data = registry::load_uddf(uddf_path.to_str().unwrap(), None)?;
    
    // Verify data was loaded and has the expected columns
    assert!(uddf_data.column("pnr").is_some(), "UDDF data should have a PNR column");
    assert!(uddf_data.column("audd").is_some(), "UDDF data should have an education type column");
    
    Ok(())
}

#[test]
fn test_load_with_filters() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Generate test data
    let temp_dir = generate_test_data()?;
    let base_path = temp_dir.path().to_str().unwrap().to_string();
    
    // Create a filter (include only records where PNR starts with '1')
    let filter = Some(vec!["pnr LIKE '1%'".to_string()]);
    
    // Test loading data with filter
    let bef_path = PathBuf::from(&base_path).join("bef");
    let bef_data = registry::load_bef(bef_path.to_str().unwrap(), filter)?;
    
    // Verify filtered data - all PNRs should start with '1'
    let pnr_column = bef_data.column("pnr").unwrap();
    let pnr_array = pnr_column.as_string::<i32>().unwrap();
    
    for i in 0..pnr_array.len() {
        let pnr = pnr_array.value(i);
        assert!(pnr.starts_with('1'), "PNR should start with '1' after filtering");
    }
    
    Ok(())
}