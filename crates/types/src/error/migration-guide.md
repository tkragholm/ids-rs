# Error Handling Migration Guide

This document serves as a guide for migrating existing code to use the new error handling patterns introduced in the types crate refactoring Phase 3. Following these guidelines will result in more robust error handling with better context and clearer error messages.

## Quick Reference

| Old Pattern | New Pattern | Notes |
|-------------|-------------|-------|
| `unwrap()` | `context("message")?` | For Results |
| `expect("msg")` | `context("message")?` | For Results with better messages |
| `unwrap()` | `ok_or_else(|| IdsError::missing_value("message"))?` | For Options |
| `expect("msg")` | `ok_or_else(|| IdsError::missing_value("msg"))?` | For Options with messages |
| `if !condition { return Err(...) }` | `ensure!(condition, "message")` | For condition checks |
| `return Err(Error::new(...))` | `bail!("message")` | For early returns |
| `.map_err(|e| Error::new(...))` | `.with_context(|| "message")` | For adding context |

## Common Migration Patterns

### 1. Replacing `unwrap()` on `Result`

**Before:**
```rust
let file = File::open("config.json").unwrap();
```

**After:**
```rust
let file = File::open("config.json")
    .with_context(|| "Failed to open config file")?;
```

### 2. Replacing `expect()` on `Result`

**Before:**
```rust
let data = parse_json(&content)
    .expect("Config should be valid JSON");
```

**After:**
```rust
let data = parse_json(&content)
    .with_context(|| "Config should be valid JSON")?;
```

### 3. Replacing `unwrap()` on `Option`

**Before:**
```rust
let name = record.get("name").unwrap();
```

**After:**
```rust
let name = record.get("name")
    .ok_or_else(|| IdsError::missing_value("Required field 'name' is missing"))?;
```

### 4. Replacing Condition Checks

**Before:**
```rust
if value < 0 {
    return Err(IdsError::validation("Value must be non-negative"));
}
```

**After:**
```rust
ensure!(value >= 0, "Value must be non-negative");
```

### 5. Replacing Early Returns

**Before:**
```rust
if data.is_empty() {
    return Err(IdsError::invalid_value("Data cannot be empty"));
}
```

**After:**
```rust
if data.is_empty() {
    bail!("Data cannot be empty");
}
```

### 6. Adding Context to Errors

**Before:**
```rust
fn process_file(path: &Path) -> Result<Data> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| IdsError::Io(e))?;
    // ...
}
```

**After:**
```rust
fn process_file(path: &Path) -> Result<Data> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file at {}", path.display()))?;
    // ...
}
```

## Error Factory Methods

Use the appropriate error factory methods to create specific error types:

```rust
// Configuration errors
IdsError::config("Invalid configuration format")

// Data loading errors
IdsError::data_loading("Failed to load AKM data")

// Missing data errors
IdsError::missing_value("Required field 'education' is missing")

// Type conversion errors
IdsError::type_conversion("Could not convert '123.45' to integer")

// Date errors
IdsError::invalid_date("Invalid date format: expected YYYY-MM-DD")

// Validation errors
IdsError::validation("Age must be between 0 and 120")
```

## Using Error Handling Macros

### `ensure!` Macro

The `ensure!` macro validates a condition and returns early with an error if the condition is false:

```rust
// With string message (creates a Validation error)
ensure!(age >= 0, "Age cannot be negative");

// With formatted message
ensure!(age < 120, "Age {} exceeds maximum allowed (120)", age);

// With specific error type
ensure!(value.is_positive(), IdsError::invalid_value("Value must be positive"));
```

### `try_with_context!` Macro

The `try_with_context!` macro attempts an operation and adds context if it fails:

```rust
// With simple message
let file = try_with_context!(
    File::open(path),
    "Failed to open configuration file"
);

// With formatted message
let data = try_with_context!(
    serde_json::from_str(&content),
    "Failed to parse '{}' as JSON", 
    content.chars().take(30).collect::<String>()
);
```

### `bail!` Macro

The `bail!` macro returns early with an error:

```rust
// With string message (creates a Validation error)
if data.is_empty() {
    bail!("Input data cannot be empty");
}

// With formatted message
if records.len() > max_records {
    bail!("Too many records: {} (maximum allowed: {})", 
          records.len(), max_records);
}

// With specific error type
if !file_exists {
    bail!(IdsError::data_loading("Required file not found"));
}
```

## Real-World Examples

### Example 1: Loading Data from a File

**Before:**
```rust
fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}
```

**After:**
```rust
fn load_config(path: &str) -> Result<Config> {
    // Validate input
    ensure!(path.ends_with(".json"), "Configuration file must be JSON");
    
    // Load file with context
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read configuration at {}", path))?;
    
    // Parse with context
    let config: Config = serde_json::from_str(&content)
        .with_context(|| "Failed to parse configuration as JSON")?;
    
    Ok(config)
}
```

### Example 2: Arrow Data Access

**Before:**
```rust
fn get_column_value(batch: &RecordBatch, column: &str, row: usize) -> Result<String> {
    let col = batch.column_by_name(column).unwrap();
    let array = col.as_any().downcast_ref::<StringArray>().unwrap();
    Ok(array.value(row).to_string())
}
```

**After:**
```rust
fn get_column_value(batch: &RecordBatch, column: &str, row: usize) -> Result<String> {
    // Get column with error handling
    let col = batch.column_by_name(column)
        .ok_or_else(|| IdsError::column_not_found(column))?;
    
    // Downcast with error handling
    let array = col.as_any().downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::type_conversion(
            format!("Column '{}' is not a string array", column)
        ))?;
    
    // Check bounds
    ensure!(row < array.len(), 
        "Row index {} out of bounds (column '{}' has {} rows)", 
        row, column, array.len());
    
    Ok(array.value(row).to_string())
}
```

## Best Practices

1. **Be Specific**: Use the most specific error type for the situation
2. **Add Context**: Always add context to explain what was happening when the error occurred
3. **Propagate Errors**: Use the `?` operator to propagate errors up the call stack
4. **Validate Early**: Use `ensure!` to validate preconditions at the start of functions
5. **Don't Panic**: Avoid `unwrap()`, `expect()`, and `panic!()` in production code
6. **Test Error Paths**: Write tests that verify error cases, not just success paths

## Testing Error Handling

To test error handling, create tests that verify both the error type and the error message:

```rust
#[test]
fn test_load_config_missing_file() {
    let result = load_config("non_existent_file.json");
    
    // Verify it's an error
    assert!(result.is_err());
    
    // Get the error
    let err = result.unwrap_err();
    
    // Verify the error type
    assert!(matches!(err, IdsError::Io(_)));
    
    // Verify the error message contains useful context
    let err_msg = format!("{}", err);
    assert!(err_msg.contains("Failed to read configuration"));
    assert!(err_msg.contains("non_existent_file.json"));
}
```

## Conclusion

By following these guidelines, you'll produce more robust code with better error handling that helps both users and developers understand what went wrong and how to fix it. Consistent error handling across the codebase improves the maintainability and reliability of the entire system.