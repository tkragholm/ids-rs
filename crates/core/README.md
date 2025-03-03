# IDS-RS Core

Core algorithms and business logic for incidence density sampling and analysis in epidemiological research.

## Overview

This crate implements the central algorithms and statistical methods for the IDS-RS system, providing:

- Incidence density sampling implementation
- Matching quality assessment
- Statistical visualization and plotting
- Performance-optimized data processing

## Key Features

- **Sampler**: Implements the incidence density sampling algorithm for case-control studies
- **Matching Quality**: Tools to assess the quality of case-control matching
- **Plotting**: Statistical visualization functions for result analysis
- **Utils**: Shared utilities for core algorithms
- **Error Handling**: Domain-specific error definitions

## Usage

The core crate is typically used by the CLI and Python interfaces rather than being called directly:

```rust
use core::sampler::IncidenceDensitySampler;
use core::matching_quality::MatchingQualityAssessor;

// Create a sampler with configuration
let sampler = IncidenceDensitySampler::new(config);

// Generate matched controls for cases
let matched_pairs = sampler.sample(&cases, &population)?;
```

## Dependencies

- **IDS-RS Types**: Uses the data types and traits defined in the types crate
- **Rayon**: For parallel processing
- **NDArray**: For numerical computations
- **Plotters**: For visualization
- **MiMalloc**: For optimized memory allocation