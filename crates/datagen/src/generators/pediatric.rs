use crate::error::DataGenError;
use chrono::{Duration, NaiveDate};
use csv::Writer;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::path::Path;

impl super::RegisterGenerator {
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

        let mut writer = Writer::from_path(filename)
            .map_err(|e| DataGenError::Generation(format!("Failed to create CSV file: {e}")))?;

        // Write header
        writer
            .write_record([
                "pnr",
                "bday",
                "treatment_date",
                "mother_bday",
                "father_bday",
            ])
            .map_err(|e| DataGenError::Generation(format!("Failed to write header: {e}")))?;

        // Define the study period (2000-2018)
        let study_start = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let study_end = NaiveDate::from_ymd_opt(2018, 12, 31).unwrap();
        let earliest_birth = NaiveDate::from_ymd_opt(1995, 1, 1).unwrap();
        let _latest_birth = study_end;

        let _birth_range_days = (study_end - earliest_birth).num_days() as i32;

        // Create treatment indices
        let mut indices: Vec<usize> = (0..self.config.total_records).collect();
        indices.shuffle(&mut self.rng);
        let treatment_indices: HashSet<usize> = indices
            .into_iter()
            .take(self.config.treatment_cases)
            .collect();

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
                let treatment_start = birth_date.max(study_start);
                let treatment_end = (birth_date + Duration::days(6 * 365)).min(study_end);

                if treatment_start <= treatment_end {
                    let treatment_days = self
                        .rng
                        .gen_range(0..=(treatment_end - treatment_start).num_days());
                    let date = treatment_start + Duration::days(treatment_days);
                    date.format("%Y-%m-%d").to_string()
                } else {
                    "NA".to_string()
                }
            } else {
                "NA".to_string()
            };

            writer
                .write_record([
                    &pnr,
                    &birth_date.format("%Y-%m-%d").to_string(),
                    &treatment_date,
                    &mother_birth.format("%Y-%m-%d").to_string(),
                    &father_birth.format("%Y-%m-%d").to_string(),
                ])
                .map_err(|e| DataGenError::Generation(format!("Failed to write record: {e}")))?;

            pb.inc(1);
        }

        writer
            .flush()
            .map_err(|e| DataGenError::Generation(format!("Failed to flush writer: {e}")))?;

        pb.finish_with_message("Pediatric data generation completed");
        Ok(())
    }
}
