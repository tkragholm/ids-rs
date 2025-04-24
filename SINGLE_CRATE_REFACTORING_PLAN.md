# Single Crate Refactoring Plan for IDS-RS - Update 4

## Current Status

We've made excellent progress implementing the single crate refactoring:

✅ Created the base directory structure  
✅ Implemented the error handling system  
✅ Added core date and string utilities  
✅ Implemented PNR handling and validation  
✅ Created the model types (Pnr, FamilyRelations, Covariate)  
✅ Implemented the Store interface with Memory and Arrow backends  
✅ Added CLI scaffolding with console output  
✅ Implemented algorithm scaffolding (matching, balance, statistics)  
✅ Created registry and schema module structure  
✅ Updated the CLI commands for clap v4  
✅ Fixed the Arrow store implementation  
✅ Added registry loader scaffolding for AKM, BEF, IND, and UDDF  
✅ Created schema definitions for all data sources  
✅ Implemented parquet reading and filtering with PNR support  
✅ Implemented parallel file loading with rayon  
✅ Added RegisterLoader trait with dynamic dispatch support  
✅ Implemented date filtering and year column extraction transformations  
✅ Added categorical value mapping and numeric scaling transformations  
✅ Implemented postal code region grouping functionality  
✅ Implemented CLI command handlers for sample and balance functions  
✅ Added record sampling functionality  
✅ Implemented balance calculation for record batches

## Next Steps

### 3. Add Tests (1-2 days)

