# Implementation Plan for LPR2/LPR3 Improvement

This document outlines a step-by-step plan to implement four key improvements to our LPR processing based on analysis of the reference implementation.

## 1. Enhance the SCD Algorithm ✅

### Files Modified:
- `src/algorithm/scd.rs`
- `src/model/icd10/diagnosis_pattern.rs` (new)

### Implementation Completed:
1. **Extended Pattern Matching System** ✅
   - Created `DiagnosisPattern` struct with support for both prefix and regex-based pattern matching
   - Implemented normalization for diagnosis codes with proper handling of formats
   - Added comprehensive test coverage

2. **Updated Diagnosis Matching Logic** ✅
   - Implemented fast-path prefix matching with cached lookups
   - Added regex pattern matching for complex cases
   - Improved error handling and pattern validation

3. **Added Detailed SCD Patterns** ✅
   - Implemented specific regex patterns for detailed diagnosis matching
   - Separated basic and detailed patterns for better organization
   - Added comprehensive coverage of disease categories

## 2. Improve Secondary Diagnosis Handling ✅

### Files Modified:
- `src/algorithm/lpr.rs`
- `src/algorithm/secondary_diagnosis.rs` (new)

### Implementation Completed:
1. **Created Structured Secondary Diagnosis Type** ✅
   - Implemented `SecondaryDiagnosis` struct with code, type, and weight
   - Added support for Arrow's List and Struct types
   - Created proper Arrow serialization/deserialization

2. **Updated Diagnosis Processing Function** ✅
   - Implemented weighted scoring based on diagnosis types
   - Added filtering for primary/secondary diagnoses
   - Optimized for memory usage and performance

3. **Updated Integration Functions** ✅
   - Modified LPR2/LPR3 integration to use structured diagnoses
   - Ensured backward compatibility with existing code
   - Added proper nullability handling

## 3. Add ICD-10 Chapter Classification ✅

### Files Created:
- `src/model/icd10/mod.rs`

### Files Modified:
- `src/model/mod.rs`
- `src/algorithm/lpr.rs`

### Implementation Completed:
1. **Created ICD-10 Chapter Model** ✅
   - Implemented full 22-chapter WHO ICD-10 classification
   - Added comprehensive mapping from codes to chapters
   - Added descriptive chapter information

2. **Modified LPR Processing to Include Chapter** ✅
   - Added chapter information to record schema
   - Implemented code-to-chapter mapping during processing
   - Ensured proper handling of edge cases

## 4. Expand Data Integration ✅

### Files Created:
- `src/registry/dod.rs` (Death register)
- `src/registry/dodsaarsag.rs` (Death cause register)
- `src/registry/vnds.rs` (Migration register)
- `src/schema/dod.rs`
- `src/schema/dodsaarsag.rs`
- `src/schema/vnds.rs`
- `src/algorithm/population_integration.rs`

### Files Modified:
- `src/registry/mod.rs`
- `src/algorithm/mod.rs`
- `src/algorithm/population.rs`
- `src/schema/mod.rs`
- `src/model/icd10/diagnosis_pattern.rs` (Updated to handle Danish D-prefixed codes)

### Implementation Completed:

1. **Created DOD (Death) Registry Schema** ✅
   ```rust
   // In src/schema/dod.rs
   pub fn dod_schema() -> Schema {
       Schema::new(vec![
           Field::new("PNR", DataType::Utf8, false),
           Field::new("DODDATO", DataType::Utf8, true),
       ])
   }
   
   pub fn dod_standardized_schema() -> Schema {
       Schema::new(vec![
           Field::new("PNR", DataType::Utf8, false),
           Field::new("DEATH_DATE", DataType::Date32, true),
       ])
   }
   ```

2. **Created DOD Registry Loader** ✅
   ```rust
   // In src/registry/dod.rs
   pub struct DodRegister;

   impl RegisterLoader for DodRegister {
       fn get_register_name(&self) -> &'static str {
           "dod"
       }
       
       fn load(&self, base_path: &str, pnr_filter: Option<&HashSet<String>>) -> Result<Vec<RecordBatch>> {
           // Implementation with proper date parsing and standardization
           // ...
       }
   }
   ```

3. **Created VNDS (Migration) Registry Schema and Loader** ✅
   ```rust
   // In src/schema/vnds.rs
   pub fn vnds_schema() -> Schema {
       Schema::new(vec![
           Field::new("PNR", DataType::Utf8, false),
           Field::new("INDUD_KODE", DataType::Utf8, true),  // Migration code
           Field::new("HAEND_DATO", DataType::Utf8, true),  // Event date
       ])
   }
   
   pub fn vnds_standardized_schema() -> Schema {
       Schema::new(vec![
           Field::new("PNR", DataType::Utf8, false),
           Field::new("MIGRATION_TYPE", DataType::Utf8, true),  // "IN" or "OUT"
           Field::new("MIGRATION_DATE", DataType::Date32, true),  // Standardized date
       ])
   }
   ```

