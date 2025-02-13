use crate::error::ArrowError;

use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ProjectionMask;

use std::fs::File;
use std::path::Path;

pub struct RegisterReader {
    base_path: String,
}

impl RegisterReader {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub fn read_parquet(
        &self,
        path: &Path,
        schema: Option<&Schema>,
    ) -> Result<Vec<RecordBatch>, ArrowError> {
        let file = File::open(path)?;

        // Create builder directly from File
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;

        // Build the reader using the provided schema or default to file schema
        let mut reader = match schema {
            Some(s) => {
                let indices: Vec<usize> = (0..s.fields().len()).collect();
                let mask = ProjectionMask::roots(builder.parquet_schema(), indices);
                builder
                    .with_batch_size(8192)
                    .with_projection(mask)
                    .build()?
            }
            None => builder.build()?,
        };

        // Collect all record batches
        let mut batches = Vec::new();
        while let Some(batch) = reader.next() {
            batches.push(batch?);
        }

        Ok(batches)
    }

    pub fn read_akm(&self, year: i32) -> Result<Vec<RecordBatch>, ArrowError> {
        let path = Path::new(&self.base_path)
            .join("akm")
            .join(format!("{}.parquet", year));

        self.read_parquet(&path, Some(&crate::schema::akm_schema()))
    }

    pub fn read_bef(
        &self,
        year: i32,
        quarter: Option<i32>,
    ) -> Result<Vec<RecordBatch>, ArrowError> {
        let filename = match quarter {
            Some(q) => format!("{}{:02}.parquet", year, q * 3),
            None => format!("{}.parquet", year),
        };

        let path = Path::new(&self.base_path).join("bef").join(filename);
        self.read_parquet(&path, Some(&crate::schema::bef_schema()))
    }

    pub fn read_ind(&self, year: i32) -> Result<Vec<RecordBatch>, ArrowError> {
        let path = Path::new(&self.base_path)
            .join("ind")
            .join(format!("{}.parquet", year));

        self.read_parquet(&path, Some(&crate::schema::ind_schema()))
    }

    pub fn read_uddf(&self, date: &str) -> Result<Vec<RecordBatch>, ArrowError> {
        let path = Path::new(&self.base_path)
            .join("uddf")
            .join(format!("{}.parquet", date));

        self.read_parquet(&path, Some(&crate::schema::uddf_schema()))
    }
}
