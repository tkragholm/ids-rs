# Integration Plan for Population, SCD, Sampling, and Balance Components

This document outlines a plan for creating an integrated pipeline that combines:
1. Population generation
2. SCD (Severe Chronic Disease) identification
3. Sampling/matching
4. Covariate balance checking

## Current Component Overview

### 1. Population Generation
- Implemented in `src/algorithm/population.rs` and `src/commands/population/handler.rs`
- Takes BEF (population register) and MFR (medical birth register) data
- Outputs a population dataset as a Parquet file containing:
  - Personal IDs (PNR)
  - Birth dates
  - Parent IDs
  - Family IDs

### 2. SCD Identification (Population SCD)
- Implemented in `src/algorithm/population_scd.rs` and `src/commands/population_scd/handler.rs`
- Takes population data and LPR (hospital register) data
- Identifies children with severe chronic diseases
- Outputs:
  - Full population with SCD indicators as a Parquet file
  - SCD children subset as a Parquet file

### 3. Sampling/Matching
- Basic implementation in `src/algorithm/sampler.rs` and `src/algorithm/matching.rs`
- Sampling: Takes a set of record batches and samples a specific number of rows
- Matching: Pairs cases with controls based on matching criteria (birth date, etc.)

### 4. Covariate Balance Checking
- Implemented in `src/algorithm/balance.rs`
- Takes case and control data
- Calculates balance metrics (standardized differences)
- Outputs a balance report

## Integration Steps

### Step 1: Create a New Integrated Command

Add a new command called `StudyDesign` that will orchestrate the full pipeline. This will require:

1. Creating a new command module in `src/commands/study_design/`
2. Defining a configuration struct for the integrated command
3. Implementing a handler function

```rust
// src/commands/study_design/config.rs
pub struct StudyDesignCommandConfig {
    // Population inputs
    pub bef_path: PathBuf,
    pub mfr_path: PathBuf,
    
    // SCD inputs
    pub lpr_data_path: PathBuf,
    pub include_lpr2: bool,
    pub include_lpr3: bool,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    
    // Matching parameters
    pub matching_ratio: usize,  // e.g., 1:4 matching would be 4
    pub birth_date_window_days: i64,
    pub parent_birth_date_window_days: i64,
    pub require_both_parents: bool,
    pub require_same_gender: bool,
    
    // Output paths
    pub output_dir: PathBuf,
    
    // Population filtering
    pub birth_inclusion_start_year: i32,
    pub birth_inclusion_end_year: i32,
}
```

### Step 2: Implement the Handler Function

Create a handler function that orchestrates the full pipeline in `src/commands/study_design/handler.rs`:

```rust
pub fn handle_study_design_command(config: &StudyDesignCommandConfig) -> Result<()> {
    // Step 1: Generate population
    info!("Step 1: Generating Population");
    let population_config = PopulationCommandConfig {
        bef_path: config.bef_path.clone(),
        mfr_path: config.mfr_path.clone(),
        output_dir: config.output_dir.join("01_population"),
        birth_inclusion_start_year: config.birth_inclusion_start_year,
        birth_inclusion_end_year: config.birth_inclusion_end_year,
    };
    
    // Create population output directory
    std::fs::create_dir_all(&population_config.output_dir)?;
    
    // Generate population data
    handle_population_command(&population_config)?;
    
    // The generated population file path
    let population_path = population_config.output_dir.join("population.parquet");
    
    // Step 2: Identify SCD in population
    info!("Step 2: Identifying SCD in Population");
    let population_scd_config = PopulationScdCommandConfig {
        population_path: population_path.clone(),
        lpr_data_path: config.lpr_data_path.clone(),
        output_dir: config.output_dir.join("02_scd"),
        include_lpr2: config.include_lpr2,
        include_lpr3: config.include_lpr3,
        start_date: config.start_date,
        end_date: config.end_date,
    };
    
    // Create SCD output directory
    std::fs::create_dir_all(&population_scd_config.output_dir)?;
    
    // Process SCD
    handle_population_scd_command(&population_scd_config)?;
    
    // The SCD children file path
    let scd_children_path = population_scd_config.output_dir.join("scd_children.parquet");
    
    // Step 3: Sample Controls and Match with Cases
    info!("Step 3: Matching Cases with Controls");
    
    // Load SCD children (cases)
    let scd_children = load_parquet_file(&scd_children_path)?;
    
    // Load full population data
    let population_data = load_parquet_file(&population_path)?;
    
    // Extract controls (non-SCD children) from population
    let controls = extract_controls(&population_data)?;
    
    // Create matching criteria
    let criteria = MatchingCriteria {
        birth_date_window_days: config.birth_date_window_days,
        parent_birth_date_window_days: config.parent_birth_date_window_days,
        require_both_parents: config.require_both_parents,
        require_same_gender: config.require_same_gender,
    };
    
    // Perform matching
    let matching_output_dir = config.output_dir.join("03_matching");
    std::fs::create_dir_all(&matching_output_dir)?;
    let (case_data, control_data) = perform_matching(
        &scd_children, 
        &controls, 
        &criteria, 
        config.matching_ratio,
        &matching_output_dir,
    )?;
    
    // Step 4: Check Covariate Balance
    info!("Step 4: Checking Covariate Balance");
    let balance_dir = config.output_dir.join("04_balance");
    std::fs::create_dir_all(&balance_dir)?;
    
    let balance_report = calculate_balance(&[case_data], &[control_data])?;
    
    // Generate balance report
    let report_path = balance_dir.join("balance_report.csv");
    generate_balance_report(&report_path.to_string_lossy(), &balance_report)?;
    
    // Print summary
    info!("Study Design Pipeline Completed Successfully");
    info!("Balance Report Summary:");
    info!(" - Total Covariates: {}", balance_report.summary.total_covariates);
    info!(" - Imbalanced Covariates: {}", balance_report.summary.imbalanced_covariates);
    info!(" - Max Standardized Difference: {:.4}", balance_report.summary.max_standardized_difference);
    info!(" - Mean Absolute Standardized Difference: {:.4}", balance_report.summary.mean_absolute_standardized_difference);
    
    Ok(())
}
```

