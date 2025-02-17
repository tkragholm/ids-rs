mod parquet;
mod reader;
mod schema;

pub use reader::{DataReader, FileReader};
pub use types::{
    arrow_utils::ArrowStore, error::IdsError, family::FamilyRelations, models::*,
    snapshot::CovariateSnapshot,
};

// Create a loader trait instead of implementing directly on ArrowStore
pub trait StoreLoader {
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError>;
}

// Implement the loader trait
pub struct ParquetLoader;

impl ParquetLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_from_path(&self, base_path: String) -> Result<ArrowStore, IdsError> {
        // Call the trait implementation
        <Self as StoreLoader>::load_from_path(base_path)
    }
}

impl StoreLoader for ParquetLoader {
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError> {
        let reader = FileReader::new(base_path);
        let mut store = ArrowStore::new();

        // Load family relations
        if let Ok(family_batches) = reader.read_family() {
            store.load_family_relations(family_batches)?;
        }

        // Load data from all years
        for year in 2000..=2022 {
            if let Ok(batches) = reader.read_akm(year) {
                store.add_akm_data(year, batches);
            }
            if let Ok(batches) = reader.read_ind(year) {
                store.add_ind_data(year, batches);
            }
        }

        // Load BEF data (quarterly from 2019)
        for year in 2000..=2018 {
            if let Ok(batches) = reader.read_bef(year, None) {
                store.add_bef_data(format!("{}", year), batches);
            }
        }
        for year in 2019..=2023 {
            for quarter in 1..=4 {
                if let Ok(batches) = reader.read_bef(year, Some(quarter)) {
                    store.add_bef_data(format!("{}{:02}", year, quarter * 3), batches);
                }
            }
        }

        // Load UDDF data
        for period in ["202009", "202209"] {
            if let Ok(batches) = reader.read_uddf(period) {
                store.add_uddf_data(period.to_string(), batches);
            }
        }

        Ok(store)
    }
}
