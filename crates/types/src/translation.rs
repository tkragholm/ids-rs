use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

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

    pub fn translate_statsb(&self, code: &str) -> Option<&str> {
        self.statsb.get(code).map(String::as_str)
    }

    pub fn translate_civst(&self, code: &str) -> Option<&str> {
        self.civst.get(code).map(String::as_str)
    }

    pub fn translate_family_type(&self, code: &str) -> Option<&str> {
        self.family_type.get(code).map(String::as_str)
    }

    pub fn translate_fm_mark(&self, code: &str) -> Option<&str> {
        self.fm_mark.get(code).map(String::as_str)
    }

    pub fn translate_hustype(&self, code: &str) -> Option<&str> {
        self.hustype.get(code).map(String::as_str)
    }

    pub fn translate_reg(&self, code: &str) -> Option<&str> {
        self.reg.get(code).map(String::as_str)
    }

    pub fn translate_socio13(&self, code: &str) -> Option<&str> {
        self.socio13.get(code).map(String::as_str)
    }
}

fn load_translation_map(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let file = File::open(Path::new(path))?;
    let map: HashMap<String, String> = serde_json::from_reader(file)?;
    Ok(map)
}
