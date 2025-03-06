#[cfg(feature = "arrow-integration")]
mod benchmarks {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use types::prelude::*;
    use types::store::DataStore;
    use types::storage::arrow::backend::ArrowBackend;
    use types::models::covariate::demographics::Demographics;
    use arrow_array::{ArrayRef, Int32Array, StringArray, RecordBatch};
    use arrow_schema::{DataType, Field, Schema};
    use std::sync::Arc;
    use chrono::NaiveDate;

    fn create_test_data() -> RecordBatch {
        let schema = Schema::new(vec![
            Field::new("pnr", DataType::Utf8, false),
            Field::new("age", DataType::Int32, false),
            Field::new("gender", DataType::Utf8, false),
            Field::new("civil_status", DataType::Utf8, true),
            Field::new("citizenship", DataType::Utf8, true),
            Field::new("children_count", DataType::Int32, true),
            Field::new("family_type", DataType::Utf8, true),
        ]);

        let pnr_array = StringArray::from(vec![
            "0123456789", "1234567890", "2345678901", "3456789012", "4567890123"
        ]);
        let age_array = Int32Array::from(vec![25, 30, 45, 50, 35]);
        let gender_array = StringArray::from(vec!["M", "F", "M", "F", "M"]);
        let civil_status_array = StringArray::from(vec![
            Some("single"), Some("married"), Some("divorced"), None, Some("married")
        ]);
        let citizenship_array = StringArray::from(vec![
            Some("DK"), Some("DK"), Some("SE"), Some("NO"), Some("DK")
        ]);
        let children_count_array = Int32Array::from(vec![Some(0), Some(2), Some(3), None, Some(1)]);
        let family_type_array = StringArray::from(vec![
            Some("single"), Some("nuclear"), Some("nuclear"), None, Some("nuclear")
        ]);

        RecordBatch::try_new(
            Arc::new(schema),
            vec![
                Arc::new(pnr_array) as ArrayRef,
                Arc::new(age_array) as ArrayRef,
                Arc::new(gender_array) as ArrayRef,
                Arc::new(civil_status_array) as ArrayRef,
                Arc::new(citizenship_array) as ArrayRef,
                Arc::new(children_count_array) as ArrayRef,
                Arc::new(family_type_array) as ArrayRef,
            ]
        ).unwrap()
    }

    fn create_test_backend() -> ArrowBackend {
        let mut backend = ArrowBackend::new();
        let batch = create_test_data();
        
        // Add test data to the backend
        backend.add_data("demographics", batch).unwrap();
        
        backend
    }

    fn bench_arrow_backend(c: &mut Criterion) {
        let backend = create_test_backend();
        
        c.bench_function("find_pnr_index", |b| {
            b.iter(|| {
                for pnr in ["0123456789", "1234567890", "2345678901"] {
                    let _ = backend.find_pnr_index("demographics", black_box(pnr));
                }
            });
        });
        
        let mut demographic_group = c.benchmark_group("demographic_retrieval");
        demographic_group.bench_function("get_demographic_with_pnr", |b| {
            b.iter(|| {
                for pnr in ["0123456789", "1234567890", "2345678901"] {
                    let _: Result<Option<Demographics>> = backend.get_demographics(black_box(pnr), 2020);
                }
            });
        });
        demographic_group.finish();
        
        // Test DataStore with Arrow backend
        let mut store = DataStore::new();
        store.set_backend(backend);
        
        c.bench_function("datastore_access", |b| {
            b.iter(|| {
                for pnr in ["0123456789", "1234567890", "2345678901"] {
                    let _: Result<Option<Demographics>> = store.get_demographics(black_box(pnr), 2020);
                }
            });
        });
    }

    criterion_group!(benches, bench_arrow_backend);
    criterion_main!(benches);
}

#[cfg(not(feature = "arrow-integration"))]
fn main() {
    println!("This benchmark requires the arrow-integration feature to be enabled");
}