//! Population SCD (Severe Chronic Disease) algorithm
//!
//! This module provides functionality to identify children in a population
//! who have been diagnosed with severe chronic diseases.

use arrow::array::{Array, BooleanArray, Date32Array, StringArray};
use arrow::compute::filter;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::algorithm::health::lpr::{process_lpr_data, LprConfig};
use crate::algorithm::health::diagnosis::scd::{apply_scd_algorithm, ScdConfig, ScdResult};
use crate::error::{IdsError, Result};

/// Configuration for Population SCD analysis
pub struct PopulationScdConfig {
    /// Whether to include LPR2 data
    pub include_lpr2: bool,
    /// Whether to include LPR3 data
    pub include_lpr3: bool,
    /// Start date for filtering health data (inclusive)
    pub start_date: Option<NaiveDate>,
    /// End date for filtering health data (inclusive)
    pub end_date: Option<NaiveDate>,
    /// Diagnosis columns to check for SCD codes
    pub diagnosis_columns: Vec<String>,
    /// Patient ID column in LPR data
    pub patient_id_column: String,
    /// Date column in LPR data
    pub date_column: String,
    /// PNR column in population data
    pub population_pnr_column: String,
}

impl Default for PopulationScdConfig {
    fn default() -> Self {
        Self {
            include_lpr2: true,
            include_lpr3: true,
            start_date: None,
            end_date: None,
            diagnosis_columns: vec![
                "primary_diagnosis".to_string(),
                "secondary_diagnosis".to_string(),
            ],
            patient_id_column: "patient_id".to_string(),
            date_column: "admission_date".to_string(),
            population_pnr_column: "PNR".to_string(),
        }
    }
}

/// Result of the Population SCD analysis
#[derive(Debug, Clone)]
pub struct PopulationScdResult {
    /// Total children in the population
    pub total_children: usize,
    /// Number of children with SCD
    pub scd_children: usize,
    /// SCD percentage
    pub scd_percentage: f64,
    /// Disease category counts
    pub category_counts: HashMap<String, usize>,
}

