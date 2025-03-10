#!/bin/bash
set -e

# Configuration
NEW_PACKAGE_NAME="ids-toolkit"  # Change this to your desired name
CURRENT_DIR=$(pwd)
REPO_ROOT="$CURRENT_DIR"
IDS_CRATE_DIR="$REPO_ROOT/crates/ids"

echo "Updating README.md for $NEW_PACKAGE_NAME..."

# Create/update the README.md
cat > "$IDS_CRATE_DIR/README.md" << EOF
# $NEW_PACKAGE_NAME

A Python package for Incidence Density Sampling (IDS) using Rust for performance.

## Installation

\`\`\`bash
pip install $NEW_PACKAGE_NAME
\`\`\`

## Usage

\`\`\`python
# Import the package
import ids_toolkit

# List available mapping files
print(ids_toolkit.list_mappings())

# List available schema files
print(ids_toolkit.list_schemas())

# Run the IDS command with arguments
ids_toolkit.run_ids(["--help"])
\`\`\`

### Command-line Usage

You can also use the package from the command line:

\`\`\`bash
# Show help
ids --help

# Run sampling
ids sample --case-file cases.csv --population-file population.csv --output results/
\`\`\`

## Features

- Fast Rust implementation of the Incidence Density Sampling algorithm
- Python bindings for easy integration
- Support for various data formats including CSV and Parquet
- Built-in tools for balance checking and visualization
- Comprehensive logging and error reporting

## License

MIT

\`\`\`
Copyright (c) 2023-2025 Tobias Kragholm

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files...
\`\`\`
EOF

echo "README.md updated successfully!"