4. **Created Death Cause Registry Schema and Loader** ✅
   ```rust
   // In src/schema/dodsaarsag.rs
   pub fn dodsaarsag_schema() -> Schema {
       Schema::new(vec![
           Field::new("PNR", DataType::Utf8, false),
           Field::new("C_AARSAG", DataType::Utf8, true),  // Cause of death code
           Field::new("C_TILSTAND", DataType::Utf8, true),  // Condition code
       ])
   }
   
   pub fn dodsaarsag_standardized_schema() -> Schema {
       Schema::new(vec![
           Field::new("PNR", DataType::Utf8, false),
           Field::new("DEATH_CAUSE", DataType::Utf8, true),  // Normalized cause code
           Field::new("DEATH_CONDITION", DataType::Utf8, true),  // Normalized condition
           Field::new("DEATH_CAUSE_CHAPTER", DataType::Utf8, true),  // ICD-10 chapter
       ])
   }
   ```

5. **Updated Module Exports** ✅
   ```rust
   // In src/registry/mod.rs
   pub mod dod;
   pub mod dodsaarsag;
   pub mod vnds;

   pub use dod::DodRegister;
   pub use dodsaarsag::DodsaarsagRegister;
   pub use vnds::VndsRegister;
   ```

6. **Implemented Population Integration** ✅
   ```rust
   // In population.rs
   pub struct PopulationConfig {
       // Existing fields
       pub birth_inclusion_start_year: i32,
       pub birth_inclusion_end_year: i32,
       pub include_death_data: bool,
       pub include_death_cause_data: bool,
       pub include_migration_data: bool,
   }

   // In population_integration.rs
   pub fn integrate_population_data(
       population_data: &RecordBatch,
       death_data: Option<&[RecordBatch]>,
       death_cause_data: Option<&[RecordBatch]>,
       migration_data: Option<&[RecordBatch]>,
       config: &PopulationConfig,
   ) -> Result<RecordBatch> {
       // Implementation with proper integration of all data sources
   }
   ```

7. **Added Support for Danish ICD-10 Code Format** ✅
   ```rust
   // In src/model/icd10/diagnosis_pattern.rs
   pub fn normalize_diagnosis_code(code: &str) -> Option<NormalizedDiagnosis> {
       // ...
       // Handle Danish-specific ICD-10 prefixes (D-prefixed codes)
       // In Danish healthcare, codes are often prefixed with a D, e.g., "DA10" for "A10"
       if clean_code.len() >= 4 && clean_code.starts_with('D') {
           let second_char = clean_code.chars().nth(1).unwrap_or('X');
           if second_char.is_ascii_alphabetic() {
               // This is likely a Danish D-prefixed code (e.g., DA10)
               // Remove the D prefix for standardization
               clean_code = clean_code[1..].to_string();
           }
       }
       // ...
   }
   ```

## Implementation Progress

### Phase 1: Improve Core Processing ✅
1. Improved Secondary Diagnosis Handling ✅
   - Implemented structured diagnosis representation
   - Added weighted scoring system for different diagnosis types
   - Created Arrow-compatible type system for serialization

2. Added ICD-10 Chapter Classification ✅
   - Implemented complete WHO ICD-10 chapter mapping
   - Integrated chapter information into LPR data
   - Added chapter descriptions and code ranges

### Phase 2: Enhance SCD Algorithm ✅
3. Enhanced SCD Algorithm ✅
   - Implemented sophisticated diagnosis pattern matching
   - Added support for both prefix and regex-based patterns
   - Improved organization of disease categories

### Phase 3: Add New Data Sources ✅
4. Expand Data Integration ✅
   - Death register integration ✅
   - Death cause register integration ✅
   - Migration register integration ✅
   - Support for Danish ICD-10 code format ✅

### Total Timeline: 
- Completed All Phases (5 weeks)

## Technical Challenges Resolved
- Successfully updated code to work with latest Arrow API (v55.0)
- Fixed ListArray and StructArray creation with proper offset buffers
- Resolved memory management issues with shared pointers
- Addressed nullability handling in complex structure types
- Implemented proper Danish ICD-10 code format handling (D-prefixed codes)
- Created standardized schemas and loaders for additional registers
- Integrated population data with multiple health and demographic registers

## Next Steps
- Add robust testing for all new functionality
- Benchmark performance with large datasets
- Add documentation and examples
- Consider adding more detailed analysis capabilities for combined data

By implementing these improvements, we've systematically enhanced our LPR2/LPR3 processing capabilities to match or exceed the reference implementation while maintaining a modular, maintainable codebase.
