# IDS-RS Re-implementation Documentation

## Overview

IDS-RS is a Rust application for Incidence Density Sampling (IDS) in epidemiological research. The application provides tools for matching cases to controls based on specific criteria, analyzing the covariate balance between matched groups, and generating reports. This document outlines the current implementation and provides guidance for a more concise reimplementation using CSV files throughout.

## Current Architecture

The application is organized as a Rust workspace with several crates, each handling specific functionality:

### Core Crates

1. **types**: Foundation with data models, traits, and error handling
   - Provides core data structures like `Pnr`, `Record`, `Covariate`
   - Implements traits for data access and processing
   - Extensive error handling system
   - Arrow/Parquet storage backend

2. **core**: Core algorithms for incidence density sampling
   - `IncidenceDensitySampler` for matching cases to controls
   - Matching quality evaluation
   - Statistical calculations
   - CSV output generation

3. **loader**: Data loading from various Danish registry formats
   - Parallel and sequential loading strategies
   - Schema definitions for different registries (AKM, BEF, IND, UDDF)
   - Integration with Arrow/Parquet

4. **covariates**: Covariate balance analysis
   - Balance checking between case and control groups
   - Memory management strategies for large datasets
   - Balance metrics calculation
   - Report generation

5. **ids**: Main CLI application
   - Command-line interface
   - Command dispatching
   - Logging and configuration
   - Integration of all other crates

6. **utils**: Shared utilities
   - Date handling
   - Logging
   - File pattern matching
   - String manipulation

7. **datagen**: Test data generation
   - Synthetic data creation for testing

### Key Data Flow

1. **Data Loading**: Load data from registry files (Parquet format)
   - Load case/control data from CSV input
   - Optionally load registry data for additional analysis

2. **Sampling**: Match cases to controls based on criteria
   - Cases identified by presence of treatment date
   - Controls matched based on birth date window
   - Parent birth dates used for additional matching criteria

3. **Result Generation**: Output matched pairs to CSV
   - Output detailed matching statistics
   - Generate quality metrics

4. **Balance Analysis**: Analyze covariate balance
   - Calculate statistical measures (standardized mean differences)
   - Generate reports on balance quality
   - Create visualizations

## Simplified Reimplementation Plan

The goal is to create a more concise implementation using CSV files throughout and with a simplified architecture:

### 1. Single Crate Structure

```
ids-rs/
├── src/
│   ├── main.rs           # Entry point and CLI handling
│   ├── sampler.rs        # Core sampling algorithm
│   ├── balance.rs        # Balance checking
│   ├── models.rs         # Core data models
│   ├── error.rs          # Error handling
│   ├── utils.rs          # Utilities (date, logging, etc.)
│   ├── reporting.rs      # Report generation
│   └── csv/
│       ├── reader.rs     # CSV reading functionality
│       └── writer.rs     # CSV writing functionality
├── examples/             # Example usage
├── tests/                # Tests
└── Cargo.toml            # Dependencies
```

### 2. Key Data Structures

Replace complex Arrow-based storage with simple CSV-oriented structures:

```rust
// Simple record structure for case/control data
struct Record {
    pnr: String,                   // Personal identification number
    birth_date: NaiveDate,         // Birth date
    treatment_date: Option<NaiveDate>, // Treatment date (if a case)
    mother_birth_date: Option<NaiveDate>, // Mother's birth date
    father_birth_date: Option<NaiveDate>, // Father's birth date
    // Additional fields as needed
}

// Matched case-control pair
struct CaseControlPair {
    case: Record,
    controls: Vec<Record>,
}

// Covariate data
struct Covariate {
    name: String,
    value_type: CovariateType,
    value: CovariateValue,
}

enum CovariateType {
    Numeric,
    Categorical,
    Binary,
    Date,
}

enum CovariateValue {
    Numeric(f64),
    Categorical(String),
    Binary(bool),
    Date(NaiveDate),
}
```

### 3. CSV-Based Data Handling

Replace Arrow/Parquet with direct CSV handling:

