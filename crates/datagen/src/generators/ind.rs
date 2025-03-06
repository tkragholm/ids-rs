use crate::{error::DataGenError, models::IndRecord};
use arrow::array::{Float64Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;

use indicatif::ProgressBar;
use rand::Rng;
use rand_distr::{Distribution, LogNormal};
use std::path::Path;
use std::sync::Arc;

impl super::RegisterGenerator {
    pub(crate) fn generate_ind(&mut self) -> Result<(), DataGenError> {
        let pb = self.progress.add(ProgressBar::new(
            (self.config.end_year - self.config.start_year + 1) as u64,
        ));
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .map_err(|e| DataGenError::Generation(format!("Failed to set progress bar template: {}", e)))?,
        );
        pb.set_message("Generating IND records...");

        // Create income distribution (log-normal for realistic income distribution)
        let income_dist = LogNormal::new(12.5, 0.5)
            .map_err(|e| DataGenError::Generation(format!("Failed to create income distribution: {}", e)))?;

        let schema = Arc::new(Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("BESKST13", DataType::Int32, true),
            Field::new("LOENMV_13", DataType::Float64, true),
            Field::new("PERINDKIALT_13", DataType::Float64, true),
            Field::new("PRE_SOCIO", DataType::Int32, true),
        ]));

        for year in self.config.start_year..=self.config.end_year {
            let mut records = Vec::with_capacity(self.config.total_records);

            for i in 0..self.config.total_records {
                let (_, pnr) = self.get_person(i).ok_or_else(|| {
                    DataGenError::Generation("Person not found in PNR pool".into())
                })?;

                let base_income = income_dist.sample(&mut self.rng);
                let wage_income = if self.rng.random_bool(0.8) {
                    Some(base_income)
                } else {
                    None
                };

                let total_income = wage_income.map(|w| w * (1.0 + self.rng.random_range(0.0..0.3))); // Add 0-30% other income

                let employment_status = if wage_income.is_some() {
                    Some(self.rng.random_range(110..=320)) // Employment status codes
                } else {
                    Some(self.rng.random_range(321..=500)) // Unemployment/pension codes
                };

                records.push(IndRecord {
                    pnr: pnr.clone(),
                    beskst13: employment_status,
                    loenmv_13: wage_income,
                    perindkialt_13: total_income,
                    pre_socio: Some(self.rng.random_range(1..=5)),
                });
            }

            let batch = RecordBatch::try_new(
                schema.clone(),
                vec![
                    Arc::new(StringArray::from(
                        records.iter().map(|r| r.pnr.as_str()).collect::<Vec<_>>(),
                    )),
                    Arc::new(Int32Array::from(
                        records.iter().map(|r| r.beskst13).collect::<Vec<_>>(),
                    )),
                    Arc::new(Float64Array::from(
                        records.iter().map(|r| r.loenmv_13).collect::<Vec<_>>(),
                    )),
                    Arc::new(Float64Array::from(
                        records.iter().map(|r| r.perindkialt_13).collect::<Vec<_>>(),
                    )),
                    Arc::new(Int32Array::from(
                        records.iter().map(|r| r.pre_socio).collect::<Vec<_>>(),
                    )),
                ],
            )?;

            let output_path = Path::new(&self.config.output_dir)
                .join("ind")
                .join(format!("{year}.parquet"));

            crate::writer::ParquetWriter::write_batch(batch, &output_path)?;

            pb.inc(1);
        }

        pb.finish_with_message("IND generation completed");
        Ok(())
    }
}
