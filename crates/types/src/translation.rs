use std::collections::HashMap;
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
}

impl TranslationMaps {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            statsb: load_translation_map("mappings/statsb.json")?,
            civst: load_translation_map("mappings/civst.json")?,
            family_type: load_translation_map("mappings/family_type.json")?,
            fm_mark: load_translation_map("mappings/fm_mark.json")?,
            hustype: load_translation_map("mappings/hustype.json")?,
            reg: load_translation_map("mappings/reg.json")?,
            socio13: load_translation_map("mappings/socio13.json")?,
        })
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
        };
        map.get(code).map(String::as_str)
    }
}

fn load_translation_map(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let file = File::open(Path::new(path))?;
    let map: HashMap<String, String> = serde_json::from_reader(file)?;
    Ok(map)
}
