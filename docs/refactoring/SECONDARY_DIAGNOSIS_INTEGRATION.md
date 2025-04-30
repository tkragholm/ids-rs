# Secondary Diagnosis Integration Plan

This document outlines how to integrate the `secondary_diagnosis.rs` functionality into the appropriate location within the new module structure. Currently, secondary diagnosis handling is in a standalone file but should be integrated with the diagnosis or LPR code.

## Current Implementation

The `secondary_diagnosis.rs` file contains:

1. A `SecondaryDiagnosis` struct that represents a secondary diagnosis with code, type, and weight
2. A function to create Arrow arrays for lists of secondary diagnoses
3. A function to process secondary diagnoses with weights based on diagnosis type

This functionality is primarily used by the LPR module for processing complex health data.

## Integration Approach

We'll move the secondary diagnosis functionality into the new `health/diagnosis/secondary.rs` file, but reorganize it to better integrate with the rest of the diagnosis processing code.

### Step 1: Create the new file structure

```
src/algorithm/health/diagnosis/
├── mod.rs         # Common diagnosis functionality
├── pattern.rs     # Moved from existing code
├── secondary.rs   # Secondary diagnosis handling
└── scd.rs         # SCD algorithm implementation
```

### Step 2: Update the secondary.rs file

```rust
//! Secondary diagnosis handling
//!
//! This module provides structures and functions for enhanced handling of secondary diagnoses
//! from health registry data, particularly the Danish National Patient Registry (LPR).

use arrow::array::{ArrayRef, Float32Array, ListArray, StringArray, StructArray};
use arrow::buffer::{BooleanBuffer, NullBuffer, OffsetBuffer};
use arrow::datatypes::{DataType, Field};
use std::sync::Arc;

/// Represents a secondary diagnosis with its code, type, and weight
#[derive(Debug, Clone)]
pub struct SecondaryDiagnosis {
    /// The diagnosis code (ICD-10)
    pub code: String,
    /// The diagnosis type (e.g., "B" for bi-diagnosis)
    pub diagnosis_type: String,
    /// The relative weight/importance of this diagnosis (0.0-1.0)
    pub weight: f32,
}

impl SecondaryDiagnosis {
    /// Create a new secondary diagnosis
    pub fn new(code: String, diagnosis_type: String, weight: Option<f32>) -> Self {
        let weight = weight.unwrap_or_else(|| {
            // Calculate weight based on diagnosis type
            match diagnosis_type.as_str() {
                "B" => 0.8, // Higher weight for bi-diagnoses
                "C" => 0.7, // Complications
                "G" => 0.6, // Grundmorbus
                "H" => 0.5, // Referring diagnosis
                "M" => 0.4, // Temporary diagnosis
                _ => 0.3,   // Default weight for unknown types
            }
        });

        Self {
            code,
            diagnosis_type,
            weight,
        }
    }
    
    /// Convert a secondary diagnosis to a structured format
    pub fn normalize(&self) -> Self {
        // For future expansion - can be used to normalize codes, etc.
        Self {
            code: self.code.clone(),
            diagnosis_type: self.diagnosis_type.clone(),
            weight: self.weight,
        }
    }
}

/// Process secondary diagnoses with weights based on diagnosis type
///
/// This function converts raw diagnosis tuples into structured secondary diagnoses
/// with appropriate weights and types.
pub fn process_secondary_diagnoses(diagnoses: &[(String, String)]) -> Vec<SecondaryDiagnosis> {
    diagnoses
        .iter()
        .filter(|(_, diag_type)| diag_type != "A") // Filter out primary diagnoses
        .map(|(diag, diag_type)| {
            SecondaryDiagnosis::new(diag.clone(), diag_type.clone(), None)
        })
        .collect()
}

/// Create Arrow array for a list of secondary diagnoses
pub fn create_secondary_diagnoses_array(
    diagnoses_list: &[Option<Vec<SecondaryDiagnosis>>],
) -> ArrayRef {
    // Implementation (unchanged from original)
    // ...
}

/// Create the Arrow schema field for secondary diagnoses
///
/// This function returns the field definition for a list of secondary diagnoses
/// that can be included in a schema.
pub fn create_secondary_diagnoses_field() -> Field {
    // Define secondary diagnosis struct fields
    let secondary_diag_fields = vec![
        Field::new("code", DataType::Utf8, false),
        Field::new("diagnosis_type", DataType::Utf8, true),
        Field::new("weight", DataType::Float32, true),
    ];
    
    // Create struct field
    let secondary_diag_struct = Field::new(
        "item", 
        DataType::Struct(secondary_diag_fields.into()), 
        false
    );
    
    // Create list field with the struct as item type
    Field::new(
        "secondary_diagnoses", 
        DataType::List(Arc::new(secondary_diag_struct)), 
        true
    )
}
```

### Step 3: Update the LPR module to use the new location

In the new `health/lpr.rs`, we would update the imports and usage:

```rust
// Old import
use crate::algorithm::secondary_diagnosis::{SecondaryDiagnosis, process_secondary_diagnoses, create_secondary_diagnoses_array};

// New import
use crate::algorithm::health::diagnosis::secondary::{
    SecondaryDiagnosis, 
    process_secondary_diagnoses, 
    create_secondary_diagnoses_array,
    create_secondary_diagnoses_field
};

// And use the new helper function in schema creation:
fn create_integrated_schema() -> Schema {
    let secondary_diag_list = create_secondary_diagnoses_field();
    
    // Add field for diagnosis chapter (based on ICD-10)
    Schema::new(vec![
        Field::new("patient_id", DataType::Utf8, true),
        Field::new("primary_diagnosis", DataType::Utf8, true),
        secondary_diag_list,
        // ... other fields
    ])
}
```

## Benefits of This Approach

1. **Better Encapsulation**: The `SecondaryDiagnosis` type now has methods for creation and normalization
2. **Clearer API**: Added a new helper function for creating the field definition, which was previously embedded in the LPR code
3. **Logical Organization**: Secondary diagnoses are now part of the diagnosis module, which is where they conceptually belong
4. **Improved Maintainability**: The secondary diagnosis code is now co-located with related diagnosis processing code

## Migration Path

1. Create the new file and copy the enhanced implementation
2. Update the LPR code to use the new location
3. Ensure backward compatibility via re-exports in the module files
4. Eventually deprecate the old location after all code has been updated