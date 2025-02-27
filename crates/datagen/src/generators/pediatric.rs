use crate::error::DataGenError;
use chrono::{Duration, NaiveDate};
use csv::Writer;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::path::Path;

impl super::RegisterGenerator {
    /// Generate a synthetic pediatric dataset for case-control sampling
    /// 
    /// This function creates a CSV file containing records with:
    /// - Personal ID (PNR)
    /// - Birth date
    /// - Treatment date (for cases only)
    /// - Mother's birth date
    /// - Father's birth date
    pub fn generate_pediatric(&mut self, filename: &str) -> Result<(), DataGenError> {
        let pb = self
            .progress
            .add(ProgressBar::new(self.config.total_records as u64));
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap(),
        );
        pb.set_message("Generating pediatric records...");

        // Create parent directory if needed
        if let Some(parent) = Path::new(filename).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create CSV writer
        let mut writer = Writer::from_path(filename)
            .map_err(|e| DataGenError::Generation(format!("Failed to create CSV file: {e}")))?;

        // Write header
        let header = ["pnr", "bday", "treatment_date", "mother_bday", "father_bday"];
        writer
            .write_record(header)
            .map_err(|e| DataGenError::Generation(format!("Failed to write header: {e}")))?;

        // Define study parameters
        let study_params = PediatricStudyParams::new();
        
        // Precompute treatment indices for better performance
        let treatment_indices = self.select_treatment_indices();

        // Process each record
        for i in 0..self.config.total_records {
            // Get person and parents from pool
            let (child, (father, mother)) = self
                .get_family(i)
                .ok_or_else(|| DataGenError::Generation("Family not found in PNR pool".into()))?;

            let (birth_date, pnr) = child;
            let (father_birth, _) = father;
            let (mother_birth, _) = mother;

            // Generate treatment date for cases
            let treatment_date = if treatment_indices.contains(&i) {
                self.generate_treatment_date(birth_date, &study_params)
            } else {
                "NA".to_string()
            };

            // Write record to CSV
            let record = [
                &pnr,
                &birth_date.format("%Y-%m-%d").to_string(),
                &treatment_date,
                &mother_birth.format("%Y-%m-%d").to_string(),
                &father_birth.format("%Y-%m-%d").to_string(),
            ];
            
            writer
                .write_record(record)
                .map_err(|e| DataGenError::Generation(format!("Failed to write record: {e}")))?;

            pb.inc(1);
        }

        writer
            .flush()
            .map_err(|e| DataGenError::Generation(format!("Failed to flush writer: {e}")))?;

        pb.finish_with_message("Pediatric data generation completed");
        Ok(())
    }
    
    /// Generate a random treatment date between birth and 6 years of age
    /// within the study period
    fn generate_treatment_date(&mut self, birth_date: NaiveDate, params: &PediatricStudyParams) -> String {
        let treatment_start = birth_date.max(params.study_start);
        let treatment_end = (birth_date + Duration::days(params.max_age_days)).min(params.study_end);

        if treatment_start <= treatment_end {
            let treatment_days = self
                .rng
                .gen_range(0..=(treatment_end - treatment_start).num_days());
            let date = treatment_start + Duration::days(treatment_days);
            date.format("%Y-%m-%d").to_string()
        } else {
            "NA".to_string()
        }
    }
    
    /// Randomly select indices for treatment cases
    fn select_treatment_indices(&mut self) -> HashSet<usize> {
        let mut indices: Vec<usize> = (0..self.config.total_records).collect();
        indices.shuffle(&mut self.rng);
        indices.into_iter().take(self.config.treatment_cases).collect()
    }
}

/// Parameters for the pediatric study
struct PediatricStudyParams {
    study_start: NaiveDate,
    study_end: NaiveDate,
    max_age_days: i64,
}

impl PediatricStudyParams {
    fn new() -> Self {
        Self {
            study_start: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            study_end: NaiveDate::from_ymd_opt(2018, 12, 31).unwrap(),
            max_age_days: 6 * 365, // 6 years in days
        }
    }
}
