use crate::{error::DataGenError, models::AkmRecord};
use arrow::array::{Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;

use indicatif::ProgressBar;
use rand::Rng;
use std::path::Path;
use std::sync::Arc;

impl super::RegisterGenerator {
    pub(crate) fn generate_akm(&mut self) -> Result<(), DataGenError> {
        let pb = self.progress.add(ProgressBar::new(
            (self.config.end_year - self.config.start_year + 1) as u64,
        ));
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .map_err(|e| DataGenError::Generation(format!("Failed to set progress bar template: {}", e)))?,
        );
        pb.set_message("Generating AKM records...");

        // Define schema
        let schema = Arc::new(Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("SOCIO", DataType::Int32, true),
            Field::new("SOCIO02", DataType::Int32, true),
            Field::new("SOCIO13", DataType::Int32, false),
            Field::new("CPRTJEK", DataType::Int32, true),
            Field::new("CPRTYPE", DataType::Int32, true),
            Field::new("VERSION", DataType::Utf8, true),
            Field::new("SENR", DataType::Utf8, true),
        ]));

        // Generate data for each year
        for year in self.config.start_year..=self.config.end_year {
            let mut records = Vec::with_capacity(self.config.total_records);

            for i in 0..self.config.total_records {
                let (_, pnr) = self.get_person(i).ok_or_else(|| {
                    DataGenError::Generation("Person not found in PNR pool".into())
                })?;

                // Get all random values at once
                let socio13 = self.rng.random_range(1..=5);
                let has_socio = self.rng.random_bool(0.9);
                let cprtjek = self.rng.random_range(0..10);
                let cprtype = self.rng.random_range(0..5);

                records.push(AkmRecord {
                    pnr,
                    socio: if has_socio { Some(socio13) } else { None },
                    socio02: if has_socio { Some(socio13) } else { None },
                    socio13,
                    cprtjek: Some(cprtjek),
                    cprtype: Some(cprtype),
                    version: Some(format!("V{year}")),
                    senr: Some(format!("S{i:06}")),
                });
            }

            // Convert to Arrow arrays
            let pnr_array =
                StringArray::from(records.iter().map(|r| r.pnr.as_str()).collect::<Vec<_>>());
            let socio_array = Int32Array::from(records.iter().map(|r| r.socio).collect::<Vec<_>>());
            let socio02_array =
                Int32Array::from(records.iter().map(|r| r.socio02).collect::<Vec<_>>());
            let socio13_array =
                Int32Array::from(records.iter().map(|r| r.socio13).collect::<Vec<_>>());
            let cprtjek_array =
                Int32Array::from(records.iter().map(|r| r.cprtjek).collect::<Vec<_>>());
            let cprtype_array =
                Int32Array::from(records.iter().map(|r| r.cprtype).collect::<Vec<_>>());
            let version_array = StringArray::from(
                records
                    .iter()
                    .map(|r| r.version.as_deref())
                    .collect::<Vec<_>>(),
            );
            let senr_array = StringArray::from(
                records
                    .iter()
                    .map(|r| r.senr.as_deref())
                    .collect::<Vec<_>>(),
            );

            // Create RecordBatch
            let batch = RecordBatch::try_new(
                schema.clone(),
                vec![
                    Arc::new(pnr_array),
                    Arc::new(socio_array),
                    Arc::new(socio02_array),
                    Arc::new(socio13_array),
                    Arc::new(cprtjek_array),
                    Arc::new(cprtype_array),
                    Arc::new(version_array),
                    Arc::new(senr_array),
                ],
            )?;

            // Write to Parquet file
            let output_path = Path::new(&self.config.output_dir)
                .join("akm")
                .join(format!("{year}.parquet"));

            crate::writer::ParquetWriter::write_batch(batch, &output_path)?;

            pb.inc(1);
        }

        pb.finish_with_message("AKM generation completed");
        Ok(())
    }
}
