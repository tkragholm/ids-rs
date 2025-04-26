# Danish Health Data Processing System Architecture

## 1. Data Structure Overview

The system processes health data from two major versions of the Danish National Patient Registry:

### LPR2 (Legacy Format)
- Three primary components:
  - **Admissions** (`LPR_ADM`): Patient hospital admissions with dates, primary diagnosis
  - **Diagnoses** (`LPR_DIAG`): Detailed diagnosis codes linked to admissions
  - **Treatments** (`LPR_BES`): Outpatient/treatment data linked to admissions
- Key identifier: `RECNUM` links records across tables

### LPR3 (Current Format)
- Two primary components:
  - **Contacts** (`LPR3_KONTAKTER`): Patient contacts with healthcare system
  - **Diagnoses** (`LPR3_DIAGNOSER`): Detailed diagnosis codes for each contact
- Key identifier: `DW_EK_KONTAKT` links records across tables

## 2. Data Processing Pipeline

```
┌───────────────┐     ┌─────────────────┐     ┌────────────────┐
│ Data Loading  │────▶│ Data Integration│────▶│ Data           │
└───────────────┘     └─────────────────┘     │ Harmonization  │
                                              └────────┬───────┘
┌───────────────┐     ┌─────────────────┐              │
│ Analytics     │◀────│ SCD Algorithm   │◀─────────────┘
│ Generation    │     │ Application     │
└───────────────┘     └─────────────────┘
```

1. **Data Loading**: 
   - Reads Parquet files using Polars (pl) library
   - Configured through paths in service configuration

2. **Data Integration**:
   - `integrate_lpr2_components()`: Joins LPR2 tables on RECNUM
   - `integrate_lpr3_components()`: Joins LPR3 tables on DW_EK_KONTAKT

3. **Data Harmonization**:
   - `harmonize_health_data()`: Standardizes column names across LPR2 and LPR3
   - Field mappings include:
     - Patient IDs: `PNR`/`CPR` → `patient_id`
     - Diagnoses: `C_ADIAG`/`aktionsdiagnose` → `primary_diagnosis` 
     - Dates: `D_INDDTO`/`dato_start` → `admission_date`

4. **Data Combination**:
   - `combine_harmonized_data()`: Combines LPR2 and LPR3 into unified dataset
   - Handles schema matching, data type conversion, null handling

5. **SCD Algorithm Application**:
   - `apply_scd_algorithm_single()`: Applies the Severe Chronic Disease algorithm

6. **Analytics Generation**:
   - Creates various analytical views (longitudinal summaries, group analysis)

## 3. Severe Chronic Disease Algorithm

The SCD algorithm identifies patients with severe chronic diseases based on ICD-10 diagnosis codes.

### Key Components:

1. **SCD Code Definition**:
   - Comprehensive list of ~200 ICD-10 code prefixes associated with severe chronic diseases
   - Codes are organized in logical groups (e.g., D55-D61 for certain blood disorders)

2. **Diagnosis Classification**:
   - Inspects all diagnosis fields (`primary_diagnosis`, `diagnosis`, `secondary_diagnosis`)
   - Extracts and standardizes the diagnosis codes (uppercase, slice relevant portion)
   - Matches against SCD code prefixes

3. **Patient-Level Aggregation**:
   - Flags patients with any SCD diagnosis (`is_scd = true`)
   - Records earliest SCD diagnosis date (`first_scd_date`)
   - Aggregates to patient level with proper boolean handling

4. **Disease Categorization**:
   - Groups diseases into 10 medical categories:
     - Blood Disorders (D55-D61, D64-D73, D76)
     - Immune System (D80-D84, D86, D89)
     - Endocrine (E22-E27, E31, E34, E70-E85, E88)
     - Neurological (F84, G11-G13, G23-G25, etc.)
     - Cardiovascular (I27, I42-I43, I50, etc.)
     - Respiratory (J41-J45, J47, J60-J70, etc.)
     - Gastrointestinal (K50-K51, K73-K74, K86-K87, K90)
     - Musculoskeletal (M05-M09, M30-M35, M40-M43, M45-M46)
     - Renal (N01-N08, N11-N16, N18-N29)
     - Congenital (P27, Q01-Q07, Q20-Q28, etc.)

## 4. Algorithm Implementation Details

```rust
// Pseudocode for the core SCD algorithm
fn apply_scd_algorithm(
    health_data: DataFrame, 
    diagnosis_columns: Vec<String>, 
    date_column: String, 
    patient_id_column: String
) -> DataFrame {
    // Define SCD codes
    let scd_codes = vec!["D55", "D56", /* ~200 more codes */];
    
    // For each record and diagnosis column
    let mut is_scd = vec![false; health_data.len()];
    let mut first_scd_date = vec![Option<Date>::None; health_data.len()];
    
    for (i, record) in health_data.iter().enumerate() {
        for diag_col in &diagnosis_columns {
            if let Some(diagnosis) = record.get(diag_col) {
                let diag_upper = diagnosis.to_uppercase();
                let code_prefix = &diag_upper[1..4]; // Extract code prefix (e.g., "D55")
                
                if scd_codes.contains(&code_prefix) || 
                   // Additional special case handling for ranges
                   (code_prefix >= "E74" && code_prefix <= "E84") ||
                   // Other special case handling...
                {
                    is_scd[i] = true;
                    if first_scd_date[i].is_none() {
                        first_scd_date[i] = record.get(date_column);
                    }
                }
            }
        }
    }
    
    // Aggregate to patient level
    let mut result = HashMap::new();
    for (i, record) in health_data.iter().enumerate() {
        let patient_id = record.get(patient_id_column);
        let entry = result.entry(patient_id).or_insert((false, None));
        
        if is_scd[i] {
            entry.0 = true;
            if entry.1.is_none() || first_scd_date[i].unwrap() < entry.1.unwrap() {
                entry.1 = first_scd_date[i];
            }
        }
    }
    
    // Convert to DataFrame and return
    // ...
}
```

## 5. Key Architectural Components

### Service-Oriented Design
- `CohortService`: Orchestrates health data processing
- `DataService`: Handles data I/O operations
- `EventService`: Manages temporal events
- `MappingService`: Provides mapping utilities

### Data Processing Abstractions
- `BaseProcessor`: Generic data processing interface
- `LPRDiagProcessor`, `LPR3DiagnoserProcessor`: Format-specific processors

### Utility Functions
- `harmonize_health_data()`: Standardizes data schemas
- `apply_scd_algorithm_single()`: Core SCD identification logic
- `get_diagnosis_groups()`: Returns categorized ICD-10 groups

## 6. Data Flow

1. LPR2 & LPR3 data is loaded from Parquet files
2. Components are integrated within each format
3. Both formats are harmonized to a common schema
4. Combined data is processed with the SCD algorithm
5. Patient-level results are aggregated and validated
6. Analytical views are optionally generated

This architecture provides a robust framework for processing Danish health registry data and identifying patients with severe chronic diseases, with careful handling of data integrity, harmonization between formats, and comprehensive disease categorization.