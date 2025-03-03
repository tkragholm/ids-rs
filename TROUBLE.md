diagnostics
ids-rs/crates/cli/src/main_run.rs

```rust
        ),
    }
}
// error: unexpected closing delimiter: `}`
//        unexpected closing delimiter

fn setup_directories(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
```

ids-rs/crates/cli/src/main.rs

```rust
    // Execute the requested command
    match &cli.command {
// error: missing match arm: `&Config { .. }` not covered
        Commands::GenerateRegisters {
            output_dir,
```

ids-rs/crates/py/src/bin/main.rs

```rust
// Generate sample matched pairs and statistics
fn generate_sample_matches(output_dir: &str, input_file: &str, controls_per_case: usize) {
// warning: unused variable: `input_file`
//          `#[warn(unused_variables)]` on by default
    let dir_path = Path::new(output_dir);
```

ids-rs/crates/cli/src/main_functions.rs

```rust
    // Check for the most common command line mistake - missing space after --family-file
    for (_i, arg) in std::env::args().enumerate() {
        if arg.starts_with("--family-file") && arg \!= "--family-file" {
// error: unknown start of token: \
            eprintln\!("ERROR: Detected possible command line issue. You provided '{}' without a space.", arg);
            eprintln\!("       Did you mean to write: --family-file {}", &arg[13..]);
```

```rust
    for (_i, arg) in std::env::args().enumerate() {
        if arg.starts_with("--family-file") && arg \!= "--family-file" {
            eprintln\!("ERROR: Detected possible command line issue. You provided '{}' without a space.", arg);
// error: unknown start of token: \
            eprintln\!("       Did you mean to write: --family-file {}", &arg[13..]);
            eprintln\!("       Check other parameters too. Put a space between each flag and its value.");
```

```rust
        if arg.starts_with("--family-file") && arg \!= "--family-file" {
            eprintln\!("ERROR: Detected possible command line issue. You provided '{}' without a space.", arg);
            eprintln\!("       Did you mean to write: --family-file {}", &arg[13..]);
// error: unknown start of token: \
            eprintln\!("       Check other parameters too. Put a space between each flag and its value.");
            std::process::exit(1);
```

```rust
            eprintln\!("ERROR: Detected possible command line issue. You provided '{}' without a space.", arg);
            eprintln\!("       Did you mean to write: --family-file {}", &arg[13..]);
            eprintln\!("       Check other parameters too. Put a space between each flag and its value.");
// error: unknown start of token: \
            std::process::exit(1);
        }
```

```rust
    // Connect logger with progress bars to prevent progress bars from being interrupted by logs
    if let Err(e) = LogWrapper::new(multi.clone(), logger).try_init() {
        eprintln\!("Warning: Failed to initialize logger: {}", e);
// error: unknown start of token: \
    }
```

```rust
        Ok(c) => c,
        Err(e) => {
            eprintln\!("{}", e);
// error: unknown start of token: \
            eprintln\!("\nNOTE: Make sure there is a space between each flag and its value\!");
            eprintln\!("Example: --family-file data/registers/family.parquet");
```

```rust
        Err(e) => {
            eprintln\!("{}", e);
            eprintln\!("\nNOTE: Make sure there is a space between each flag and its value\!");
// error: unknown start of token: \
            eprintln\!("Example: --family-file data/registers/family.parquet");
            std::process::exit(1);
```

```rust
        Err(e) => {
            eprintln\!("{}", e);
            eprintln\!("\nNOTE: Make sure there is a space between each flag and its value\!");
// error: unknown character escape: `!`
//        for more information, visit <https://doc.rust-lang.org/reference/tokens.html#literals>
            eprintln\!("Example: --family-file data/registers/family.parquet");
            std::process::exit(1);
```

```rust
            eprintln\!("{}", e);
            eprintln\!("\nNOTE: Make sure there is a space between each flag and its value\!");
            eprintln\!("Example: --family-file data/registers/family.parquet");
// error: unknown start of token: \
            std::process::exit(1);
        }
```

```rust
    }

    info\!("Created output directories in {}", output_dir);
// error: unknown start of token: \
    Ok(())
}
```

```rust
fn configure_logging_with_dir(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = format\!("{}/log/cli.log", output_dir);
// error: unknown start of token: \

    // Use more restrictive logging in the console to reduce terminal noise
```
