use arrow_schema::{DataType, Field, Schema};

pub fn akm_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("SOCIO", DataType::Int32, true),
        Field::new("SOCIO02", DataType::Int32, true),
        Field::new("SOCIO13", DataType::Int32, true),
    ])
}

pub fn bef_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("AEGTE_ID", DataType::Utf8, true),
        Field::new("ALDER", DataType::Utf8, true),
        Field::new("ANTBOERNF", DataType::Int32, true),
        Field::new("ANTBOERNH", DataType::Int32, true),
        Field::new("ANTPERSF", DataType::Int32, true),
        Field::new("ANTPERSH", DataType::Int32, true),
        Field::new("BOP_VFRA", DataType::Date32, true),
        Field::new("CIVST", DataType::Utf8, true),
        Field::new("FAMILIE_ID", DataType::Utf8, true),
        Field::new("FAMILIE_TYPE", DataType::Utf8, true),
        Field::new("FAR_ID", DataType::Utf8, true),
        Field::new("FOED_DAG", DataType::Date32, true),
        Field::new("KOM", DataType::Int32, true),
        Field::new("MOR_ID", DataType::Utf8, true),
    ])
}

pub fn ind_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("BESKST13", DataType::Int32, true),
        Field::new("LOENMV_13", DataType::Float64, true),
        Field::new("PERINDKIALT_13", DataType::Float64, true),
    ])
}

pub fn uddf_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("HFAUDD", DataType::Utf8, true),
        Field::new("HF_VFRA", DataType::Date32, true),
        Field::new("HF_VTIL", DataType::Date32, true),
    ])
}