```rust
// CSV loading
fn load_records_from_csv(path: &str) -> Result<Vec<Record>, Error> {
    let mut reader = csv::Reader::from_path(path)?;
    let records: Vec<Record> = reader.deserialize().collect::<Result<Vec<_>, _>>()?;
    Ok(records)
}

// CSV writing
fn write_matches_to_csv(pairs: &[CaseControlPair], path: &str) -> Result<(), Error> {
    let mut writer = csv::Writer::from_path(path)?;
    
    // Write header
    writer.write_record(&[
        "case_id", "case_pnr", "case_birth_date", "case_treatment_date",
        "control_id", "control_pnr", "control_birth_date", 
        "birth_date_diff_days", "mother_diff_days", "father_diff_days"
    ])?;
    
    // Write data
    for pair in pairs {
        for control in &pair.controls {
            writer.write_record(&[
                &pair.case.pnr,
                &pair.case.birth_date.to_string(),
                &pair.case.treatment_date.map_or("".to_string(), |d| d.to_string()),
                &control.pnr,
                &control.birth_date.to_string(),
                &(pair.case.birth_date - control.birth_date).num_days().abs().to_string(),
                // Other differences...
            ])?;
        }
    }
    
    writer.flush()?;
    Ok(())
}
```

### 4. Simplified Sampling Algorithm

```rust
struct Sampler {
    records: Vec<Record>,
    birth_date_window: i64,
    parent_date_window: i64,
}

impl Sampler {
    // Create a new sampler
    fn new(records: Vec<Record>, birth_date_window: i64, parent_date_window: i64) -> Self {
        Self { records, birth_date_window, parent_date_window }
    }
    
    // Perform matching
    fn sample(&self, controls_per_case: usize) -> Vec<CaseControlPair> {
        let mut pairs = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Identify cases and controls
        let cases: Vec<&Record> = self.records.iter()
            .filter(|r| r.treatment_date.is_some())
            .collect();
            
        let controls: Vec<&Record> = self.records.iter()
            .filter(|r| r.treatment_date.is_none())
            .collect();
            
        // For each case, find eligible controls
        for case in cases {
            let eligible_controls: Vec<&Record> = controls.iter()
                .filter(|c| {
                    // Birth date within window
                    let birth_diff = (case.birth_date - c.birth_date).num_days().abs();
                    if birth_diff > self.birth_date_window {
                        return false;
                    }
                    
                    // Check mother's birth date if available
                    if let (Some(case_mother), Some(control_mother)) = (case.mother_birth_date, c.mother_birth_date) {
                        let mother_diff = (case_mother - control_mother).num_days().abs();
                        if mother_diff > self.parent_date_window {
                            return false;
                        }
                    } else if case.mother_birth_date.is_some() != c.mother_birth_date.is_some() {
                        return false;
                    }
                    
                    // Check father's birth date if available
                    if let (Some(case_father), Some(control_father)) = (case.father_birth_date, c.father_birth_date) {
                        let father_diff = (case_father - control_father).num_days().abs();
                        if father_diff > self.parent_date_window {
                            return false;
                        }
                    } else if case.father_birth_date.is_some() != c.father_birth_date.is_some() {
                        return false;
                    }
                    
                    true
                })
                .cloned()
                .collect();
                
            // Select controls randomly
            let selected_controls = if eligible_controls.len() <= controls_per_case {
                eligible_controls.clone()
            } else {
                use rand::seq::SliceRandom;
                eligible_controls.choose_multiple(&mut rng, controls_per_case).cloned().collect()
            };
            
            // Add case-control pair
            if !selected_controls.is_empty() {
                pairs.push(CaseControlPair {
                    case: case.clone(),
                    controls: selected_controls,
                });
            }
        }
        
        pairs
    }
}
```

### 5. Simplified Balance Checking

