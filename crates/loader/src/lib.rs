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
        let reader = FileReader::new(base_path.clone());
        let mut store = ArrowStore::new();

        log::info!("Loading data from path: {}", base_path);

        // Load family relations
        match reader.read_family() {
            Ok(family_batches) => {
                log::info!("Loaded {} family relation batches", family_batches.len());
                store.load_family_relations(family_batches)?;
            }
            Err(e) => log::warn!("Failed to load family relations: {}", e),
        }

        // Load data from all years
        for year in 2000..=2022 {
            match reader.read_akm(year) {
                Ok(batches) => {
                    log::info!("Loaded {} AKM batches for year {}", batches.len(), year);
                    store.add_akm_data(year, batches);
                }
                Err(e) => log::warn!("Failed to load AKM data for year {}: {}", year, e),
            }

            match reader.read_ind(year) {
                Ok(batches) => {
                    log::info!("Loaded {} IND batches for year {}", batches.len(), year);
                    store.add_ind_data(year, batches);
                }
                Err(e) => log::warn!("Failed to load IND data for year {}: {}", year, e),
            }
        }

        // Load BEF data (quarterly from 2019)
        for year in 2000..=2018 {
            match reader.read_bef(year, None) {
                Ok(batches) => {
                    log::info!("Loaded {} BEF batches for year {}", batches.len(), year);
                    store.add_bef_data(format!("{}", year), batches);
                }
                Err(e) => log::warn!("Failed to load BEF data for year {}: {}", year, e),
            }
        }
        for year in 2019..=2023 {
            for quarter in 1..=4 {
                match reader.read_bef(year, Some(quarter)) {
                    Ok(batches) => {
                        log::info!(
                            "Loaded {} BEF batches for year {} Q{}",
                            batches.len(),
                            year,
                            quarter
                        );
                        store.add_bef_data(format!("{}{:02}", year, quarter * 3), batches);
                    }
                    Err(e) => log::warn!(
                        "Failed to load BEF data for year {} Q{}: {}",
                        year,
                        quarter,
                        e
                    ),
                }
            }
        }

        // Load UDDF data
        for period in ["202009", "202209"] {
            match reader.read_uddf(period) {
                Ok(batches) => {
                    log::info!(
                        "Loaded {} UDDF batches for period {}",
                        batches.len(),
                        period
                    );
                    store.add_uddf_data(period.to_string(), batches);
                }
                Err(e) => log::warn!("Failed to load UDDF data for period {}: {}", period, e),
            }
        }

        Ok(store)
    }
}
