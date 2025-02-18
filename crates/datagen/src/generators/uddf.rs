use crate::{error::DataGenError, models::UddfRecord};
use arrow::array::{Date32Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::Datelike;
use chrono::NaiveDate;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use rand::Rng;
use std::path::Path;
use std::sync::Arc;

impl super::RegisterGenerator {
    pub(crate) fn generate_uddf(&mut self) -> Result<(), DataGenError> {
        let pb = self.progress.add(ProgressBar::new(2)); // Only 2020 and 2022
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap(),
        );
        pb.set_message("Generating UDDF records...");

        let schema = Arc::new(Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("HFAUDD", DataType::Utf8, true),
            Field::new("HF_VFRA", DataType::Date32, true),
            Field::new("HF_VTIL", DataType::Date32, true),
            Field::new("INSTNR", DataType::Int32, true),
        ]));

        // Education levels (simplified Danish education codes)
        let education_levels = vec![
            "10", "20", "25", // Basic education
            "30", "35", // High school
            "40", "45", "50", // Vocational
            "60", "65", // Bachelor
            "70", "75", // Master
            "80", // PhD
        ];

        for year in [2020, 2022] {
            let mut records = Vec::with_capacity(self.config.total_records);

            for i in 0..self.config.total_records {
                let (birth_date, pnr) = self.get_person(i).ok_or_else(|| {
                    DataGenError::Generation("Person not found in PNR pool".into())
                })?;

                let has_education = self.rng.gen_bool(0.9); // 90% have education records

                if has_education {
                    let education = education_levels.choose(&mut self.rng).unwrap();
                    let completion_year = self.rng.gen_range(birth_date.year() + 18..=year);
                    let completion_month = self.rng.gen_range(1..=12);

                    let completion_date =
                        NaiveDate::from_ymd_opt(completion_year, completion_month, 1).unwrap();

                    records.push(UddfRecord {
                        pnr: pnr.clone(),
                        hfaudd: Some(education.to_string()),
                        hf_vfra: Some(completion_date),
                        hf_vtil: Some(completion_date + chrono::Duration::days(30)),
                        instnr: Some(self.rng.gen_range(100000..=999999)),
                    });
                } else {
                    records.push(UddfRecord {
                        pnr: pnr.clone(),
                        hfaudd: None,
                        hf_vfra: None,
                        hf_vtil: None,
                        instnr: None,
                    });
                }
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
                            .map(|r| r.hf_vfra.map(|d| Self::date_to_days_since_epoch(d)))
                            .collect::<Vec<_>>(),
                    )),
                    Arc::new(Date32Array::from(
                        records
                            .iter()
                            .map(|r| r.hf_vtil.map(|d| Self::date_to_days_since_epoch(d)))
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
                .join(format!("{}09.parquet", year));

            crate::writer::ParquetWriter::write_batch(batch, &output_path)?;

            pb.inc(1);
        }

        pb.finish_with_message("UDDF generation completed");
        Ok(())
    }
}
