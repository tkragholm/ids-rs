# LPR and SCD Implementation for ids-rs

This document provides an overview of the implementation of the Danish Health Data Processing System in the ids-rs codebase.

## Overview

The implementation adds functionality to process Danish National Patient Registry (LPR) data and apply the Severe Chronic Disease (SCD) algorithm for identifying patients with severe chronic diseases based on ICD-10 diagnosis codes.

## Components Implemented

### Registry Loaders

The following registry loaders have been implemented in `/src/registry/lpr.rs`:

1. `LprAdmRegister`: For loading LPR2 admission data (LPR_ADM)
2. `LprDiagRegister`: For loading LPR2 diagnosis data (LPR_DIAG)
3. `LprBesRegister`: For loading LPR2 treatment data (LPR_BES)
4. `Lpr3KontakterRegister`: For loading LPR3 contacts data (LPR3_KONTAKTER)
5. `Lpr3DiagnoserRegister`: For loading LPR3 diagnosis data (LPR3_DIAGNOSER)

These loaders handle loading data from Parquet files and applying filters as needed.

### Data Processing Algorithms

The LPR data processing functionality is implemented in `/src/algorithm/lpr.rs` with the following key functions:

1. `integrate_lpr2_components`: Joins LPR2 tables (LPR_ADM, LPR_DIAG, LPR_BES) on RECNUM
2. `integrate_lpr3_components`: Joins LPR3 tables (LPR3_KONTAKTER, LPR3_DIAGNOSER) on DW_EK_KONTAKT
3. `combine_harmonized_data`: Standardizes and combines data from both LPR2 and LPR3
4. `filter_by_date_range`: Filters health data by date range
5. `process_lpr_data`: Main function that orchestrates the data processing pipeline

### SCD Algorithm

The Severe Chronic Disease algorithm is implemented in `/src/algorithm/scd.rs` with the following key components:

1. `ScdDiseaseCodes`: Stores and manages the ICD-10 code mappings for different disease categories
2. `apply_scd_algorithm`: Applies the SCD algorithm to process health data
3. `scd_results_to_record_batch`: Converts the SCD results to an Arrow RecordBatch

### Command Interface

The SCD command is implemented in the following files:

1. `/src/commands/scd/config.rs`: Configuration options for the SCD command
2. `/src/commands/scd/handler.rs`: Handler for executing the SCD command
3. `/src/cli/commands.rs`: CLI integration for the SCD command

## Data Flow

1. Load LPR data through the registry loaders
2. Integrate components within each format (LPR2 and LPR3)
3. Harmonize data to a common schema
4. Combine data from both formats
5. Apply the SCD algorithm
6. Generate and save analysis results

## Schema Harmonization

Data harmonization is performed with the following field mappings:

| LPR2          | LPR3             | Harmonized      |
|---------------|------------------|-----------------|
| PNR           | CPR              | patient_id      |
| C_ADIAG       | aktionsdiagnose  | primary_diagnosis |
| D_INDDTO      | dato_start       | admission_date  |
| D_UDDTO       | dato_slut        | discharge_date  |
| C_SGH         | SORENHED_ANS     | hospital_code   |
| C_AFD         | SORENHED_ANS     | department_code |
| C_PATTYPE     | kontakttype      | admission_type  |

## SCD Classification

The SCD algorithm classifies patients into 10 disease categories:

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

Each category is associated with specific ICD-10 code prefixes as defined in the `ScdDiseaseCodes` class.

## Usage

The LPR and SCD functionality can be used through the command-line interface:

```
ids-rs scd --lpr <LPR_DATA_PATH> --output <OUTPUT_PATH> [--include-lpr2 <BOOL>] [--include-lpr3 <BOOL>] [--start-date <DATE>] [--end-date <DATE>]
```

This will:
1. Load and integrate LPR data from the specified path
2. Apply the SCD algorithm to identify patients with severe chronic diseases
3. Generate analysis results and save them to the specified output path

## Output

The command generates the following output files:

1. `processed_lpr_data.parquet`: Harmonized LPR data
2. `scd_results.parquet`: Patient-level SCD analysis results
3. `scd_summary.csv`: Summary statistics of the SCD analysis

## Implementation Notes

- The implementation follows the existing ids-rs codebase architecture
- Arrow is used for efficient data processing
- The SCD algorithm is optimized for performance with large datasets
- All functionality is accessible through both the CLI and programmatically through the Rust API