```rust
struct BalanceChecker {
    pairs: Vec<CaseControlPair>,
    covariates: Vec<Covariate>,
}

impl BalanceChecker {
    fn check_balance(&self) -> BalanceResults {
        let mut results = BalanceResults::new();
        
        for covariate in &self.covariates {
            match covariate.value_type {
                CovariateType::Numeric => {
                    // Calculate standardized mean difference for numeric covariates
                    let case_values: Vec<f64> = self.pairs.iter()
                        .filter_map(|p| extract_numeric_value(&p.case, covariate.name.as_str()))
                        .collect();
                        
                    let control_values: Vec<f64> = self.pairs.iter()
                        .flat_map(|p| p.controls.iter())
                        .filter_map(|c| extract_numeric_value(c, covariate.name.as_str()))
                        .collect();
                        
                    let smd = calculate_standardized_mean_difference(&case_values, &control_values);
                    results.add_numeric_result(covariate.name.clone(), smd);
                },
                CovariateType::Categorical => {
                    // Calculate distribution differences for categorical covariates
                    // [Implementation here]
                },
                // Other types...
            }
        }
        
        results
    }
}

fn calculate_standardized_mean_difference(case_values: &[f64], control_values: &[f64]) -> f64 {
    let case_mean = case_values.iter().sum::<f64>() / case_values.len() as f64;
    let control_mean = control_values.iter().sum::<f64>() / control_values.len() as f64;
    
    let case_var = case_values.iter()
        .map(|x| (x - case_mean).powi(2))
        .sum::<f64>() / (case_values.len() - 1) as f64;
        
    let control_var = control_values.iter()
        .map(|x| (x - control_mean).powi(2))
        .sum::<f64>() / (control_values.len() - 1) as f64;
        
    let pooled_sd = (case_var + control_var).sqrt() / 2.0;
    
    (case_mean - control_mean).abs() / pooled_sd
}
```

### 6. Efficient CSV-Based Registry Loading

```rust
// Define registry loader for different registry types
enum RegistryType {
    BEF, // Population register
    AKM, // Labor market register
    IND, // Income register
    UDDF, // Education register
}

// Load registry data directly into memory from CSV
fn load_registry(reg_type: RegistryType, path: &str) -> Result<Vec<HashMap<String, String>>, Error> {
    let mut reader = csv::Reader::from_path(path)?;
    let records: Vec<HashMap<String, String>> = reader.deserialize().collect::<Result<Vec<_>, _>>()?;
    Ok(records)
}

// Join registry data with case-control pairs by PNR
fn enrich_pairs_with_registry(
    pairs: &mut [CaseControlPair], 
    registry_data: &[HashMap<String, String>],
    fields_to_extract: &[&str]
) -> Result<(), Error> {
    // Create an index of registry data by PNR for fast lookup
    let registry_index: HashMap<String, &HashMap<String, String>> = registry_data.iter()
        .filter_map(|record| {
            record.get("pnr").map(|pnr| (pnr.clone(), record))
        })
        .collect();
    
    // Enrich case and control records
    for pair in pairs {
        // Enrich case
        if let Some(registry_record) = registry_index.get(&pair.case.pnr) {
            for field in fields_to_extract {
                if let Some(value) = registry_record.get(*field) {
                    // Add to case covariates
                }
            }
        }
        
        // Enrich controls
        for control in &mut pair.controls {
            if let Some(registry_record) = registry_index.get(&control.pnr) {
                for field in fields_to_extract {
                    if let Some(value) = registry_record.get(*field) {
                        // Add to control covariates
                    }
                }
            }
        }
    }
    
    Ok(())
}
```

### 7. CLI Interface

