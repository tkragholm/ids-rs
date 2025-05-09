# Population SCD Implementation

This document describes the implementation of the Population SCD functionality, which identifies children from a population who have been diagnosed with a severe chronic disease (SCD).

## Overview

The Population SCD functionality combines two key components of the IDS-RS system:

1. **Population Data**: Children data generated by the Population command
2. **SCD Analysis**: The Severe Chronic Disease (SCD) algorithm for health data

By linking these components, we can identify which children in the population have been diagnosed with severe chronic diseases, what specific types of diseases they have, and when they were first diagnosed.

## Key Components

### 1. `algorithm/population_scd.rs`

This module contains the core algorithms for:

- Matching population data with SCD results
- Creating an enhanced population dataset with SCD indicators
- Extracting children with SCD into a separate dataset
- Generating summary statistics

### 2. `commands/population_scd/`

The command module provides:

- Configuration structure (`PopulationScdCommandConfig`)
- Command handler (`handle_population_scd_command`)
- CLI integration

### 3. CLI Integration

The functionality is integrated into the CLI through:

- New subcommand: `population-scd`
- Command arguments for population data, LPR data, and output configuration

## Data Flow

1. Load population data from Parquet file
2. Load LPR data (LPR2 and/or LPR3)
3. Process LPR data with the LPR module
4. Apply SCD algorithm to processed LPR data
5. Match population data with SCD results
6. Enhance population data with SCD indicators
7. Extract children with SCD into a separate dataset
8. Generate summary statistics
9. Save results to Parquet files and CSV reports

## Usage

To use the Population SCD functionality:

1. First generate population data:
   ```
   ids-rs population --bef /path/to/bef/data --mfr /path/to/mfr/data --output ./output
   ```

2. Then analyze this population for SCD:
   ```
   ids-rs population-scd --population ./output/population.parquet --lpr /path/to/lpr/data --output ./output/scd
   ```

3. Optional parameters:
   - `--include-lpr2` (default: true): Include LPR2 data
   - `--include-lpr3` (default: true): Include LPR3 data
   - `--start-date YYYY-MM-DD`: Filter health data from this date
   - `--end-date YYYY-MM-DD`: Filter health data until this date

## Output Files

The command produces the following outputs:

1. `population_scd.parquet`: Enhanced population data with SCD indicators
2. `scd_children.parquet`: Only children with SCD diagnosis
3. `population_scd_summary.csv`: Summary statistics

## Technical Implementation Details

### Population SCD Algorithm

The `identify_scd_in_population` function:

1. Takes population data and LPR data as input
2. Applies the SCD algorithm to LPR data to identify health records with SCD
3. Creates a map from patient ID (PNR) to SCD result
4. Processes each child in the population:
   - Looks up the child's PNR in the SCD result map
   - Adds SCD indicators (is_scd, first_scd_date, disease categories)
5. Returns an enhanced population dataset and summary statistics

### SCD Indicators

For each child, the following indicators are added:

- `is_scd`: Boolean indicating if the child has any SCD diagnosis
- `first_scd_date`: The date of the first SCD diagnosis
- `scd_category_*`: Boolean fields for each disease category

### Disease Categories

The SCD algorithm classifies diseases into 10 categories:

1. Blood Disorders
2. Immune System Disorders
3. Endocrine Disorders
4. Neurological Disorders
5. Cardiovascular Disorders
6. Respiratory Disorders
7. Gastrointestinal Disorders
8. Musculoskeletal Disorders
9. Renal Disorders
10. Congenital Disorders

Each category has associated ICD-10 diagnosis code prefixes, and a child can have diagnoses in multiple categories.

## Summary Statistics

The PopulationScdResult provides:

- Total children in the population
- Number of children with SCD
- SCD percentage
- Counts for each disease category

These statistics help understand the prevalence of severe chronic diseases in the population and their distribution across disease categories.