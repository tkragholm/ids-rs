use crate::{error::DataGenError, models::BefRecord};
use arrow::array::{Array, Date32Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;

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
                .map_err(|e| DataGenError::Generation(format!("Failed to set progress bar template: {}", e)))?,
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
            Field::new("FAMILIE_TYPE", DataType::Int32, true),
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
                let current_date =
                    NaiveDate::from_ymd_opt(year, period.parse::<u32>().unwrap(), 1).unwrap();
                let mut records = Vec::with_capacity(self.config.total_records);

                for i in 0..self.config.total_records {
                    let (child, (father, mother)) = self.get_family(i).ok_or_else(|| {
                        DataGenError::Generation("Family not found in PNR pool".into())
                    })?;

                    let (birth_date, pnr) = child;
                    let (_father_birth, father_pnr) = father;
                    let (_mother_birth, mother_pnr) = mother;

                    // Skip if person isn't born yet
                    if birth_date > current_date {
                        continue;
                    }

                    // Generate all random values upfront
                    let gender = if self.rng.random_bool(0.5) { "M" } else { "F" };
                    let kommune = self.rng.random_range(101..=851);
                    let civil_status = match self.rng.random_range(0..4) {
                        0 => "U",
                        1 => "G",
                        2 => "F",
                        _ => "E",
                    };
                    let family_size = self.rng.random_range(1..=6);
                    let age = (current_date - birth_date).num_days() / 365;
                    let children = if age > 18 {
                        self.rng.random_range(0..4)
                    } else {
                        0
                    };

                    // Get a random statsb code according to our defined rules
                    let statsb_code = self.get_random_statsb();

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
                        familie_type: Some(self.rng.random_range(1..=10)),
                        far_id: Some(father_pnr),
                        foed_dag: birth_date,
                        koen: gender.to_string(),
                        kom: Some(kommune),
                        mor_id: Some(mother_pnr),
                        statsb: Some(statsb_code),
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

    fn get_random_statsb(&mut self) -> String {
        // Most people should be Danish (5100)
        if self.rng.random_ratio(9, 10) {
            // 90% chance of being Danish
            return "5100".to_string(); // Denmark code
        }

        // For the other 10%, choose one of these common countries
        // You can adjust this list based on your needs
        let common_codes = [
            "5120", // Sweden
            "5110", // Norway
            "5170", // Great Britain
            "5180", // Germany
            "5130", // France
            "5150", // Italy
            "5172", // Turkey
            "5436", // Iraq
            "5448", // China
            "5432", // India
            "5472", // Pakistan
        ];

        if self.rng.random_ratio(8, 10) {
            // 80% of the remaining 10% (8% total)
            common_codes[self.rng.random_range(0..common_codes.len())].to_string()
        } else {
            // For the remaining 2%, choose a completely random code
            // This is a simplified approach - in reality you might want to
            // filter out certain codes or apply other rules
            let random_code = self.rng.random_range(5100..=5999);
            random_code.to_string()
        }
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
                    .map(|r| {
                        r.bop_vfra.map(|date| {
                            date.signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
                                .num_days() as i32
                        })
                    })
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
            Arc::new(Int32Array::from(
                records.iter().map(|r| r.familie_type).collect::<Vec<_>>(),
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
                    .map(|r| {
                        r.foed_dag
                            .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
                            .num_days() as i32
                    })
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
