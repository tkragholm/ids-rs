use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Annual Register (AKM) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - `SOCIO`: Socioeconomic classification (nullable)
/// - `SOCIO02`: Alternative socioeconomic classification (nullable)
/// - `SOCIO13`: Another socioeconomic classification (nullable)
pub fn akm_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("SOCIO", DataType::Int32, true),
        Field::new("SOCIO02", DataType::Int32, true),
        Field::new("SOCIO13", DataType::Int32, true),
    ])
}

/// Defines the schema for Population Register (BEF) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Multiple demographic and family-related fields (all nullable)
/// Including age, family composition, birth information, etc.
pub fn bef_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("AEGTE_ID", DataType::Utf8, true),
        Field::new("ALDER", DataType::Int16, true),
        Field::new("ANTBOERNF", DataType::Int16, true),
        Field::new("ANTBOERNH", DataType::Int16, true),
        Field::new("ANTPERSF", DataType::Int16, true),
        Field::new("ANTPERSH", DataType::Int16, true),
        Field::new("BOP_VFRA", DataType::Date32, true),
        Field::new("CIVST", DataType::Utf8, true),
        Field::new("CPRTJEK", DataType::Int16, true),
        Field::new("CPRTYPE", DataType::Int16, true),
        Field::new("E_FAELLE_ID", DataType::Utf8, true),
        Field::new("FAMILIE_ID", DataType::Utf8, true),
        Field::new("FAMILIE_TYPE", DataType::Int16, true),
        Field::new("FAR_ID", DataType::Utf8, true),
        Field::new("FM_MARK", DataType::Int16, true),
        Field::new("FOED_DAG", DataType::Date32, true),
        Field::new("HUSTYPE", DataType::Int16, true),
        Field::new("IE_TYPE", DataType::Utf8, true),
        Field::new("KOEN", DataType::Utf8, true),
        Field::new("KOM", DataType::Int16, true),
        Field::new("MOR_ID", DataType::Utf8, true),
        Field::new("OPR_LAND", DataType::Utf8, true),
        Field::new("PLADS", DataType::Int16, true),
        Field::new("REG", DataType::Int16, true),
        Field::new("STATSB", DataType::Int16, true),
        Field::new("VERSION", DataType::Utf8, true),
    ])
}

/// Defines the schema for Individual (IND) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Employment and income-related fields (nullable)
pub fn ind_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("BESKST13", DataType::Int32, true),
        Field::new("LOENMV_13", DataType::Float64, true),
        Field::new("PERINDKIALT_13", DataType::Float64, true),
    ])
}

/// Defines the schema for Education (UDDF) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Education-related fields with date ranges (nullable)
pub fn uddf_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("HFAUDD", DataType::Utf8, true),
        Field::new("HF_VFRA", DataType::Date32, true),
        Field::new("HF_VTIL", DataType::Date32, true),
    ])
}

/// Defines the schema for Family Relations data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Family and parental information with dates (nullable)
pub fn family_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("BIRTH_DATE", DataType::Date32, true),
        Field::new("FATHER_ID", DataType::Utf8, true),
        Field::new("MOTHER_ID", DataType::Utf8, true),
        Field::new("FAMILY_ID", DataType::Utf8, true),
        Field::new("FATHER_BIRTH_DATE", DataType::Date32, true),
        Field::new("MOTHER_BIRTH_DATE", DataType::Date32, true),
    ])
}
