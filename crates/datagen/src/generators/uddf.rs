use crate::{error::DataGenError, models::UddfRecord};
use arrow::array::{Date32Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::Datelike;
use chrono::NaiveDate;
use indicatif::ProgressBar;
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

impl super::RegisterGenerator {
    /// Load the HFAUDD to ISCED mapping from the hfaudd.json file
    fn load_hfaudd_mapping() -> Result<HashMap<String, String>, DataGenError> {
        let mapping_path = Path::new("mappings/hfaudd.json");
        
        let mut file = File::open(mapping_path).map_err(|e| {
            DataGenError::Generation(format!("Failed to open hfaudd.json: {}", e))
        })?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            DataGenError::Generation(format!("Failed to read hfaudd.json: {}", e))
        })?;
        
        let mapping: HashMap<String, String> = serde_json::from_str(&contents).map_err(|e| {
            DataGenError::Generation(format!("Failed to parse hfaudd.json: {}", e))
        })?;
        
        Ok(mapping)
    }
    
    /// Get a random HFAUDD code for a given ISCED level
    fn get_random_hfaudd_for_isced_level(
        mapping: &HashMap<String, String>,
        isced_level: &str,
        rng: &mut rand::rngs::StdRng,
    ) -> Option<String> {
        // Collect all HFAUDD codes that map to the given ISCED level
        let valid_codes: Vec<String> = mapping
            .iter()
            .filter(|(_, v)| v == &isced_level)
            .map(|(k, _)| k.clone())
            .collect();
            
        if valid_codes.is_empty() {
            None
        } else {
            // Choose a random code from the valid ones
            Some(valid_codes.choose(rng).unwrap().clone())
        }
    }
    
    pub(crate) fn generate_uddf(&mut self) -> Result<(), DataGenError> {
        let pb = self.progress.add(ProgressBar::new(2)); // Only 2020 and 2022
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap(),
        );
        pb.set_message("Generating UDDF records...");

        // Load the HFAUDD mapping
        let hfaudd_mapping = Self::load_hfaudd_mapping()?;
        
        let schema = Arc::new(Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("HFAUDD", DataType::Utf8, true),
            Field::new("HF_VFRA", DataType::Date32, true),
            Field::new("HF_VTIL", DataType::Date32, true),
            Field::new("INSTNR", DataType::Int32, true),
        ]));

        // ISCED education levels
        // 1-2: Basic education
        // 3-4: Upper secondary/High school/Vocational
        // 5: Short-cycle tertiary
        // 6: Bachelor's or equivalent
        // 7: Master's or equivalent
        // 8: Doctoral or equivalent
        // 9: Missing/Unknown
        // 
        // We'll weight the distribution to create a realistic population
        let isced_levels = vec![
            ("1", 0.05),  // Low education (5%)
            ("2", 0.10),  // Basic education (10%)
            ("3", 0.35),  // High school/Vocational (35%)
            ("5", 0.15),  // Short-cycle tertiary (15%)
            ("6", 0.20),  // Bachelor's (20%)
            ("7", 0.12),  // Master's (12%)
            ("8", 0.03),  // PhD (3%)
        ];

        for year in [2020, 2022] {
            let mut records = Vec::with_capacity(self.config.total_records);

            for i in 0..self.config.total_records {
                let (birth_date, pnr) = self.get_person(i).ok_or_else(|| {
                    DataGenError::Generation("Person not found in PNR pool".into())
                })?;

                // Calculate age at the time of record
                let age = year - birth_date.year();

                // Only generate education records for people old enough
                // 90% of adults have education records, increasing probability slightly with age
                let base_chance = if age >= 25 { 0.92 } else if age >= 20 { 0.90 } else { 0.85 };
                let has_education = age >= 18 && self.rng.random_bool(base_chance);

                if has_education {
                    // Select education level based on weighted distribution
                    // Higher age slightly increases chances of higher education
                    let age_modifier = (age as f64 - 18.0) / 100.0; // Small age-based adjustment
                    
                    // Use weighted selection for education level
                    let mut cdf = 0.0;
                    let roll: f64 = self.rng.random();
                    
                    // Default to level 3 (high school) if something goes wrong
                    let mut selected_level = "3";
                    
                    for (level, weight) in &isced_levels {
                        // Adjust weights slightly based on age
                        let adjusted_weight = match *level {
                            "1" | "2" => weight * (1.0 - age_modifier), // Decrease probability with age
                            "6" | "7" | "8" => weight * (1.0 + age_modifier), // Increase probability with age
                            _ => *weight, // Keep others the same
                        };
                        
                        cdf += adjusted_weight;
                        if roll <= cdf {
                            selected_level = level;
                            break;
                        }
                    }
                    
                    // Get actual HFAUDD code from the mapping
                    if let Some(hfaudd_code) = Self::get_random_hfaudd_for_isced_level(
                        &hfaudd_mapping, 
                        selected_level,
                        &mut self.rng
                    ) {
                        // Generate completion year between age 18 and current age
                        // Adjust min age based on education level
                        let min_age = match selected_level {
                            "1" => 16,
                            "2" => 18,
                            "3" => 19,
                            "5" => 21,
                            "6" => 22,
                            "7" => 24,
                            "8" => 28,
                            _ => 18,
                        };
                        
                        // Ensure person is old enough for this education level
                        if age >= min_age {
                            let min_completion_year = birth_date.year() + min_age;
                            let completion_year = self.rng.random_range(min_completion_year..=year);
                            let completion_month = self.rng.random_range(1..=12);
                            
                            let completion_date =
                                NaiveDate::from_ymd_opt(completion_year, completion_month, 1).unwrap();

                            records.push(UddfRecord {
                                pnr: pnr.clone(),
                                hfaudd: Some(hfaudd_code),
                                hf_vfra: Some(completion_date),
                                hf_vtil: Some(completion_date + chrono::Duration::days(30)),
                                instnr: Some(self.rng.random_range(100000..=999999)),
                            });
                            continue;
                        }
                    }
                }
                
                // If we get here, either person has no education or selection failed
                records.push(UddfRecord {
                    pnr: pnr.clone(),
                    hfaudd: None,
                    hf_vfra: None,
                    hf_vtil: None,
                    instnr: None,
                });
            }

            // Create RecordBatch
            let batch = RecordBatch::try_new(
                schema.clone(),
                vec![
                    Arc::new(StringArray::from(
                        records.iter().map(|r| r.pnr.as_str()).collect::<Vec<_>>(),
                    )),
                    Arc::new(StringArray::from(
                        records
                            .iter()
                            .map(|r| r.hfaudd.as_deref())
                            .collect::<Vec<_>>(),
                    )),
                    Arc::new(Date32Array::from(
                        records
                            .iter()
                            .map(|r| r.hf_vfra.map(Self::date_to_days_since_epoch))
                            .collect::<Vec<_>>(),
                    )),
                    Arc::new(Date32Array::from(
                        records
                            .iter()
                            .map(|r| r.hf_vtil.map(Self::date_to_days_since_epoch))
                            .collect::<Vec<_>>(),
                    )),
                    Arc::new(Int32Array::from(
                        records.iter().map(|r| r.instnr).collect::<Vec<_>>(),
                    )),
                ],
            )?;

            // Write to Parquet file
            let output_path = Path::new(&self.config.output_dir)
                .join("uddf")
                .join(format!("{year}09.parquet"));

            crate::writer::ParquetWriter::write_batch(batch, &output_path)?;

            pb.inc(1);
        }

        pb.finish_with_message("UDDF generation completed");
        Ok(())
    }
}
