mod parquet;
mod reader;
mod schema;

pub use reader::{DataReader, FileReader};
pub use types::{
    error::IdsError,
    family::FamilyRelations,
    models::*,
    store::{ArrowStore, Store, UnifiedStore},
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct LoaderProgress {
    multi_progress: MultiProgress,
    main_pb: ProgressBar,
    sub_pb: Option<ProgressBar>,
}

impl LoaderProgress {
    pub fn new() -> Self {
        let multi_progress = MultiProgress::new();
        let main_style = ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap();

        let main_pb = multi_progress.add(ProgressBar::new(5));
        main_pb.set_style(main_style);
        main_pb.set_prefix("Overall Progress");

        Self {
            multi_progress,
            main_pb,
            sub_pb: None,
        }
    }

    pub fn create_file_progress(&self, size: u64, filename: &str) -> ProgressBar {
        let style = ProgressStyle::default_bar()
                .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.yellow/red} {bytes}/{total_bytes} ({percent}%) {msg}")
                .unwrap()
                .progress_chars("█▇▆▅▄▃▂▁  ");

        let pb = self.multi_progress.add(ProgressBar::new(size));
        pb.set_style(style);
        pb.set_prefix(filename.to_string());
        pb
    }

    pub fn start_sub_progress(&mut self, total: u64, prefix: String) {
        let style = ProgressStyle::default_bar()
                .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.yellow/red} {pos}/{len} ({percent}%) {msg}")
                .unwrap();

        let pb = self.multi_progress.add(ProgressBar::new(total));
        pb.set_style(style);
        pb.set_prefix(prefix);
        self.sub_pb = Some(pb);
    }

    pub fn increment_main(&self) {
        self.main_pb.inc(1);
    }

    pub fn increment_sub(&self) {
        if let Some(pb) = &self.sub_pb {
            pb.inc(1);
        }
    }

    pub fn finish_main(&self, msg: &str) {
        self.main_pb.finish_with_message(msg.to_string());
    }
}

pub trait StoreLoader {
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError>;
}

pub struct ParquetLoader;

impl Default for ParquetLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParquetLoader {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn load_from_path(&self, base_path: String) -> Result<ArrowStore, IdsError> {
        <Self as StoreLoader>::load_from_path(base_path)
    }
}

impl StoreLoader for ParquetLoader {
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError> {
        let reader = FileReader::new(base_path.clone());
        let mut store = UnifiedStore::new_arrow()?;
        let mut progress = LoaderProgress::new();

        log::info!("Loading data from path: {}", base_path);
        progress.main_pb.set_message("Loading family relations...");

        // Add data file existence checks
        for dir in ["akm", "bef", "ind", "uddf"] {
            let path = std::path::Path::new(&base_path).join(dir);
            if !path.exists() {
                log::warn!("Directory does not exist: {}", path.display());
            } else {
                if let Ok(entries) = std::fs::read_dir(&path) {
                    let files: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
                    log::info!("Found {} files in {}: {:?}", files.len(), dir, files);
                }
            }
        }

        // Load family relations
        match reader.read_family() {
            Ok(family_batches) => {
                progress.main_pb.inc(1);
                log::info!(
                    "Loaded {} family relation batches with {} total rows",
                    family_batches.len(),
                    family_batches.iter().map(|b| b.num_rows()).sum::<usize>()
                );
                store.load_family_relations(family_batches)?;
            }
            Err(e) => log::warn!("Failed to load family relations: {}", e),
        }

        // Load AKM data
        progress.main_pb.set_message("Loading AKM data...");
        progress.start_sub_progress(23, "AKM Years".to_string());
        for year in 2000..=2022 {
            match reader.read_akm(year) {
                Ok(batches) => {
                    if let Some(ref pb) = progress.sub_pb {
                        pb.inc(1);
                    }
                    log::info!("Loaded {} AKM batches for year {}", batches.len(), year);
                    store.add_akm_data(year, batches);
                }
                Err(e) => log::warn!("Failed to load AKM data for year {}: {}", year, e),
            }
        }
        progress.main_pb.inc(1);

        // Load IND data
        progress.main_pb.set_message("Loading IND data...");
        progress.start_sub_progress(23, "IND Years".to_string());
        for year in 2000..=2022 {
            match reader.read_ind(year) {
                Ok(batches) => {
                    if let Some(ref pb) = progress.sub_pb {
                        pb.inc(1);
                    }
                    log::info!("Loaded {} IND batches for year {}", batches.len(), year);
                    store.add_ind_data(year, batches);
                }
                Err(e) => log::warn!("Failed to load IND data for year {}: {}", year, e),
            }
        }
        progress.main_pb.inc(1);

        // Load BEF data
        progress.main_pb.set_message("Loading BEF data...");
        progress.start_sub_progress(24, "BEF Years".to_string());

        for year in 2000..=2018 {
            match reader.read_bef(year, None) {
                Ok(batches) => {
                    if let Some(ref pb) = progress.sub_pb {
                        pb.inc(1);
                    }
                    log::info!("Loaded {} BEF batches for year {}", batches.len(), year);
                    store.add_bef_data(format!("{year}"), batches);
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
            if let Some(ref pb) = progress.sub_pb {
                pb.inc(1);
            }
        }
        progress.main_pb.inc(1);

        // Load UDDF data
        progress.main_pb.set_message("Loading UDDF data...");
        progress.start_sub_progress(2, "UDDF Periods".to_string());
        for period in ["202009", "202209"] {
            match reader.read_uddf(period) {
                Ok(batches) => {
                    if let Some(ref pb) = progress.sub_pb {
                        pb.inc(1);
                    }
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
        progress.main_pb.inc(1);
        progress.main_pb.finish_with_message("Loading complete");

        store.into_arrow_backend()
    }
}
