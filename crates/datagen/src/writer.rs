use crate::error::DataGenError;
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use std::fs::File;
use std::path::Path;

pub struct ParquetWriter;

impl ParquetWriter {
    pub fn write_batch(batch: RecordBatch, output_path: &Path) -> Result<(), DataGenError> {
        let file = File::create(output_path)?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), None)?;

        writer.write(&batch)?;
        writer.close()?;

        Ok(())
    }
}