```rust
fn main() -> Result<(), Error> {
    // Parse command line arguments
    let matches = clap::Command::new("ids-rs")
        .version("1.0.0")
        .about("Incidence Density Sampling")
        .subcommand(
            clap::Command::new("sample")
                .about("Sample controls for cases")
                .arg(clap::Arg::new("input")
                    .help("Input CSV file")
                    .required(true))
                .arg(clap::Arg::new("controls")
                    .help("Number of controls per case")
                    .default_value("4"))
                .arg(clap::Arg::new("birth-window")
                    .help("Birth date matching window in days")
                    .default_value("30"))
                .arg(clap::Arg::new("parent-window")
                    .help("Parent birth date matching window in days")
                    .default_value("365"))
                .arg(clap::Arg::new("output")
                    .help("Output directory")
                    .default_value("output"))
        )
        .subcommand(
            clap::Command::new("check-balance")
                .about("Check covariate balance")
                .arg(clap::Arg::new("matches-file")
                    .help("File with matched pairs")
                    .required(true))
                // Other arguments...
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("sample", args)) => {
            let input = args.get_one::<String>("input").unwrap();
            let controls = args.get_one::<usize>("controls").unwrap();
            let birth_window = args.get_one::<i64>("birth-window").unwrap();
            let parent_window = args.get_one::<i64>("parent-window").unwrap();
            let output_dir = args.get_one::<String>("output").unwrap();
            
            sample_command(input, *controls, *birth_window, *parent_window, output_dir)?;
        },
        Some(("check-balance", args)) => {
            let matches_file = args.get_one::<String>("matches-file").unwrap();
            // Other arguments...
            
            check_balance_command(matches_file)?;
        },
        _ => {
            println!("No command specified. Use --help for usage information.");
        }
    }
    
    Ok(())
}

// Sample command implementation
fn sample_command(
    input: &str, 
    controls: usize, 
    birth_window: i64, 
    parent_window: i64, 
    output_dir: &str
) -> Result<(), Error> {
    // Create output directory
    std::fs::create_dir_all(output_dir)?;
    
    // Load records
    let records = load_records_from_csv(input)?;
    
    // Create sampler
    let sampler = Sampler::new(records, birth_window, parent_window);
    
    // Perform sampling
    let pairs = sampler.sample(controls);
    
    // Write results
    let output_path = format!("{}/matched_pairs.csv", output_dir);
    write_matches_to_csv(&pairs, &output_path)?;
    
    // Calculate and write matching quality statistics
    let stats_path = format!("{}/matching_stats.csv", output_dir);
    write_matching_statistics(&pairs, &stats_path)?;
    
    println!("Successfully matched {} cases with controls", pairs.len());
    Ok(())
}
```

## Key Advantages of the Simplified Implementation

1. **Reduced Complexity**: Single crate instead of 7 interdependent crates
2. **Simplified Data Format**: Direct CSV processing instead of Arrow/Parquet
3. **Smaller Memory Footprint**: No overhead from Arrow columnar format
4. **Fewer Dependencies**: Minimal external libraries required
5. **More Direct Implementation**: Clear data flow and simpler code paths
6. **Easier Maintenance**: Less code to maintain, simpler interfaces
7. **Better Performance for Small/Medium Datasets**: No Arrow conversion overhead

## Key Components to Implement

1. **CSV Reading/Writing**: Utilizing the `csv` crate for direct parsing
2. **Efficient Matching Algorithm**: Hash-based lookup for birth date windows
3. **Simple Balance Checking**: Direct statistical calculations without complex abstractions
4. **Comprehensive Error Handling**: Clear error messages with context
5. **Progress Reporting**: Simple progress indication for long-running operations
6. **Reporting**: CSV-based reports with optional visualization

## Dependencies for Reimplementation

Essential dependencies:
- `csv`: For CSV reading/writing
- `chrono`: For date handling
- `clap`: For command-line argument parsing
- `rand`: For random sampling
- `thiserror`: For error handling
- `serde`: For serialization/deserialization
- `log` and `env_logger`: For logging

Optional dependencies:
- `indicatif`: For progress bars (can be simplified)
- `statrs`: For statistical calculations
- `plotters`: For visualization (if needed)

## Performance Considerations

1. **Pre-indexing**: Create indexes for fast lookup, especially for birth dates
2. **Memory Management**: Stream data when possible, process in batches
3. **Parallelism**: Use Rayon for parallel processing of independent operations
4. **I/O Optimizations**: Buffer CSV reading/writing to reduce system calls

## Timeline for Reimplementation

1. Core data structures and CSV handling (2 days)
2. Sampling algorithm (3 days)
3. Balance checking (2 days)
4. CLI interface and command handling (1 day)
5. Testing and optimization (2 days)

Total estimate: 10 days for a complete reimplementation