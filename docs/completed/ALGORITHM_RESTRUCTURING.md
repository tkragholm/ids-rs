# Algorithm Module Restructuring Plan

Based on an analysis of the current code organization, this document outlines a plan to improve the structure of the `algorithm` module by consolidating related functionality and removing redundancy.

## Current Issues

1. **Population-related code is spread across three files:**
   - `population.rs`: Core population generation
   - `population_scd.rs`: SCD classification for population
   - `population_integration.rs`: Integration with additional registers

2. **LPR and diagnosis-related code is fragmented:**
   - `lpr.rs`: LPR data processing
   - `scd.rs`: SCD algorithm implementation
   - `secondary_diagnosis.rs`: Secondary diagnosis handling

3. **Redundancy and unclear module boundaries:**
   - Secondary diagnosis functionality should be part of LPR or a dedicated diagnosis module
   - Population-related functionality lacks clear boundaries

## Proposed New Structure

### 1. Population Module

Create a new `population` submodule with clear component separation:

```
src/algorithm/population/
├── mod.rs               # Exports and common types
├── core.rs              # Core population dataset generation
├── integration.rs       # Integration with additional registers
└── classification.rs    # Population classification (SCD, etc.)
```

The `population/mod.rs` would export a unified `PopulationConfig` that handles all aspects of population data processing.

### 2. Health Module

Create a `health` submodule that organizes health data processing:

```
src/algorithm/health/
├── mod.rs               # Exports and common types
├── lpr.rs               # LPR data processing
├── diagnosis/
│   ├── mod.rs           # Common diagnosis functionality
│   ├── pattern.rs       # Diagnosis pattern matching
│   ├── secondary.rs     # Secondary diagnosis handling
│   └── scd.rs           # SCD algorithm implementation
└── integration.rs       # Integration between health data sources
```

### 3. Core Algorithm Module

Retain algorithms that are not specific to population or health data:

```
src/algorithm/
├── mod.rs               # Main module exports
├── matching.rs          # Matching algorithm
├── balance.rs           # Balance checking
├── statistics.rs        # Statistical calculations
├── sampler.rs           # Data sampling utilities
├── population/          # Population submodule
└── health/              # Health submodule
```

## Implementation Steps

### Step 1: Reorganize Population Module

1. Create the `population` directory structure
2. Move and refactor core population functionality from `population.rs` to `population/core.rs`
3. Move integration functionality from `population_integration.rs` to `population/integration.rs`
4. Move SCD population functionality from `population_scd.rs` to `population/classification.rs`
5. Create a unified API in `population/mod.rs`

### Step 2: Reorganize Health Module

1. Create the `health` directory structure
2. Move LPR functionality from `lpr.rs` to `health/lpr.rs`
3. Move diagnosis pattern functionality to `health/diagnosis/pattern.rs`
4. Move secondary diagnosis functionality from `secondary_diagnosis.rs` to `health/diagnosis/secondary.rs`
5. Move SCD algorithm from `scd.rs` to `health/diagnosis/scd.rs`
6. Create appropriate exports in module files

### Step 3: Update Algorithm Module Exports

1. Update `algorithm/mod.rs` to export the new module structure
2. Ensure backward compatibility by re-exporting types at the original paths

### Step 4: Update References

1. Update imports throughout the codebase to use the new module paths
2. Update documentation references

## Benefits of the New Structure

1. **Clearer Organization**: Code is organized by domain (population vs. health)
2. **Better Cohesion**: Related functionality is grouped together
3. **Reduced Redundancy**: Common code can be shared more effectively
4. **Easier Navigation**: Logical grouping makes it easier to find related code
5. **Clearer Boundaries**: Each submodule has a clear responsibility

## Specific Improvements

1. **Population Classification**: Moving SCD functionality into `population/classification.rs` allows for adding more classification methods while keeping a clean API

2. **Secondary Diagnosis Integration**: Placing secondary diagnosis handling within the diagnosis module emphasizes that it's a part of diagnosis processing rather than a standalone concept

3. **Code Reuse**: The reorganization will expose opportunities for code reuse between different parts of the algorithm module