use crate::error::DataGenError;
use arrow::array::{Date32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use types::family::FamilyRelations;

use indicatif::ProgressBar;
use std::path::Path;
use std::sync::Arc;

impl super::RegisterGenerator {
    pub(crate) fn generate_family(&mut self) -> Result<(), DataGenError> {
        let pb = self
            .progress
            .add(ProgressBar::new(self.config.total_records as u64));
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap(),
        );
        pb.set_message("Generating family records...");

        // Define schema
        let schema = Arc::new(Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("BIRTH_DATE", DataType::Date32, false),
            Field::new("FATHER_ID", DataType::Utf8, true),
            Field::new("FATHER_BIRTH_DATE", DataType::Date32, true),
            Field::new("MOTHER_ID", DataType::Utf8, true),
            Field::new("MOTHER_BIRTH_DATE", DataType::Date32, true),
            Field::new("FAMILY_ID", DataType::Utf8, true),
        ]));

        let mut records = Vec::with_capacity(self.config.total_records);

        for i in 0..self.config.total_records {
            let (child, (father, mother)) = self
                .get_family(i)
                .ok_or_else(|| DataGenError::Generation("Family not found in PNR pool".into()))?;

            let (child_birth, child_pnr) = child;
            let (father_birth, father_pnr) = father;
            let (mother_birth, mother_pnr) = mother;

            records.push(FamilyRelations {
                pnr: child_pnr,
                birth_date: child_birth,
                father_id: Some(father_pnr),
                father_birth_date: Some(father_birth),
                mother_id: Some(mother_pnr),
                mother_birth_date: Some(mother_birth),
                family_id: Some(format!("F{:08}", i)),
            });

            pb.inc(1);
        }

        // Convert to Arrow arrays
        let batch = RecordBatch::try_new(
            schema,
            vec![
                Arc::new(StringArray::from(
                    records.iter().map(|r| r.pnr.as_str()).collect::<Vec<_>>(),
                )),
                Arc::new(Date32Array::from(
                    records
                        .iter()
                        .map(|r| Self::date_to_days_since_epoch(r.birth_date))
                        .collect::<Vec<_>>(),
                )),
                Arc::new(StringArray::from(
                    records
                        .iter()
                        .map(|r| r.father_id.as_deref())
                        .collect::<Vec<_>>(),
                )),
                Arc::new(Date32Array::from(
                    records
                        .iter()
                        .map(|r| r.father_birth_date.map(Self::date_to_days_since_epoch))
                        .collect::<Vec<_>>(),
                )),
                Arc::new(StringArray::from(
                    records
                        .iter()
                        .map(|r| r.mother_id.as_deref())
                        .collect::<Vec<_>>(),
                )),
                Arc::new(Date32Array::from(
                    records
                        .iter()
                        .map(|r| r.mother_birth_date.map(Self::date_to_days_since_epoch))
                        .collect::<Vec<_>>(),
                )),
                Arc::new(StringArray::from(
                    records
                        .iter()
                        .map(|r| r.family_id.as_deref())
                        .collect::<Vec<_>>(),
                )),
            ],
        )?;

        // Write to Parquet file
        let output_path = Path::new(&self.config.output_dir).join("family.parquet");

        crate::writer::ParquetWriter::write_batch(batch, &output_path)?;

        pb.finish_with_message("Family generation completed");
        Ok(())
    }
}
