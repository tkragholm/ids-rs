# IDS-RS: Incidence Density Sampling Tool

This Python package provides access to the IDS-RS tool, a Rust implementation for performing incidence density sampling in epidemiological research. The package includes both the binary executable and all necessary mapping and schema files.

## Installation

```bash
pip install ids-tk
```

## Usage

You can use this package from the command line:

```bash
ids --help
```

Or from Python:

```python
import ids_tk

# Run with arguments
ids_tk.run_ids(["--help"])

# Check what mappings and schemas are available
print(ids_tk.list_mappings())
print(ids_tk.list_schemas())

# Get a specific mapping file
hfaudd_mapping = ids_tk.get_mapping_file("hfaudd")
if hfaudd_mapping:
    with open(hfaudd_mapping) as f:
        hfaudd_data = json.load(f)
    print(f"HFAUDD mapping contains {len(hfaudd_data)} entries")
```

## Working with Mappings and Schemas

The package includes all necessary mapping and schema files:

```python
import ids_tk
import json

# List all available mappings
mappings = ids_tk.list_mappings()
print(f"Available mappings: {mappings}")

# Load a specific mapping file
hfaudd_path = ids_tk.get_mapping_file("hfaudd")
with open(hfaudd_path) as f:
    hfaudd_mapping = json.load(f)

# List all available schemas
schemas = ids_tk.list_schemas()
print(f"Available schemas: {schemas}")

# Load a specific schema
bef_schema_path = ids_tk.get_schema_file("bef")
with open(bef_schema_path) as f:
    bef_schema = json.load(f)
```

## Features

- Fast incidence density sampling for large datasets
- Matching on birth date, parent age and other covariates
- Comprehensive reporting and statistics
- Generate synthetic data for testing
- Built-in access to JSON mapping and schema files