### Step 3: Add Missing Helper Functions

Implement helper functions needed for the integration:

```rust
/// Extract controls (non-SCD children) from the population data
fn extract_controls(population_data: &RecordBatch) -> Result<RecordBatch> {
    // Get the is_scd column
    let is_scd_idx = population_data
        .schema()
        .index_of("is_scd")
        .map_err(|e| IdsError::Data(format!("is_scd column not found: {e}")))?;
    
    let is_scd_col = population_data.column(is_scd_idx);
    let is_scd_array = is_scd_col
        .as_any()
        .downcast_ref::<BooleanArray>()
        .ok_or_else(|| IdsError::Data("is_scd column is not a boolean array".to_string()))?;
    
    // Create a mask for rows where is_scd is false
    let mask = BooleanArray::from(
        (0..is_scd_array.len())
            .map(|i| {
                if is_scd_array.is_null(i) {
                    None
                } else {
                    Some(!is_scd_array.value(i))  // Note the negation here
                }
            })
            .collect::<Vec<Option<bool>>>()
    );
    
    // Apply the mask to all columns
    let mut filtered_columns = Vec::with_capacity(population_data.num_columns());
    for col in population_data.columns() {
        let filtered_col = arrow::compute::filter(col, &mask)
            .map_err(|e| IdsError::Data(format!("Failed to filter column: {e}")))?;
        filtered_columns.push(filtered_col);
    }
    
    // Create filtered batch
    let filtered_batch = RecordBatch::try_new(population_data.schema(), filtered_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))?;
    
    Ok(filtered_batch)
}

/// Perform matching between cases and controls
fn perform_matching(
    cases: &RecordBatch,
    controls: &RecordBatch,
    criteria: &MatchingCriteria,
    matching_ratio: usize,
    output_dir: &Path,
) -> Result<(RecordBatch, RecordBatch)> {
    // Convert cases and controls to the format needed for matching
    // This requires extracting PNR and birth date from the record batches
    let case_pairs = extract_pnr_and_birth_date(cases)?;
    let control_pairs = extract_pnr_and_birth_date(controls)?;
    
    // Create matcher with the given criteria
    let matcher = Matcher::new(criteria.clone());
    
    // Set match date to today
    let match_date = chrono::Local::now().naive_local().date();
    
    // For each case, find multiple controls based on matching_ratio
    let mut matched_cases = Vec::new();
    let mut matched_controls = Vec::new();
    
    // Implementation would depend on how we want to handle the matching ratio
    // This is a simplified approach - a more sophisticated one might be needed
    for i in 0..case_pairs.len() {
        let case_pnr = case_pairs[i].0.clone();
        let case_birth_date = case_pairs[i].1;
        
        // Find eligible controls
        let eligible_controls = find_eligible_controls(
            &case_pnr,
            case_birth_date,
            &control_pairs,
            criteria,
        )?;
        
        if eligible_controls.is_empty() {
            log::warn!("No eligible controls found for case {}", case_pnr.value());
            continue;
        }
        
        // Select up to matching_ratio controls randomly
        let mut rng = rand::thread_rng();
        let num_to_select = std::cmp::min(matching_ratio, eligible_controls.len());
        let selected_indices: Vec<_> = eligible_controls
            .choose_multiple(&mut rng, num_to_select)
            .collect();
        
        // Add the case and selected controls to the matched sets
        matched_cases.push(find_record_by_pnr(cases, &case_pnr)?);
        
        for &idx in &selected_indices {
            let control_pnr = &control_pairs[idx].0;
            matched_controls.push(find_record_by_pnr(controls, control_pnr)?);
        }
    }
    
    // Combine matched cases into a single RecordBatch
    let matched_cases_batch = combine_record_batches(&matched_cases)?;
    
    // Combine matched controls into a single RecordBatch
    let matched_controls_batch = combine_record_batches(&matched_controls)?;
    
    // Save matched cases and controls
    save_batch_as_parquet(&matched_cases_batch, &output_dir.join("matched_cases.parquet"))?;
    save_batch_as_parquet(&matched_controls_batch, &output_dir.join("matched_controls.parquet"))?;
    
    // Return the matched data
    Ok((matched_cases_batch, matched_controls_batch))
}

/// Extract PNR and birth date pairs from a RecordBatch
fn extract_pnr_and_birth_date(batch: &RecordBatch) -> Result<Vec<(Pnr, NaiveDate)>> {
    let pnr_idx = batch
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found: {e}")))?;
    
    let birth_date_idx = batch
        .schema()
        .index_of("FOED_DAG")
        .map_err(|e| IdsError::Data(format!("FOED_DAG column not found: {e}")))?;
    
    let pnr_col = batch.column(pnr_idx);
    let birth_date_col = batch.column(birth_date_idx);
    
    let pnr_array = pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;
    
    let mut pairs = Vec::new();
    
    for i in 0..batch.num_rows() {
        if pnr_array.is_null(i) {
            continue;
        }
        
        let pnr_str = pnr_array.value(i);
        let pnr = Pnr::from(pnr_str);
        
        if let Some(date) = date_utils::extract_date_from_array(birth_date_col.as_ref(), i) {
            pairs.push((pnr, date));
        }
    }
    
    Ok(pairs)
}

/// Find a record by PNR in a RecordBatch
fn find_record_by_pnr(batch: &RecordBatch, pnr: &Pnr) -> Result<RecordBatch> {
    let pnr_idx = batch
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found: {e}")))?;
    
    let pnr_col = batch.column(pnr_idx);
    let pnr_array = pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;
    
    let mut row_idx = None;
    for i in 0..pnr_array.len() {
        if !pnr_array.is_null(i) && pnr_array.value(i) == pnr.value() {
            row_idx = Some(i);
            break;
        }
    }
    
    if let Some(idx) = row_idx {
        // Create a new RecordBatch with just this row
        let mut filtered_columns = Vec::with_capacity(batch.num_columns());
        
        for col in batch.columns() {
            let array = arrow::compute::filter(
                col,
                &BooleanArray::from(vec![if idx < col.len() { Some(true) } else { None }]),
            )
            .map_err(|e| IdsError::Data(format!("Failed to filter column: {e}")))?;
            
            filtered_columns.push(array);
        }
        
        let filtered_batch = RecordBatch::try_new(batch.schema(), filtered_columns)
            .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))?;
        
        Ok(filtered_batch)
    } else {
        Err(IdsError::Data(format!("PNR {} not found in batch", pnr.value())))
    }
}

/// Find eligible controls for a case
fn find_eligible_controls(
    case_pnr: &Pnr,
    case_birth_date: NaiveDate,
    controls: &[(Pnr, NaiveDate)],
    criteria: &MatchingCriteria,
) -> Result<Vec<usize>> {
    let mut eligible_indices = Vec::new();
    
    for (idx, (control_pnr, control_birth_date)) in controls.iter().enumerate() {
        // Skip if case and control are the same person
        if case_pnr.value() == control_pnr.value() {
            continue;
        }
        
        // Check birth date window
        let diff = (*control_birth_date - case_birth_date).num_days().abs();
        if diff > criteria.birth_date_window_days {
            continue;
        }
        
        // Additional criteria checks would go here
        // (gender, parents, etc.)
        
        eligible_indices.push(idx);
    }
    
    Ok(eligible_indices)
}
```

