use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Annual Register (AKM) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various employment and occupation fields (nullable)
#[must_use]
pub fn akm_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("BESKST13", DataType::Int32, true),
        Field::new("SOCIO13", DataType::Int32, true),
        Field::new("DISCO", DataType::Int32, true),
        Field::new("JOBTYP3", DataType::Int32, true),
        Field::new("STARTAAR", DataType::Int32, true),
        Field::new("STARTDATO", DataType::Date32, true),
        Field::new("SLUTAAR", DataType::Int32, true),
        Field::new("SLUTDATO", DataType::Date32, true),
        Field::new("ARB_TIMER", DataType::Float64, true),
        Field::new("BRUTTO_INDKM", DataType::Float64, true),
        Field::new("GGS_INDKM", DataType::Float64, true),
        Field::new("ARB_HHK", DataType::Int32, true),
        Field::new("BRANCHE3", DataType::Int32, true),
        Field::new("BRANCHENR_DB07", DataType::Int32, true),
        Field::new("DIST_KOEA", DataType::Float64, true),
        Field::new("JOBTYP1", DataType::Int32, true),
        Field::new("STILLING", DataType::Utf8, true),
        Field::new("TILKNYTKOEA", DataType::Int32, true),
        Field::new("LØN", DataType::Float64, true),
        // Legacy fields
        Field::new("SOCIO", DataType::Int32, true),
        Field::new("SOCIO02", DataType::Int32, true),
    ])
}

/// Defines the schema for Population Register (BEF) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Multiple demographic and family-related fields (all nullable)
///   Including age, family composition, birth information, etc.
#[must_use]
pub fn bef_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("FOED_DAG", DataType::Date32, true),
        Field::new("KOEN", DataType::Int32, true),
        Field::new("STATSB", DataType::Int32, true),
        Field::new("IE_TYPE", DataType::Int32, true),
        Field::new("KOM", DataType::Int32, true),
        Field::new("POSTNR", DataType::Int32, true),
        Field::new("BOPIKOM", DataType::Int32, true),
        Field::new("CIVST", DataType::Int32, true),
        Field::new("HFAUDD", DataType::Int32, true),
        Field::new("HFFSP", DataType::Int32, true),
        Field::new("IELAND", DataType::Int32, true),
        Field::new("IESLAND", DataType::Int32, true),
        Field::new("KMAND", DataType::Int32, true),
        Field::new("KMDKN", DataType::Int32, true),
        Field::new("OPRKOD", DataType::Int32, true),
        Field::new("STATKOD", DataType::Int32, true),
        Field::new("FM_MARK", DataType::Int32, true),
        Field::new("HUSTYPE", DataType::Int32, true),
        // Legacy fields
        Field::new("AEGTE_ID", DataType::Utf8, true),
        Field::new("ALDER", DataType::Int16, true),
        Field::new("ANTBOERNF", DataType::Int16, true),
        Field::new("ANTBOERNH", DataType::Int16, true),
        Field::new("ANTPERSF", DataType::Int16, true),
        Field::new("ANTPERSH", DataType::Int16, true),
        Field::new("BOP_VFRA", DataType::Date32, true),
        Field::new("CPRTJEK", DataType::Int16, true),
        Field::new("CPRTYPE", DataType::Int16, true),
        Field::new("E_FAELLE_ID", DataType::Utf8, true),
        Field::new("FAMILIE_ID", DataType::Utf8, true),
        Field::new("FAMILIE_TYPE", DataType::Int16, true),
        Field::new("FAR_ID", DataType::Utf8, true),
        Field::new("MOR_ID", DataType::Utf8, true),
        Field::new("OPR_LAND", DataType::Utf8, true),
        Field::new("PLADS", DataType::Int16, true),
        Field::new("REG", DataType::Int16, true),
        Field::new("VERSION", DataType::Utf8, true),
    ])
}

/// Defines the schema for Individual (IND) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various income and financial fields (nullable)
#[must_use]
pub fn ind_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("PERINDKIALT_13", DataType::Float64, true),
        Field::new("LOENMV_13", DataType::Float64, true),
        Field::new("NETFORMUE_13", DataType::Float64, true),
        Field::new("SAMLET_SKAT_13", DataType::Float64, true),
        Field::new("DISPON_13", DataType::Float64, true),
        Field::new("AINDBPI_13", DataType::Float64, true),
        Field::new("EJVSINDKIALT_13", DataType::Float64, true),
        Field::new("KONTANT_13", DataType::Float64, true),
        Field::new("PENSION_13", DataType::Float64, true),
        Field::new("SUBSYG_13", DataType::Float64, true),
        Field::new("SUBLEDIG_13", DataType::Float64, true),
        Field::new("SUBDAG_13", DataType::Float64, true),
        Field::new("KORSTUTT_13", DataType::Float64, true),
        Field::new("FORMUE_13", DataType::Float64, true),
        Field::new("GAELD_13", DataType::Float64, true),
        Field::new("SOCIO13", DataType::Int32, true),
        // Legacy fields
        Field::new("BESKST13", DataType::Int32, true),
    ])
}

