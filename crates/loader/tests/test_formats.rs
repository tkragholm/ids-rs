use std::io::Write;
use arrow::array::{Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use tempfile::{NamedTempFile, TempDir};

// Import test helpers
mod test_helpers;
use test_helpers::{setup, formats};

// Helper to create a test parquet file
fn create_test_parquet_file() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    // Create a temporary file
    let mut temp_file = NamedTempFile::new()?;
    
    // Define schema
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, false),
    ]);
    
    // Create data
    let id_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let name_array = StringArray::from(vec!["Alice", "Bob", "Charlie", "David", "Eve"]);
    
    // Create record batch
    let batch = RecordBatch::try_new(
        std::sync::Arc::new(schema),
        vec![
            std::sync::Arc::new(id_array),
            std::sync::Arc::new(name_array),
        ],
    )?;
    
    // Write to parquet
    let mut writer = ArrowWriter::try_new(temp_file.as_file_mut(), batch.schema(), None)?;
    writer.write(&batch)?;
    writer.close()?;
    
    Ok(temp_file)
}

// Helper to create multiple test parquet files
fn create_multiple_parquet_files(count: usize) -> Result<TempDir, Box<dyn std::error::Error>> {
    // Create a temporary directory
    let temp_dir = TempDir::new()?;
    
    for i in 0..count {
        // Define schema
        let schema = Schema::new(vec![
            Field::new("id", DataType::Int32, false),
            Field::new("name", DataType::Utf8, false),
        ]);
        
        // Create data with different values for each file
        let id_array = Int32Array::from(vec![i as i32 * 10 + 1, i as i32 * 10 + 2, i as i32 * 10 + 3]);
        let name_array = StringArray::from(vec![
            format!("Person_{}_1", i),
            format!("Person_{}_2", i),
            format!("Person_{}_3", i),
        ]);
        
        // Create record batch
        let batch = RecordBatch::try_new(
            std::sync::Arc::new(schema),
            vec![
                std::sync::Arc::new(id_array),
                std::sync::Arc::new(name_array),
            ],
        )?;
        
        // Create file in the temporary directory
        let file_path = temp_dir.path().join(format!("test_{}.parquet", i));
        let file = std::fs::File::create(&file_path)?;
        
        // Write to parquet
        let mut writer = ArrowWriter::try_new(file, batch.schema(), None)?;
        writer.write(&batch)?;
        writer.close()?;
    }
    
    Ok(temp_dir)
}

#[test]
fn test_read_parquet() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Create a test parquet file
    let temp_file = create_test_parquet_file()?;
    let file_path = temp_file.path().to_str().unwrap();
    
    // Read the parquet file
    let record_batch = formats::read_parquet(file_path)?;
    
    // Verify the data
    assert_eq!(record_batch.num_rows(), 5);
    assert_eq!(record_batch.num_columns(), 2);
    
    let id_column = record_batch.column(0);
    let name_column = record_batch.column(1);
    
    // Check the first value
    let id_array = id_column.as_primitive::<arrow::datatypes::Int32Type>();
    let name_array = name_column.as_string::<i32>().unwrap();
    
    assert_eq!(id_array.value(0), 1);
    assert_eq!(name_array.value(0), "Alice");
    
    Ok(())
}

#[test]
fn test_read_parquet_with_filter() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Create a test parquet file
    let temp_file = create_test_parquet_file()?;
    let file_path = temp_file.path().to_str().unwrap();
    
    // Read the parquet file with a filter (id > 3)
    let filter = vec!["id > 3".to_string()];
    let record_batch = formats::read_parquet_with_filter(file_path, &filter)?;
    
    // Verify the filtered data
    assert_eq!(record_batch.num_rows(), 2); // Should only include id=4 and id=5
    
    let id_column = record_batch.column(0);
    let id_array = id_column.as_primitive::<arrow::datatypes::Int32Type>();
    
    // All values should be > 3
    for i in 0..id_array.len() {
        assert!(id_array.value(i) > 3);
    }
    
    Ok(())
}

#[test]
fn test_load_parquet_files_parallel() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Create multiple test parquet files
    let temp_dir = create_multiple_parquet_files(5)?;
    let dir_path = temp_dir.path().to_str().unwrap();
    
    // Load all parquet files in parallel
    let pattern = format!("{}/*.parquet", dir_path);
    let result = formats::load_parquet_files_parallel(&pattern, None)?;
    
    // Verify the combined data
    assert_eq!(result.num_rows(), 15); // 5 files * 3 rows each
    assert_eq!(result.num_columns(), 2);
    
    Ok(())
}

#[test]
fn test_load_parquet_files_parallel_with_filter() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    
    // Create multiple test parquet files
    let temp_dir = create_multiple_parquet_files(5)?;
    let dir_path = temp_dir.path().to_str().unwrap();
    
    // Load all parquet files in parallel with a filter
    let pattern = format!("{}/*.parquet", dir_path);
    let filter = Some(vec!["id > 20".to_string()]);
    let result = formats::load_parquet_files_parallel(&pattern, filter)?;
    
    // Verify the filtered data
    let id_column = result.column(0);
    let id_array = id_column.as_primitive::<arrow::datatypes::Int32Type>();
    
    // All values should be > 20
    for i in 0..id_array.len() {
        assert!(id_array.value(i) > 20);
    }
    
    Ok(())
}