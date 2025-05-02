use crate::error::{config_error, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Generic config loading function for any type that implements Deserialize
pub fn load_config<T>(path: impl AsRef<Path>) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(path.as_ref())
        .with_context(|| format!("Failed to open config file: {:?}", path.as_ref()))?;

    let reader = BufReader::new(file);

    if path.as_ref().extension().is_some_and(|ext| ext == "json") {
        serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse JSON config: {:?}", path.as_ref()))
    } else {
        // Default to JSON, but you can add support for other formats like YAML or TOML
        Err(config_error(format!(
            "Unsupported config format: {:?}",
            path.as_ref().extension().unwrap_or_default()
        )))
    }
}

/// Save config to a file in the specified format
pub fn save_config<T>(config: &T, path: impl AsRef<Path>) -> Result<()>
where
    T: Serialize,
{
    let file = File::create(path.as_ref())
        .with_context(|| format!("Failed to create config file: {:?}", path.as_ref()))?;

    if path.as_ref().extension().is_some_and(|ext| ext == "json") {
        serde_json::to_writer_pretty(file, config)
            .with_context(|| format!("Failed to write JSON config: {:?}", path.as_ref()))
    } else {
        // Default to JSON, but you can add support for other formats
        Err(config_error(format!(
            "Unsupported config format: {:?}",
            path.as_ref().extension().unwrap_or_default()
        )))
    }
}

/// Create default config file if it doesn't exist
pub fn create_default_config<T>(path: impl AsRef<Path>, default_config: T) -> Result<()>
where
    T: Serialize,
{
    if path.as_ref().exists() {
        Ok(())
    } else {
        save_config(&default_config, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestConfig {
        name: String,
        value: i32,
    }

    #[test]
    fn test_load_and_save_config() {
        // Create a temp file with a .json extension
        let temp_file_path = NamedTempFile::new().unwrap().path().with_extension("json");
        let mut temp_file = std::fs::File::create(&temp_file_path).unwrap();

        let test_config = TestConfig {
            name: "test".to_string(),
            value: 42,
        };

        // Write initial JSON to the temp file
        write!(
            temp_file,
            "{}",
            serde_json::to_string_pretty(&test_config).unwrap()
        )
        .unwrap();

        // Test loading config
        let loaded_config: TestConfig = load_config(&temp_file_path).unwrap();
        assert_eq!(loaded_config, test_config);

        // Test saving config
        let modified_config = TestConfig {
            name: "modified".to_string(),
            value: 99,
        };
        save_config(&modified_config, &temp_file_path).unwrap();

        // Verify save worked by loading again
        let reloaded_config: TestConfig = load_config(&temp_file_path).unwrap();
        assert_eq!(reloaded_config, modified_config);
    }

    #[test]
    fn test_create_default_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Delete the file so we can test creation
        std::fs::remove_file(&path).unwrap();

        let default_config = TestConfig {
            name: "default".to_string(),
            value: 0,
        };

        // Create default config - add file extension to make it work with the detection
        let path_with_extension = path.with_extension("json");

        // Create default config
        create_default_config(&path_with_extension, default_config.clone()).unwrap();

        // Verify it was created correctly
        let loaded_config: TestConfig = load_config(&path_with_extension).unwrap();
        assert_eq!(loaded_config, default_config);
    }
}
