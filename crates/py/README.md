# ids-rs

A command-line tool for incidence density sampling.

## Installation

```bash
pip install ids-rs
```

## Usage

After installation, the `ids` command will be available in your PATH:

```bash
# Generate synthetic data
ids -g -i data.csv -t 1200000 -c 50000

# Sample controls
ids -i data.csv -n 4 -b 30 -p 365 -o output
```

## Options

- `-i, --input <FILE>`: Input CSV file path
- `-n, --controls <N>`: Number of controls to match per case (default: 4)
- `-b, --birth-window <DAYS>`: Birth date matching window in days (default: 30)
- `-p, --parent-window <DAYS>`: Parent age matching window in days (default: 365)
- `-o, --output-dir <DIR>`: Output directory for results (default: "output")
- `-g, --generate`: Generate synthetic data
- `-t, --num-records <N>`: Number of total records to generate (default: 1,200,000)
- `-c, --num-cases <N>`: Number of treatment cases to generate (default: 50,000)
