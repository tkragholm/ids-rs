use crate::{error::DataGenError, models::BefRecord};
use arrow::array::{Array, Date32Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::Datelike;

use indicatif::ProgressBar;
use rand::Rng;
use std::path::Path;
use std::sync::Arc;

impl super::RegisterGenerator {
    pub(crate) fn generate_bef(&mut self) -> Result<(), DataGenError> {
        let total_periods = self.calculate_bef_periods();
        let pb = self.progress.add(ProgressBar::new(total_periods));
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap(),
        );
        pb.set_message("Generating BEF records...");

        let schema = Arc::new(Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("AEGTE_ID", DataType::Utf8, true),
            Field::new("ALDER", DataType::Utf8, false),
            Field::new("ANTBOERNF", DataType::Int32, true),
            Field::new("ANTBOERNH", DataType::Int32, true),
            Field::new("ANTPERSF", DataType::Int32, true),
            Field::new("ANTPERSH", DataType::Int32, true),
            Field::new("BOP_VFRA", DataType::Date32, true),
            Field::new("CIVST", DataType::Utf8, true),
            Field::new("FAMILIE_ID", DataType::Utf8, true),
            Field::new("FAMILIE_TYPE", DataType::Utf8, true),
            Field::new("FAR_ID", DataType::Utf8, true),
            Field::new("FOED_DAG", DataType::Date32, false),
            Field::new("KOEN", DataType::Utf8, false),
            Field::new("KOM", DataType::Int32, true),
            Field::new("MOR_ID", DataType::Utf8, true),
            Field::new("STATSB", DataType::Utf8, true),
        ]));

        for year in self.config.start_year..=self.config.end_year {
            let periods = if year >= 2019 {
                vec!["03", "06", "09", "12"]
            } else {
                vec!["12"]
            };

            for period in periods {
                let mut records = Vec::with_capacity(self.config.total_records);

                for i in 0..self.config.total_records {
                    let (child, (father, mother)) = self.get_family(i).ok_or_else(|| {
                        DataGenError::Generation("Family not found in PNR pool".into())
                    })?;

                    let (birth_date, pnr) = child;
                    let (_father_birth, father_pnr) = father;
                    let (_mother_birth, mother_pnr) = mother;

                    // Generate all random values upfront
                    let gender = if self.rng.gen_bool(0.5) { "M" } else { "F" };
                    let kommune = self.rng.gen_range(101..=851);
                    let civil_status = match self.rng.gen_range(0..4) {
                        0 => "U",
                        1 => "G",
                        2 => "F",
                        _ => "E",
                    };
                    let family_size = self.rng.gen_range(1..=6);
                    let age = year - birth_date.year();
                    let children = if age > 18 {
                        self.rng.gen_range(0..4)
                    } else {
                        0
                    };

                    records.push(BefRecord {
                        pnr,
                        aegte_id: if civil_status == "G" {
                            Some(
                                self.get_person(i + 1)
                                    .map(|(_, pnr)| pnr)
                                    .unwrap_or_default(),
                            )
                        } else {
                            None
                        },
                        alder: age.to_string(),
                        antboernf: Some(children),
                        antboernh: Some(children),
                        antpersf: Some(family_size),
                        antpersh: Some(family_size),
                        bop_vfra: Some(birth_date),
                        civst: Some(civil_status.to_string()),
                        familie_id: Some(format!("F{:08}", i / (family_size as usize))),
                        familie_type: Some(format!("TYPE{}", self.rng.gen_range(1..=5))),
                        far_id: Some(father_pnr),
                        foed_dag: birth_date,
                        koen: gender.to_string(),
                        kom: Some(kommune),
                        mor_id: Some(mother_pnr),
                        statsb: Some("DNK".to_string()),
                    });
                }

                // Convert to Arrow arrays
                let batch = self.create_bef_batch(&records, &schema)?;

                // Write to Parquet file
                let output_path = Path::new(&self.config.output_dir)
                    .join("bef")
                    .join(format!("{year}{period}.parquet"));

                crate::writer::ParquetWriter::write_batch(batch, &output_path)?;

                pb.inc(1);
            }
        }

        pb.finish_with_message("BEF generation completed");
        Ok(())
    }

    fn calculate_bef_periods(&self) -> u64 {
        let mut count = 0;
        for year in self.config.start_year..=self.config.end_year {
            count += if year >= 2019 { 4 } else { 1 };
        }
        count
    }

    fn create_bef_batch(
        &self,
        records: &[BefRecord],
        schema: &Schema,
    ) -> Result<RecordBatch, DataGenError> {
        let arrays: Vec<Arc<dyn Array>> = vec![
            Arc::new(StringArray::from(
                records.iter().map(|r| r.pnr.as_str()).collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records
                    .iter()
                    .map(|r| r.aegte_id.as_deref())
                    .collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records.iter().map(|r| r.alder.as_str()).collect::<Vec<_>>(),
            )),
            Arc::new(Int32Array::from(
                records.iter().map(|r| r.antboernf).collect::<Vec<_>>(),
            )),
            Arc::new(Int32Array::from(
                records.iter().map(|r| r.antboernh).collect::<Vec<_>>(),
            )),
            Arc::new(Int32Array::from(
                records.iter().map(|r| r.antpersf).collect::<Vec<_>>(),
            )),
            Arc::new(Int32Array::from(
                records.iter().map(|r| r.antpersh).collect::<Vec<_>>(),
            )),
            Arc::new(Date32Array::from(
                records
                    .iter()
                    .map(|r| r.bop_vfra.map(Self::date_to_days_since_epoch))
                    .collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records
                    .iter()
                    .map(|r| r.civst.as_deref())
                    .collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records
                    .iter()
                    .map(|r| r.familie_id.as_deref())
                    .collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records
                    .iter()
                    .map(|r| r.familie_type.as_deref())
                    .collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records
                    .iter()
                    .map(|r| r.far_id.as_deref())
                    .collect::<Vec<_>>(),
            )),
            Arc::new(Date32Array::from(
                records
                    .iter()
                    .map(|r| Self::date_to_days_since_epoch(r.foed_dag))
                    .collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records.iter().map(|r| r.koen.as_str()).collect::<Vec<_>>(),
            )),
            Arc::new(Int32Array::from(
                records.iter().map(|r| r.kom).collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records
                    .iter()
                    .map(|r| r.mor_id.as_deref())
                    .collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(
                records
                    .iter()
                    .map(|r| r.statsb.as_deref())
                    .collect::<Vec<_>>(),
            )),
        ];

        Ok(RecordBatch::try_new(Arc::new(schema.clone()), arrays)?)
    }
}
