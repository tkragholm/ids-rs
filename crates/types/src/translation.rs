use hashbrown::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslationType {
    Statsb,
    Civst,
    FamilyType,
    FmMark,
    Hustype,
    Reg,
    Socio13,
    Hfaudd, // Added HFAUDD to ISCED mapping
}

#[derive(Debug, Clone)]
pub struct TranslationMaps {
    statsb: HashMap<String, String>,
    civst: HashMap<String, String>,
    family_type: HashMap<String, String>,
    fm_mark: HashMap<String, String>,
    hustype: HashMap<String, String>,
    reg: HashMap<String, String>,
    socio13: HashMap<String, String>,
    hfaudd: HashMap<String, String>, // Added HFAUDD to ISCED mapping
}

impl TranslationMaps {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // First try loading from files
        match Self::load_from_files() {
            Ok(maps) => {
                log::info!("Successfully loaded translation maps from files");
                Ok(maps)
            }
            Err(e) => {
                log::warn!(
                    "Failed to load translation maps from files: {}. Using embedded maps instead.",
                    e
                );
                Ok(Self::load_embedded())
            }
        }
    }

    fn load_from_files() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            statsb: load_translation_map("mappings/statsb.json")?,
            civst: load_translation_map("mappings/civst.json")?,
            family_type: load_translation_map("mappings/family_type.json")?,
            fm_mark: load_translation_map("mappings/fm_mark.json")?,
            hustype: load_translation_map("mappings/hustype.json")?,
            reg: load_translation_map("mappings/reg.json")?,
            socio13: load_translation_map("mappings/socio13.json")?,
            hfaudd: load_translation_map("mappings/hfaudd.json")?,
        })
    }

    fn load_embedded() -> Self {
        Self {
            statsb: parse_embedded_json(include_str!(
                "../../ids/python/ids_toolkit/mappings/statsb.json"
            )),
            civst: parse_embedded_json(include_str!("../../ids/python/ids_toolkit/mappings/civst.json")),
            family_type: parse_embedded_json(include_str!(
                "../../ids/python/ids_toolkit/mappings/family_type.json"
            )),
            fm_mark: parse_embedded_json(include_str!(
                "../../ids/python/ids_toolkit/mappings/fm_mark.json"
            )),
            hustype: parse_embedded_json(include_str!(
                "../../ids/python/ids_toolkit/mappings/hustype.json"
            )),
            reg: parse_embedded_json(include_str!("../../ids/python/ids_toolkit/mappings/reg.json")),
            socio13: parse_embedded_json(include_str!(
                "../../ids/python/ids_toolkit/mappings/socio13.json"
            )),
            hfaudd: parse_embedded_json(include_str!(
                "../../ids/python/ids_toolkit/mappings/hfaudd.json"
            )),
        }
    }

    /// Create an empty translation map for diagnostic purposes
    #[must_use] pub fn new_empty() -> Self {
        Self {
            statsb: HashMap::new(),
            civst: HashMap::new(),
            family_type: HashMap::new(),
            fm_mark: HashMap::new(),
            hustype: HashMap::new(),
            reg: HashMap::new(),
            socio13: HashMap::new(),
            hfaudd: HashMap::new(), // Added HFAUDD to ISCED mapping
        }
    }

    pub fn translate(&self, translation_type: TranslationType, code: &str) -> Option<&str> {
        let map = match translation_type {
            TranslationType::Statsb => &self.statsb,
            TranslationType::Civst => &self.civst,
            TranslationType::FamilyType => &self.family_type,
            TranslationType::FmMark => &self.fm_mark,
            TranslationType::Hustype => &self.hustype,
            TranslationType::Reg => &self.reg,
            TranslationType::Socio13 => &self.socio13,
            TranslationType::Hfaudd => &self.hfaudd, // Added HFAUDD to ISCED mapping
        };
        map.get(code).map(String::as_str)
    }

    /// Get all codes that translate to a specific value for a given translation type
    /// Useful for finding all HFAUDD codes that map to a specific ISCED level
    #[must_use] pub fn get_codes_for_value(
        &self,
        translation_type: TranslationType,
        value: &str,
    ) -> Vec<String> {
        let map = match translation_type {
            TranslationType::Statsb => &self.statsb,
            TranslationType::Civst => &self.civst,
            TranslationType::FamilyType => &self.family_type,
            TranslationType::FmMark => &self.fm_mark,
            TranslationType::Hustype => &self.hustype,
            TranslationType::Reg => &self.reg,
            TranslationType::Socio13 => &self.socio13,
            TranslationType::Hfaudd => &self.hfaudd,
        };

        map.iter()
            .filter(|(_, v)| v == &value)
            .map(|(k, _)| k.clone())
            .collect()
    }
}

// Parse JSON string to HashMap
fn parse_embedded_json(json_str: &str) -> HashMap<String, String> {
    match serde_json::from_str(json_str) {
        Ok(map) => map,
        Err(e) => {
            log::error!("Failed to parse embedded JSON: {}", e);
            HashMap::new()
        }
    }
}

fn load_translation_map(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // Log the attempted path
    log::info!("Loading translation map from path: {}", path);

    // Try locations in order of priority:
    // 1. Check if there's an environment variable specifying mappings directory
    // 2. Try the provided path directly
    // 3. Try with current directory

    // First, check for environment variable IDS_MAPPINGS_DIR
    if let Ok(mappings_dir) = std::env::var("IDS_MAPPINGS_DIR") {
        let file_name = Path::new(path).file_name().ok_or("Invalid path")?;
        let env_path = Path::new(&mappings_dir).join(file_name);
        log::info!("Trying path from IDS_MAPPINGS_DIR: {}", env_path.display());

        if let Ok(file) = File::open(&env_path) {
            let map: HashMap<String, String> = serde_json::from_reader(file)?;
            return Ok(map);
        }
        
        log::warn!(
            "Failed to open translation map at environment path: {}",
            env_path.display()
        );
    }

    // Try the provided path directly
    let file_result = File::open(Path::new(path));

    if let Err(ref e) = file_result {
        log::warn!("Failed to open translation map at {}: {}", path, e);

        // Try with current directory
        let current_dir = std::env::current_dir()?;
        let absolute_path = current_dir.join(path);
        log::info!("Trying absolute path: {}", absolute_path.display());

        let file = File::open(absolute_path)?;
        let map: HashMap<String, String> = serde_json::from_reader(file)?;
        return Ok(map);
    }

    let file = file_result?;
    let map: HashMap<String, String> = serde_json::from_reader(file)?;
    Ok(map)
}