/// Identify children in a population who have SCD
pub fn identify_scd_in_population(
    population_data: &RecordBatch,
    lpr_data: &RecordBatch,
    config: &PopulationScdConfig,
) -> Result<(RecordBatch, PopulationScdResult)> {
    // Step 1: Apply SCD algorithm to health data
    let scd_config = ScdConfig {
        diagnosis_columns: config.diagnosis_columns.clone(),
        date_column: config.date_column.clone(),
        patient_id_column: config.patient_id_column.clone(),
    };

    log::info!("Applying SCD algorithm to {} health records...", lpr_data.num_rows());
    let scd_results = apply_scd_algorithm(lpr_data, &scd_config)?;
    log::info!("SCD analysis complete: {} patient records", scd_results.len());

    // Step 2: Create map of patient ID to SCD result with preallocated capacity
    // Use a more efficient hash map implementation for string keys
    log::debug!("Creating index for fast SCD lookup");
    let estimated_capacity = scd_results.len();
    
    // Use rustc_hash::FxHashMap which is faster for string keys than the standard HashMap
    use rustc_hash::FxHashMap;
    let mut scd_map: FxHashMap<&str, &ScdResult> = FxHashMap::with_capacity_and_hasher(
        estimated_capacity, 
        Default::default()
    );
    
    // Store references to avoid cloning strings
    for result in &scd_results {
        scd_map.insert(&result.patient_id, result);
    }
    
    // Step 3: Extract PNR from population data
    let pnr_col_idx = population_data
        .schema()
        .index_of(&config.population_pnr_column)
        .map_err(|e| {
            IdsError::Data(format!(
                "PNR column not found in population data: {e}"
            ))
        })?;

    let pnr_col = population_data.column(pnr_col_idx);
    
    // Handle the StringArray conversion
    let mut converted_array_holder = None;
    
    // First, try direct downcast
    let pnr_array = if let Some(array) = pnr_col.as_any().downcast_ref::<StringArray>() {
        array
    } else {
        // If direct downcast fails, try to convert using Arrow cast
        log::warn!("Attempting generic conversion to StringArray for column {} with type {:?}", 
                  config.population_pnr_column, pnr_col.data_type());
        
        // Create a cast result that will be owned
        let cast_result = arrow::compute::cast(pnr_col, &arrow::datatypes::DataType::Utf8)
            .map_err(|e| IdsError::Data(format!(
                "Failed to convert PNR column to StringArray: {e}"
            )))?;
        
        log::info!("Successfully converted column {} to StringArray using Arrow cast", 
                  config.population_pnr_column);
        
        // Keep the converted array alive for the duration of this function
        converted_array_holder = Some(cast_result);
        
        // Get a StringArray reference from the cast result
        converted_array_holder.as_ref().unwrap().as_any().downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data(format!(
                "Failed to convert {} to StringArray after casting", 
                config.population_pnr_column
            )))?
    };

    // Step 4: Match population records with SCD results
    log::debug!("Matching population records with SCD results");
    let num_rows = population_data.num_rows();
    let mut is_scd = Vec::with_capacity(num_rows);
    let mut first_scd_date = Vec::with_capacity(num_rows);
    let mut disease_categories = HashMap::new();

    // Get all unique disease categories from SCD results
    let all_categories: HashSet<String> = scd_results
        .iter()
        .flat_map(|r| r.disease_categories.keys().cloned())
        .collect();

    // Initialize category vectors
    for category in &all_categories {
        disease_categories.insert(category.clone(), Vec::with_capacity(num_rows));
    }

    // Counter for children with SCD
    let mut scd_children_count = 0;
    let mut category_counts: HashMap<String, usize> = HashMap::with_capacity(all_categories.len());
    for category in &all_categories {
        category_counts.insert(category.clone(), 0);
    }

    // Process data in parallel using Rayon
    use rayon::prelude::*;
    
    // Create intermediate vectors that will be processed in parallel
    let chunk_size = 10000;
    let num_chunks = num_rows.div_ceil(chunk_size);
    
    // Define the chunk structure to be processed in parallel
    struct ChunkResult {
        is_scd: Vec<Option<bool>>,
        first_scd_date: Vec<Option<i32>>,
        category_values: HashMap<String, Vec<Option<bool>>>,
        scd_children_count: usize,
        category_counts: HashMap<String, usize>,
    }
    
    // Process chunks in parallel
    let chunk_results: Vec<ChunkResult> = (0..num_chunks)
        .into_par_iter()
        .map(|chunk_index| {
            let start_idx = chunk_index * chunk_size;
            let end_idx = (start_idx + chunk_size).min(num_rows);
            let chunk_size = end_idx - start_idx;
            
            // Initialize chunk result data
            let mut chunk_is_scd = Vec::with_capacity(chunk_size);
            let mut chunk_first_scd_date = Vec::with_capacity(chunk_size);
            let mut chunk_category_values: HashMap<String, Vec<Option<bool>>> = HashMap::new();
            let mut chunk_scd_children_count = 0;
            let mut chunk_category_counts: HashMap<String, usize> = HashMap::new();
            
            // Initialize category vectors and counts
            for category in &all_categories {
                chunk_category_values.insert(category.clone(), Vec::with_capacity(chunk_size));
                chunk_category_counts.insert(category.clone(), 0);
            }
            
            // Process each child in this chunk
            for i in start_idx..end_idx {
                if pnr_array.is_null(i) {
                    // Add nulls for missing PNR
                    chunk_is_scd.push(None);
                    chunk_first_scd_date.push(None);
                    for category_vec in chunk_category_values.values_mut() {
                        category_vec.push(None);
                    }
                    continue;
                }

                let pnr = pnr_array.value(i);
                
                // Look up SCD result for this PNR (O(1) lookup with HashMap)
                if let Some(scd_result) = scd_map.get(pnr) {
                    chunk_is_scd.push(Some(scd_result.is_scd));
                    
                    if scd_result.is_scd {
                        chunk_scd_children_count += 1;
                        
                        // Update first SCD date
                        if let Some(date) = scd_result.first_scd_date {
                            let days = (date.signed_duration_since(
                                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
                            ).num_days()) as i32;
                            chunk_first_scd_date.push(Some(days));
                        } else {
                            chunk_first_scd_date.push(None);
                        }
                        
                        // Update disease categories
                        for (category, has_disease) in &scd_result.disease_categories {
                            let category_vec = chunk_category_values.get_mut(category).unwrap();
                            category_vec.push(Some(*has_disease));
                            
                            if *has_disease {
                                *chunk_category_counts.get_mut(category).unwrap() += 1;
                            }
                        }
                    } else {
                        // Not SCD
                        chunk_first_scd_date.push(None);
                        
                        // Set all categories to false
                        for category_vec in chunk_category_values.values_mut() {
                            category_vec.push(Some(false));
                        }
                    }
                } else {
                    // No SCD result for this PNR
                    chunk_is_scd.push(Some(false));
                    chunk_first_scd_date.push(None);
                    
                    // Set all categories to false
                    for category_vec in chunk_category_values.values_mut() {
                        category_vec.push(Some(false));
                    }
                }
            }
            
            // Log progress
            log::debug!("Processed chunk {}/{}, found {} SCD children", 
                       chunk_index + 1, num_chunks, chunk_scd_children_count);
            
            // Return this chunk's results
            ChunkResult {
                is_scd: chunk_is_scd,
                first_scd_date: chunk_first_scd_date,
                category_values: chunk_category_values,
                scd_children_count: chunk_scd_children_count,
                category_counts: chunk_category_counts,
            }
        })
        .collect();
    
    // Combine results from all chunks
    for chunk_result in chunk_results {
        // Extend vectors
        is_scd.extend(chunk_result.is_scd);
        first_scd_date.extend(chunk_result.first_scd_date);
        
        // Extend category vectors
        for (category, values) in chunk_result.category_values {
            disease_categories.get_mut(&category).unwrap().extend(values);
        }
        
        // Update counts
        scd_children_count += chunk_result.scd_children_count;
        
        // Update category counts
        for (category, count) in chunk_result.category_counts {
            *category_counts.get_mut(&category).unwrap() += count;
        }
    }

    // Step 5: Create a new RecordBatch with combined data
    let mut fields = population_data.schema().fields().to_vec();
    let mut columns = population_data.columns().to_vec();
    
    // Add SCD fields
    fields.push(Arc::new(Field::new("is_scd", DataType::Boolean, true)));
    fields.push(Arc::new(Field::new("first_scd_date", DataType::Date32, true)));
    
    // Add category fields
    for category in &all_categories {
        let field_name = format!("scd_category_{category}");
        fields.push(Arc::new(Field::new(&field_name, DataType::Boolean, true)));
    }
    
    // Create schema
    let schema = Arc::new(Schema::new(fields));
    
    // Add SCD columns
    columns.push(Arc::new(BooleanArray::from(is_scd)));
    columns.push(Arc::new(Date32Array::from(first_scd_date)));
    
    // Add category columns
    for category in &all_categories {
        columns.push(Arc::new(BooleanArray::from(disease_categories.get(category).unwrap().clone())));
    }
    
    // Create batch
    let result_batch = RecordBatch::try_new(schema, columns)
        .map_err(|e| IdsError::Data(format!("Error creating population SCD batch: {e}")))?;
    
    // Step 6: Create summary result
    let scd_percentage = if num_rows > 0 {
        (scd_children_count as f64 / num_rows as f64) * 100.0
    } else {
        0.0
    };
    
    let result = PopulationScdResult {
        total_children: num_rows,
        scd_children: scd_children_count,
        scd_percentage,
        category_counts,
    };
    
    Ok((result_batch, result))
}

