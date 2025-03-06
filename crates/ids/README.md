# IDS Crate

This crate provides the command-line interface and core functionality for the Incidence Density Sampling (IDS) toolkit. It exposes both a Rust library and a command-line binary.

## Python Integration

This crate is also available as a Python package:

```bash
pip install ids-toolkit
```

See the Python usage section below for details.

## Crate Structure

The crate is organized into the following modules:

### Core Modules

- `core/` - Core functionality and shared components
  - `config/` - Configuration handling
  - `error.rs` - Error types and handling

### CLI Modules

- `cli/` - Command-line interface
  - `parser.rs` - Command-line argument parsing
  - `types.rs` - CLI data structures

### Command Modules

- `commands/` - Command implementation
  - `balance/` - Balance checking command
    - `config.rs` - Balance command configuration
    - `data_loading.rs` - Data loading utilities
    - `handler.rs` - Main command handler
    - `metrics.rs` - Balance metrics calculation
    - `reporting.rs` - Report generation
  - `config/` - Configuration command
  - `generate/` - Data generation command
  - `sample/` - Sampling command

### Utility Modules

- `utils/` - Utility functions
  - `paths/` - Path handling utilities
  - `reports/` - Report generation utilities
  - `setup/` - Setup utilities

### Convenience Modules

- `prelude/` - Common imports for convenience

## Command-line Usage

```bash
# Show help
ids --help

# Run sampling
ids sample --input cases.csv --controls 4 --birth-window 30 --parent-window 365

# Check covariate balance
ids check-balance --matches-file matches.csv --covariate-dir data/registers/
```

## Python Usage

```python
# Import the package
import ids_toolkit

# List available mapping files
print(ids_toolkit.list_mappings())

# List available schema files
print(ids_toolkit.list_schemas())

# Run the IDS command with arguments
ids_toolkit.run_ids(["--help"])
```

## Features

- Fast Rust implementation of the Incidence Density Sampling algorithm
- Python bindings for easy integration
- Support for various data formats including CSV and Parquet
- Built-in tools for balance checking and visualization
- Comprehensive logging and error reporting

## License

MIT

```
Copyright (c) 2023-2025 Tobias Kragholm

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files...
```
