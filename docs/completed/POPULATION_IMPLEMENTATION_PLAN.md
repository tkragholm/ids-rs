# Implementation Plan for Population Generation in ids-rs

This document outlines a structured approach to implement population generation functionality in the ids-rs codebase, based on analysis of the cdef_cohort Python package.

> **Important Note**: The `/crates` directory contains legacy code from before refactoring and should not be modified. All new code should be implemented in the `/src` directory.

## 1. Component Analysis & Mapping

| Python Component | Rust Equivalent/Location | Notes |
|------------------|--------------------------|-------|
| Parquet Reading | `src/schema/parquet.rs` | Use the new filter expressions |
| Population Data Model | `src/model` | Create population specific model |
| Registry Definitions | `src/registry` | Add MFR registry if needed |
| Date Parsing | `src/core/date.rs` | Use existing date functionality |
| Filtering/Processing | `src/algorithm` | Add population-specific algorithms |
| Summary Generation | `src/utils/reports` | Extend reporting capabilities |

## 2. Implementation Steps

### Phase 1: Core Data Structures (Foundational)

1. **Create Population-Related Models**:
   - Define Population and Family record types in `src/model/population.rs`
   - Implement necessary traits for Arrow conversions

2. **Add MFR Registry (If Missing)**:
   - Add schema definition in `src/schema/mfr.rs`
   - Add registry loader in `src/registry/mfr.rs`
   - Register in registry factory

### Phase 2: Data Processing Logic

3. **Implement Population Builder**:
   - Create `src/algorithm/population.rs` for combining BEF and MFR data
   - Implement functions for filtering by birth year range
   - Add parent-child linking and processing

4. **Implement Data Summarization**:
   - Create utility for population statistics in `src/utils/reports/population.rs`
   - Implement functions for generating statistics similar to Python version

### Phase 3: CLI Integration

5. **Add Command-Line Interface**:
   - Add population generation command in `src/commands/population.rs`
   - Create configuration in `src/commands/population/config.rs`
   - Implement handler in `src/commands/population/handler.rs`

6. **Implement CLI Parser Updates**:
   - Update `src/cli/parser.rs` to include population command
   - Add relevant configuration parameters and defaults

### Phase 4: Tests & Documentation

7. **Add Tests**:
   - Unit tests for population generation
   - Integration tests for the full pipeline
   - Test with sample data

8. **Add Documentation**:
   - Update README with population generation features
   - Add example usage in documentation
   - Document the data structure and formats

## 3. Technical Requirements

1. **Population Data Model**:
```rust
// src/model/population.rs
pub struct Population {
    pub pnr: String,
    pub birth_date: NaiveDate,
    pub father_id: Option<String>,
    pub mother_id: Option<String>,
    pub family_id: Option<String>,
}

pub struct FamilyData {
    pub pnr: String,
    pub birth_date: NaiveDate,
    pub father_id: Option<String>,
    pub father_birth_date: Option<NaiveDate>,
    pub mother_id: Option<String>,
    pub mother_birth_date: Option<NaiveDate>,
    pub family_id: Option<String>,
}
```

2. **Population Generation Logic**:
```rust
// src/algorithm/population.rs
pub fn combine_population_data(
    bef_data: &RecordBatch,
    mfr_data: &RecordBatch,
    birth_year_start: i32,
    birth_year_end: i32,
) -> Result<RecordBatch, IdsError> {
    // Apply year range filter
    // Combine data from both sources
    // Process and link parent data
    // Return combined data
}
```

3. **Command Implementation**:
```rust
// src/commands/population/handler.rs
pub fn generate_population(config: &PopulationConfig) -> Result<(), IdsError> {
    // Read BEF data
    // Read MFR data
    // Combine data
    // Generate summary statistics
    // Save output files
}
```

## 4. Integration Approach

1. **Utilize Existing Arrow Functionality**:
   - Leverage the existing Arrow storage backend for efficient data handling
   - Use the advanced filtering capabilities from your recent implementation

2. **Leverage Parallelism**:
   - Implement parallel processing for large registers
   - Utilize the batch processing capabilities in the existing code

3. **Progressive Implementation**:
   - Start with basic functionality (data loading and filtering)
   - Add more complex features (parent linking, statistics) iteratively
   - Ensure each step is well-tested before proceeding

## 5. Priority Tasks (Implementation Order)

1. Define data models and schema definitions (foundational)
2. Implement basic data loading and filtering (core functionality)
3. Add data combination and parent-child linking (business logic)
4. Implement summary generation (reporting)
5. Create CLI command and integration (user interface)
6. Add tests and documentation (quality assurance)

## 6. Data Processing Workflow

1. **Read Register Data**:
   - Read BEF (population) data with selected columns
   - Read MFR (medical birth) data with selected columns
   - Standardize column names between data sources

2. **Filter and Preprocess**:
   - Filter children by birth year range
   - Extract unique children records
   - Parse and standardize date formats

3. **Combine Data Sources**:
   - Combine BEF and MFR data, preferring BEF when available
   - Track statistics about data sources and missing values
   - Create a unified population dataset

4. **Process Parent Information**:
   - Extract parent data from source registers
   - Link parent information to child records
   - Create a family-based dataset with relations

5. **Generate Summary Statistics**:
   - Create basic population statistics
   - Generate missing data reports
   - Prepare age distribution data
   - Calculate parent-child age differences

6. **Save Output**:
   - Save main population file
   - Save summary statistics files
   - Create report files