### Step 4: Update CLI to Support the Integrated Command

Update the CLI to support the new integrated command:

```rust
// Add to src/cli/commands.rs

/// Study design command handler
pub struct StudyDesignCommand {
    // Same fields as StudyDesignCommandConfig
}

impl CommandHandler for StudyDesignCommand {
    fn execute(&self) -> Result<()> {
        Console::print_header("Running Integrated Study Design Pipeline");
        
        // Create config and execute
        let config = StudyDesignCommandConfig {
            // Map fields from command to config
        };
        
        crate::commands::study_design::handle_study_design_command(&config)
    }
}

// Add to Commands enum
enum Commands {
    // Existing commands...
    
    /// Run the full study design pipeline
    StudyDesign(StudyDesignArgs),
}

// Define arguments
#[derive(Args)]
struct StudyDesignArgs {
    // Population inputs
    #[clap(short, long)]
    bef: PathBuf,
    
    #[clap(short, long)]
    mfr: PathBuf,
    
    // SCD inputs
    #[clap(short, long)]
    lpr: PathBuf,
    
    #[clap(long, default_value = "true")]
    include_lpr2: bool,
    
    #[clap(long, default_value = "true")]
    include_lpr3: bool,
    
    #[clap(long)]
    start_date: Option<String>,
    
    #[clap(long)]
    end_date: Option<String>,
    
    // Matching parameters
    #[clap(long, default_value = "4")]
    matching_ratio: usize,
    
    #[clap(long, default_value = "30")]
    birth_window: i64,
    
    #[clap(long, default_value = "365")]
    parent_birth_window: i64,
    
    #[clap(long, default_value = "false")]
    require_both_parents: bool,
    
    #[clap(long, default_value = "true")]
    require_same_gender: bool,
    
    // Output
    #[clap(short, long)]
    output: PathBuf,
    
    // Population filtering
    #[clap(long, default_value = "1995")]
    start_year: i32,
    
    #[clap(long, default_value = "2018")]
    end_year: i32,
}

// Update the match statement in run()
match cli.command {
    // Existing commands...
    
    Commands::StudyDesign(args) => {
        // Parse dates
        let start_date = args.start_date.map(|date_str| {
            chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .unwrap_or_else(|_| panic!("Invalid start date format. Expected YYYY-MM-DD"))
        });
        
        let end_date = args.end_date.map(|date_str| {
            chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .unwrap_or_else(|_| panic!("Invalid end date format. Expected YYYY-MM-DD"))
        });
        
        let command = StudyDesignCommand {
            bef_path: args.bef,
            mfr_path: args.mfr,
            lpr_data_path: args.lpr,
            include_lpr2: args.include_lpr2,
            include_lpr3: args.include_lpr3,
            start_date,
            end_date,
            matching_ratio: args.matching_ratio,
            birth_date_window_days: args.birth_window,
            parent_birth_date_window_days: args.parent_birth_window,
            require_both_parents: args.require_both_parents,
            require_same_gender: args.require_same_gender,
            output_dir: args.output,
            birth_inclusion_start_year: args.start_year,
            birth_inclusion_end_year: args.end_year,
        };
        
        command.execute()
    }
}
```

