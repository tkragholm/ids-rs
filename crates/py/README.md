# ids-rs Python Package

A Python wrapper for the IDS (Incidence Density Sampling) CLI tool.

## Installation

```bash
pip install ids-rs
```

## Prerequisites

This package is a wrapper around the IDS CLI tool. You need to have the `ids` binary installed and available in your PATH for this package to work.

## Usage

After installation, you can access the IDS functionality through the `ids-py` command, which forwards all commands to the main `ids` CLI:

```bash
# Check balance between matched cases and controls
ids-py CheckBalance --matches-file matched_pairs.csv --covariate-dir data --structured

# Generate synthetic register data
ids-py GenerateRegisters --output-dir data/registers --num-records 1000000

# Sample controls for cases
ids-py Sample --input data/pediatric.csv --controls 4
```

## From Python Code

You can also call the IDS CLI from Python code:

```python
import subprocess

# Run a command
result = subprocess.run([
    "ids-py", 
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

The following commands are available through the `ids-py` wrapper:

- `GenerateRegisters`: Create synthetic register data
- `Sample`: Perform incidence density sampling
- `CheckBalance`: Analyze covariate balance
- `Config`: Generate and manage configurations

For detailed usage of each command, use:

```bash
ids-py <command> --help
```

## Troubleshooting

If you get an error like "Failed to execute the IDS CLI binary", ensure that:
1. The main `ids` binary is installed
2. The `ids` binary is in your PATH
3. You have proper permissions to execute the binary
