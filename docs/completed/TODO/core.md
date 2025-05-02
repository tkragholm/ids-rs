<analysis>
<duplication_area name="Console Output and Formatting">
After analyzing the codebase, one of the most significant areas of duplication is in console output and formatting utilities. The `ConsoleOutput` struct in `console.rs` contains multiple methods for printing formatted output, but there are opportunities for more generic and reusable approaches.

<refactoring_example>
Current implementation (duplicative formatting):
```rust
pub fn key_value(key: &str, value: &str) {
    println!("{}: {}", key.bold(), value);
}

pub fn key_value_colored(key: &str, value: &str, success: bool) {
    let colored_value = if success { value.green() } else { value.red() };
    println!("{}: {}", key.bold(), colored_value);
}
```

Proposed refactoring:
```rust
pub fn format_key_value(key: &str, value: &str, colorize: Option<fn(&str) -> ColoredString>) -> String {
    let formatted_value = match colorize {
        Some(color_fn) => color_fn(value),
        None => value.into(),
    };
    format!("{}: {}", key.bold(), formatted_value)
}

// Usage examples:
println!("{}", ConsoleOutput::format_key_value("Status", "Success", Some(|v| v.green())));
println!("{}", ConsoleOutput::format_key_value("Error", "Failed", Some(|v| v.red())));
println!("{}", ConsoleOutput::format_key_value("Info", "Neutral", None));
```

This approach:
- Reduces code duplication
- Provides more flexible formatting
- Allows for easy extension of coloring and formatting strategies
</refactoring_example>
</duplication_area>

<duplication_area name="Date Validation and Parsing">
Another area with potential for refactoring is date-related utilities spread across different modules.

<refactoring_example>
Current implementation (scattered date validation):
```rust
// In utils.rs
pub fn validate_date(date_str: &str) -> Result<(), SamplingError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| SamplingError::invalid_date("Invalid date format".to_string()))
}

// Duplicate validation logic in multiple modules
pub fn validate_optional_date(date: &Option<NaiveDate>) -> Result<(), SamplingError> {
    match date {
        Some(d) => validate_date(&d.to_string()),
        None => Ok(()),
    }
}
```

Proposed refactoring:
```rust
pub trait DateValidation {
    fn validate(&self) -> Result<(), SamplingError>;
    fn validate_optional(option: &Option<Self>) -> Result<(), SamplingError>
    where
        Self: Sized;
}

impl DateValidation for NaiveDate {
    fn validate(&self) -> Result<(), SamplingError> {
        // Centralized validation logic
        match NaiveDate::parse_from_str(&self.to_string(), "%Y-%m-%d") {
            Ok(_) => Ok(()),
            Err(_) => Err(SamplingError::invalid_date("Invalid date format".to_string())),
        }
    }

    fn validate_optional(option: &Option<Self>) -> Result<(), SamplingError> {
        option.as_ref().map_or(Ok(()), |date| date.validate())
    }
}
```

Benefits:
- Centralizes date validation logic
- Provides a trait-based approach for extensibility
- Reduces scattered validation code
</refactoring_example>
</duplication_area>

<duplication_area name="Error Handling and Logging">
The error handling and logging mechanisms show potential for standardization.

<refactoring_example>
Current error handling:
```rust
pub fn load_records(filename: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    // Multiple explicit error logging and conversion
    if let Err(e) = validate_date(&record.bday.to_string()) {
        log::error!("Invalid birth date at record {}: {}", idx + 1, e);
        return Err(Box::new(SamplingError::invalid_date(
            "Invalid birth date".to_string(),
        )));
    }
}
```

Proposed refactoring:
```rust
pub trait ErrorContext<T> {
    fn with_context<F>(self, context_fn: F) -> Result<T, SamplingError>
    where
        F: FnOnce() -> String;
}

impl<T> ErrorContext<T> for Result<T, impl Error> {
    fn with_context<F>(self, context_fn: F) -> Result<T, SamplingError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let context = context_fn();
            log::error!("{}: {}", context, e);
            SamplingError::invalid_date(context)
        })
    }
}

// Usage
pub fn load_records(filename: &str) -> Result<Vec<Record>, SamplingError> {
    rdr.deserialize()
        .enumerate()
        .map(|(idx, result)| {
            result
                .with_context(|| format!("Invalid record at line {}", idx + 1))
        })
        .collect()
}
```

Benefits:
- Standardizes error context creation
- Reduces boilerplate error handling
- Provides consistent logging and error transformation
</refactoring_example>
</duplication_area>

<recommendation>
Main Duplication Areas:
1. Console output and formatting
2. Date validation and parsing
3. Error handling and logging
4. Matching and sampling utilities

Proposed Refactoring Strategies:
- Introduce trait-based abstractions
- Create generic utility functions
- Centralize common functionality in shared modules
- Use macro-based code generation for repetitive patterns

Potential Benefits:
- Reduced code complexity
- Improved maintainability
- Enhanced code reusability
- More consistent error handling and logging

Challenges:
- Careful migration to avoid breaking existing code
- Potential performance overhead with trait-based abstractions
- Ensuring backward compatibility
- Coordinating changes across multiple modules and crates

Recommended Next Steps:
1. Create a comprehensive RFC documenting proposed refactoring
2. Implement refactoring in stages
3. Develop comprehensive test suite
4. Perform thorough performance profiling
5. Gradual rollout with feature flags
</recommendation>
</analysis>