1. Unit tests for core utilities:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_date_utilities() {
           let date = "2022-01-15".parse_date().unwrap();
           assert_eq!(date.year(), 2022);
           assert_eq!(date.month(), 1);
           assert_eq!(date.day(), 15);
       }

       #[test]
       fn test_pnr_validation() {
           assert!(Pnr::validate("1501701234").is_ok());
           assert!(Pnr::validate("invalid").is_err());
       }
   }
   ```

2. Integration tests for data loading and processing:
   ```rust
   #[test]
   fn test_akm_registry_loader() {
       let records = AkmRegister::load("test_data/akm", None).unwrap();
       assert!(!records.is_empty());
       
       // Check schema is correct
       let schema = records[0].schema();
       assert!(schema.field_with_name("PNR").is_ok());
       assert!(schema.field_with_name("DATE").is_ok());
   }
   ```

### 4. Finalize CLI Implementation (Completed ✅)

1. Completed the sample command implementation:
   ```rust
   impl CommandHandler for SampleCommand {
       fn execute(&self) -> Result<()> {
           Console::print_header("Sampling Data");
           Console::print_key_value("Input", &self.input_path);
           Console::print_key_value("Output", &self.output_path);
           Console::print_key_value("Samples", &self.sample_count.to_string());

           // Load data from the specified registry
           Console::print_info("Loading registry data...");
           let registry = crate::registry::registry_from_path(&self.input_path)?;
           let records = registry.load(&self.input_path, None)?;
           
           Console::print_info(&format!("Loaded {} record batches", records.len()));
           
           // Sample from the loaded data
           Console::print_info(&format!("Sampling {} records...", self.sample_count));
           let sampled = crate::core::sampler::sample_records(&records, self.sample_count, None)?;
           
           // Save to the output path
           Console::print_info(&format!("Writing sampled data to {}", self.output_path));
           crate::core::sampler::write_parquet(&self.output_path, &sampled)?;
           
           Console::print_success("Sampling completed");
           Ok(())
       }
   }
   ```

2. Completed the balance command implementation:
   ```rust
   impl CommandHandler for BalanceCommand {
       fn execute(&self) -> Result<()> {
           Console::print_header("Checking Balance");
           Console::print_key_value("Cases", &self.case_path);
           Console::print_key_value("Controls", &self.control_path);
           Console::print_key_value("Report", &self.report_path);

           // Load case and control data
           Console::print_info("Loading case data...");
           let case_records = crate::algorithm::balance::load_records(&self.case_path)?;
           Console::print_info(&format!("Loaded {} case record batches", case_records.len()));
           
           Console::print_info("Loading control data...");
           let control_records = crate::algorithm::balance::load_records(&self.control_path)?;
           Console::print_info(&format!("Loaded {} control record batches", control_records.len()));
           
           // Calculate balance metrics
           Console::print_info("Calculating balance metrics...");
           let balance_report = crate::algorithm::balance::calculate_balance(&case_records, &control_records)?;
           
           // Generate report
           Console::print_info(&format!("Generating report at {}", self.report_path));
           crate::algorithm::balance::generate_balance_report(&self.report_path, &balance_report)?;
           
           // Print summary
           Console::print_info("Balance Check Summary:");
           Console::print_key_value("Total Covariates", &balance_report.summary.total_covariates.to_string());
           Console::print_key_value("Imbalanced Covariates", &balance_report.summary.imbalanced_covariates.to_string());
           Console::print_key_value("Max Standardized Difference", 
               &format!("{:.4}", balance_report.summary.max_standardized_difference));
           Console::print_key_value("Mean Absolute Standardized Difference", 
               &format!("{:.4}", balance_report.summary.mean_absolute_standardized_difference));
           
           Console::print_success("Balance check completed");
           Ok(())
       }
   }
   ```

### 5. Documentation and Examples (1 day)

1. Add comprehensive documentation:
   - Module-level documentation
   - Function-level documentation
   - Example code

2. Create usage examples:
   ```rust
   /// Example: Loading AKM data for a specific set of PNRs
   pub fn example_load_akm() -> Result<()> {
       // Create a filter with specific PNRs
       let mut pnr_filter = HashSet::new();
       pnr_filter.insert("1501701234".to_string());
       
       // Load data
       let records = AkmRegister::load("/path/to/akm", Some(&pnr_filter))?;
       println!("Loaded {} record batches", records.len());
       
       Ok(())
   }
   ```

## Enhanced Features

1. **Parallel Processing**: Add rayon-based parallel processing for data loading and transformations
   ```rust
   pub fn load_parquet_files_parallel(
       dir: &Path,
       schema: Option<&Schema>,
       pnr_filter: Option<&HashSet<String>>,
   ) -> Result<Vec<RecordBatch>> {
       // ... 
       // Using rayon for parallel processing
       let all_batches: Vec<Result<Vec<RecordBatch>>> = parquet_files
           .par_iter()
           .map(|path| read_parquet(path, schema, pnr_filter))
           .collect();
       // ...
   }
   ```

2. **Data Quality Checks**: Add validation and quality checking for data loading
   ```rust
   pub fn validate_record_batch(batch: &RecordBatch, schema: &Schema) -> Result<()> {
       // Check for required fields
       // Validate data types
       // Check for value ranges
       Ok(())
   }
   ```

## Migration Plan

1. **Parallel Development**: Continue developing the new single-crate structure while keeping the existing crates
2. **Component Testing**: Test each component of the new structure independently
3. **Functional Migration**: Gradually move functionality from the old crates to the new structure
4. **Integration Testing**: Test the full system with real data
5. **Switchover**: When confident, remove the old crates and use only the new structure

## Benefits of This Approach

1. **Clean architecture**: Unified codebase with clear module boundaries
2. **Elimination of duplication**: All utility code in one place
3. **Simplified dependency management**: Single cargo.toml file
4. **Easy refactoring**: Moving code between modules without changing APIs
5. **Improved discoverability**: Everything in one place makes it easier to navigate
6. **Better integration**: Components work together more seamlessly
7. **Performance improvements**: Optimized data loading and processing

## Timeline

- Current progress: ~95% complete
- Estimated completion: 1-2 days (mainly documentation and tests if required)
- Total refactoring time: ~2 weeks