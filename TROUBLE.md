BEF:
Field::new("PNR", DataType::Utf8, false),
Field::new("AEGTE_ID", DataType::Utf8, true),
Field::new("ALDER", DataType::Utf8, false),  // ✓ Added to balance calculation
Field::new("ANTBOERNF", DataType::Int32, true),  // ✓ Added to balance calculation as children_count
Field::new("ANTBOERNH", DataType::Int32, true),  // ✓ Added to balance calculation as children_count
Field::new("ANTPERSF", DataType::Int32, true),  // ✓ Already in balance calculation as family_size
Field::new("ANTPERSH", DataType::Int32, true),  // ✓ Already in balance calculation as family_size
Field::new("BOP_VFRA", DataType::Date32, true),
Field::new("CIVST", DataType::Utf8, true),  // ✓ Added to balance calculation as civil_status
Field::new("FAMILIE_ID", DataType::Utf8, true),
Field::new("FAMILIE_TYPE", DataType::Int32, true),  // ✓ Already in balance calculation as family_type
Field::new("FAR_ID", DataType::Utf8, true),
Field::new("FOED_DAG", DataType::Date32, false),
Field::new("KOEN", DataType::Utf8, false),  // ✓ Added to balance calculation as gender
Field::new("KOM", DataType::Int32, true),  // ✓ Already in balance calculation as municipality
Field::new("MOR_ID", DataType::Utf8, true),
Field::new("STATSB", DataType::Utf8, true),  // ✓ Added to balance calculation as citizenship

IND:
Field::new("PNR", DataType::Utf8, false),
Field::new("BESKST13", DataType::Int32, true),  // ✓ Added to balance calculation as employment_status
Field::new("LOENMV_13", DataType::Float64, true),  // ✓ Added to balance calculation as wage_income
Field::new("PERINDKIALT_13", DataType::Float64, true),  // ✓ Already in balance calculation as income
Field::new("PRE_SOCIO", DataType::Int32, true),  // ✓ Added to balance calculation as pre_socio

AKM:
Field::new("PNR", DataType::Utf8, false),
Field::new("SOCIO", DataType::Int32, true),  // ✓ Added to balance calculation
Field::new("SOCIO02", DataType::Int32, true),  // ✓ Added to balance calculation
Field::new("SOCIO13", DataType::Int32, false),  // ✓ Already in balance calculation
Field::new("CPRTJEK", DataType::Int32, true),
Field::new("CPRTYPE", DataType::Int32, true),
Field::new("VERSION", DataType::Utf8, true),
Field::new("SENR", DataType::Utf8, true),