/// Defines the schema for Education (UDDF) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Education-related fields with date ranges (nullable)
#[must_use]
pub fn uddf_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("HFAUDD", DataType::Int32, true),
        Field::new("HFPRIA", DataType::Int32, true),
        Field::new("AUDD", DataType::Int32, true),
        Field::new("AUDD_TYPE", DataType::Int32, true),
        Field::new("AUDD_NAVN", DataType::Utf8, true),
        Field::new("AUDD_STED", DataType::Utf8, true),
        Field::new("AUDD_START", DataType::Date32, true),
        Field::new("AUDD_SLUT", DataType::Date32, true),
        Field::new("UDD_NIVEAU", DataType::Int32, true),
        Field::new("UDD_RET", DataType::Int32, true),
        Field::new("INSTNR", DataType::Int32, true),
        Field::new("DISCKODE", DataType::Int32, true),
        // Legacy fields
        Field::new("HF_VFRA", DataType::Date32, true),
        Field::new("HF_VTIL", DataType::Date32, true),
    ])
}

/// Defines the schema for Family Relations data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Family and parental information (nullable)
#[must_use]
pub fn family_schema() -> Schema {
    Schema::new(vec![
        // Core fields
        Field::new("PNR", DataType::Utf8, false),
        Field::new("mor_pnr", DataType::Utf8, true),
        Field::new("far_pnr", DataType::Utf8, true),
        
        // Optional fields
        Field::new("child_id", DataType::Utf8, true),
        Field::new("mother_id", DataType::Utf8, true),
        Field::new("father_id", DataType::Utf8, true),
        Field::new("person_id", DataType::Utf8, true),
        Field::new("mother_pnr", DataType::Utf8, true),
        Field::new("father_pnr", DataType::Utf8, true),
        Field::new("birth_date", DataType::Date32, true),
        
        // Legacy fields
        Field::new("BIRTH_DATE", DataType::Date32, true),
        Field::new("FATHER_ID", DataType::Utf8, true),
        Field::new("MOTHER_ID", DataType::Utf8, true),
        Field::new("FAMILY_ID", DataType::Utf8, true),
        Field::new("FATHER_BIRTH_DATE", DataType::Date32, true),
        Field::new("MOTHER_BIRTH_DATE", DataType::Date32, true),
    ])
}

/// Create a schema from a JSON file
///
/// # Arguments
///
/// * `schema_path` - Path to the JSON schema file
///
/// # Returns
///
/// A Result containing the Schema or an IdsError
#[cfg(feature = "schema_json")]
pub fn load_schema_from_json(schema_path: impl AsRef<std::path::Path>) -> Result<Schema, crate::IdsError> {
    use std::fs::File;
    use std::io::Read;
    
    // Read JSON schema file
    let mut file = File::open(schema_path.as_ref())
        .map_err(|e| crate::IdsError::Io(e))?;
        
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)
        .map_err(|e| crate::IdsError::Io(e))?;
        
    // Parse JSON into Schema
    let schema: Schema = serde_json::from_str(&json_str)
        .map_err(|e| crate::IdsError::invalid_format(format!(
            "Failed to parse schema JSON: {}", e
        )))?;
        
    Ok(schema)
}

/// Try to find and load a schema for a register type
///
/// This function first checks the schema path from environment variables,
/// then falls back to the default hardcoded schemas.
///
/// # Arguments
///
/// * `register_type` - Type of register (family, akm, bef, ind, uddf)
///
/// # Returns
///
/// The Schema for the specified register type
#[cfg(feature = "schema_json")]
pub fn get_schema(register_type: &str) -> Schema {
    // First try to load from schema path
    if let Ok(schemas_dir) = std::env::var("IDS_SCHEMAS_DIR") {
        let schema_path = std::path::Path::new(&schemas_dir)
            .join(format!("{}.json", register_type));
            
        if schema_path.exists() {
            if let Ok(schema) = load_schema_from_json(&schema_path) {
                log::info!("Using schema from {}", schema_path.display());
                return schema;
            } else {
                log::warn!("Failed to load schema from {}, using default", schema_path.display());
            }
        }
    }
    
    // Fall back to default schemas
    match register_type {
        "family" => family_schema(),
        "akm" => akm_schema(),
        "bef" => bef_schema(),
        "ind" => ind_schema(),
        "uddf" => uddf_schema(),
        _ => {
            log::warn!("Unknown register type: {}, using empty schema", register_type);
            Schema::empty()
        }
    }
}

/// Returns the schema based on register type
/// 
/// Simpler version when schema_json feature is not enabled
#[cfg(not(feature = "schema_json"))]
pub fn get_schema(register_type: &str) -> Schema {
    match register_type {
        "family" => family_schema(),
        "akm" => akm_schema(),
        "bef" => bef_schema(),
        "ind" => ind_schema(),
        "uddf" => uddf_schema(),
        _ => {
            log::warn!("Unknown register type: {}, using empty schema", register_type);
            Schema::empty()
        }
    }
}