/// Process LPR data and identify SCD children in a population
pub fn process_lpr_and_identify_scd(
    population_data: &RecordBatch,
    lpr2_adm: Option<&[RecordBatch]>,
    lpr2_diag: Option<&[RecordBatch]>,
    lpr2_bes: Option<&[RecordBatch]>,
    lpr3_kontakter: Option<&[RecordBatch]>,
    lpr3_diagnoser: Option<&[RecordBatch]>,
    config: &PopulationScdConfig,
) -> Result<(RecordBatch, PopulationScdResult)> {
    // Step 1: Process LPR data
    let lpr_config = LprConfig {
        include_lpr2: config.include_lpr2,
        include_lpr3: config.include_lpr3,
        start_date: config.start_date,
        end_date: config.end_date,
    };
    
    let processed_data = process_lpr_data(
        lpr2_adm,
        lpr2_diag,
        lpr2_bes,
        lpr3_kontakter,
        lpr3_diagnoser,
        None, // lpr3_procedurer
        &lpr_config,
    )?;
    
    log::info!("Processed LPR data: {} rows", processed_data.num_rows());
    
    // Step 2: Identify SCD in population
    identify_scd_in_population(population_data, &processed_data, config)
}

/// Extract only the children with SCD from the population
pub fn extract_scd_children(population_scd_data: &RecordBatch) -> Result<RecordBatch> {
    // Get the is_scd column
    let is_scd_idx = population_scd_data
        .schema()
        .index_of("is_scd")
        .map_err(|e| IdsError::Data(format!("is_scd column not found: {e}")))?;
    
    let is_scd_col = population_scd_data.column(is_scd_idx);
    let is_scd_array = is_scd_col
        .as_any()
        .downcast_ref::<BooleanArray>()
        .ok_or_else(|| IdsError::Data("is_scd column is not a boolean array".to_string()))?;
    
    // Create a mask for rows where is_scd is true
    let mask = BooleanArray::from(
        (0..is_scd_array.len())
            .map(|i| {
                if is_scd_array.is_null(i) {
                    None
                } else {
                    Some(is_scd_array.value(i))
                }
            })
            .collect::<Vec<Option<bool>>>()
    );
    
    // Apply the mask to all columns
    let mut filtered_columns = Vec::with_capacity(population_scd_data.num_columns());
    for col in population_scd_data.columns() {
        let filtered_col = filter(col, &mask)
            .map_err(|e| IdsError::Data(format!("Failed to filter column: {e}")))?;
        filtered_columns.push(filtered_col);
    }
    
    // Create filtered batch
    let filtered_batch = RecordBatch::try_new(population_scd_data.schema(), filtered_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))?;
    
    Ok(filtered_batch)
}