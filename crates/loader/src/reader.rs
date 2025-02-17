use crate::schema;
use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;
use std::path::Path;
use types::error::IdsError;

pub trait DataReader {
    fn read_batches(&self, path: &Path, schema: &Schema) -> Result<Vec<RecordBatch>, IdsError>;
    fn read_akm(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError>;
    fn read_bef(&self, year: i32, quarter: Option<i32>) -> Result<Vec<RecordBatch>, IdsError>;
    fn read_ind(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError>;
    fn read_uddf(&self, period: &str) -> Result<Vec<RecordBatch>, IdsError>;
    fn read_family(&self) -> Result<Vec<RecordBatch>, IdsError>;
}

pub struct FileReader {
    base_path: String,
}

impl FileReader {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

impl DataReader for FileReader {
    fn read_batches(&self, path: &Path, schema: &Schema) -> Result<Vec<RecordBatch>, IdsError> {
        crate::parquet::read_parquet(path, Some(schema))
    }

    fn read_akm(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError> {
        let path = Path::new(&self.base_path)
            .join("akm")
            .join(format!("{}.parquet", year));
        self.read_batches(&path, &schema::akm_schema())
    }

    fn read_bef(&self, year: i32, quarter: Option<i32>) -> Result<Vec<RecordBatch>, IdsError> {
        let filename = match quarter {
            Some(q) => format!("{}{:02}.parquet", year, q * 3),
            None => format!("{}.parquet", year),
        };
        let path = Path::new(&self.base_path).join("bef").join(filename);
        self.read_batches(&path, &schema::bef_schema())
    }

    fn read_ind(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError> {
        let path = Path::new(&self.base_path)
            .join("ind")
            .join(format!("{}.parquet", year));
        self.read_batches(&path, &schema::ind_schema())
    }

    fn read_uddf(&self, period: &str) -> Result<Vec<RecordBatch>, IdsError> {
        let path = Path::new(&self.base_path)
            .join("uddf")
            .join(format!("{}.parquet", period));
        self.read_batches(&path, &schema::uddf_schema())
    }

    fn read_family(&self) -> Result<Vec<RecordBatch>, IdsError> {
        let path = Path::new(&self.base_path).join("family.parquet");
        self.read_batches(&path, &schema::family_schema())
    }
}
