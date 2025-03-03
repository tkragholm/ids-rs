# ids-rs Python Package

A Python distribution of the IDS (Incidence Density Sampling) CLI placeholder.

## Installation

```bash
pip install ids-rs
```

## Overview

This package provides a standalone placeholder implementation of the IDS CLI. After installation, the `ids` command will be available in your PATH.

**Note**: This is currently a minimal placeholder that recognizes the same commands as the full IDS-RS implementation but doesn't provide the full functionality. It serves as a command-line interface demonstration and will be expanded in future versions.

## Usage

After installation, you can access the IDS placeholder through the `ids` command:

```bash
# Check balance between matched cases and controls
ids CheckBalance

# Generate synthetic register data
ids GenerateRegisters

# Sample controls for cases
ids Sample

# Generate configurations
ids Config
```

## From Python Code

You can also call the IDS CLI from Python code:

```python
import subprocess

# Run a command
result = subprocess.run([
    "ids", 
    "CheckBalance"
], capture_output=True, text=True)

# Process output
print(result.stdout)
```

## Available Commands

The following commands are recognized by the placeholder:

- `GenerateRegisters`: Create synthetic register data
- `Sample`: Perform incidence density sampling  
- `CheckBalance`: Analyze covariate balance
- `Config`: Generate and manage configurations

For help with the placeholder command, use:

```bash
ids --help
```