## Enhancements for Future Versions

1. **Improved Matching Algorithm**:
   - Add support for more complex matching criteria
   - Implement propensity score matching
   - Add support for exact matching on specific variables

2. **Balance Optimization**:
   - Implement iterative balance optimization algorithms
   - Allow for specific covariate weighting

3. **Progress Reporting**:
   - Add progress bars and detailed logging for long-running steps
   - Generate comprehensive HTML reports with visualizations

4. **Parallelization**:
   - Optimize large data processing with better parallelization
   - Implement chunked processing for memory-intensive operations

5. **Data Export**:
   - Add export options for statistical software (R, Stata, SAS)
   - Implement anonymization for data sharing

## Usage Example

```bash
ids-rs study-design \
  --bef /path/to/bef/data \
  --mfr /path/to/mfr/data \
  --lpr /path/to/lpr/data \
  --output /path/to/output \
  --start-year 2000 \
  --end-year 2020 \
  --matching-ratio 4 \
  --birth-window 30 \
  --parent-birth-window 365 \
  --require-same-gender
```

This will run the complete pipeline and produce a directory structure with:
- `/01_population/`: Population data
- `/02_scd/`: SCD analysis results
- `/03_matching/`: Matched case-control pairs
- `/04_balance/`: Balance assessment reports