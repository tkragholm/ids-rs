# ids-rs Python Package

A complete Python distribution of the IDS (Incidence Density Sampling) CLI tool.

## Installation

```bash
pip install ids-rs
```

## Overview

This package provides a complete, standalone implementation of the IDS CLI tool. After installation, the `ids` command will be available in your PATH and ready to use without any additional dependencies.

## Usage

After installation, you can access the IDS functionality through the `ids` command:

```bash
# Check balance between matched cases and controls
ids CheckBalance --matches-file matched_pairs.csv --covariate-dir data --structured

# Generate synthetic register data
ids GenerateRegisters --output-dir data/registers --num-records 1000000

# Sample controls for cases
ids Sample --input data/pediatric.csv --controls 4
```

## From Python Code

You can also call the IDS CLI from Python code:

```python
import subprocess

# Run a command
result = subprocess.run([
    "ids", 
    "CheckBalance", 
    "--matches-file", "matched_pairs.csv", 
    "--covariate-dir", "data", 
    "--structured"
], capture_output=True, text=True)

# Process output
print(result.stdout)

# Check for errors
if result.returncode != 0:
    print(f"Error: {result.stderr}")
```

## Available Commands

The following commands are available:

- `GenerateRegisters`: Create synthetic register data
- `Sample`: Perform incidence density sampling
- `CheckBalance`: Analyze covariate balance
- `Config`: Generate and manage configurations

For detailed usage of each command, use:

```bash
ids <command> --help
```
