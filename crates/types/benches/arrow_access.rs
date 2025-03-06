#[cfg(feature = "arrow-integration")]
mod benchmarks {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use arrow_array::{ArrayRef, Int32Array, StringArray};
    use arrow_schema::{DataType, Field, Schema};
    use std::sync::Arc;
    use types::prelude::*;
    use types::storage::arrow::values::ArrowValue;

    fn create_test_batch() -> arrow_array::RecordBatch {
        let schema = Schema::new(vec![
            Field::new("id", DataType::Int32, false),
            Field::new("name", DataType::Utf8, true),
        ]);

        let id_array: ArrayRef = Arc::new(Int32Array::from(vec![1, 2, 3, 4, 5]));
        let name_array: ArrayRef = Arc::new(StringArray::from(vec![
            Some("Alice"), Some("Bob"), None, Some("Dave"), Some("Eve"),
        ]));

        arrow_array::RecordBatch::try_new(
            Arc::new(schema),
            vec![id_array, name_array],
        ).unwrap()
    }

    fn bench_arrow_access(c: &mut Criterion) {
        let batch = create_test_batch();
        let arrow_value = ArrowValue::from(batch);

        c.bench_function("get_int32_value", |b| {
            b.iter(|| {
                for i in 0..5 {
                    let _: i32 = arrow_value.get_value("id", black_box(i)).unwrap();
                }
            });
        });

        c.bench_function("get_string_value", |b| {
            b.iter(|| {
                for i in 0..5 {
                    let _: Option<String> = arrow_value.get_optional_value("name", black_box(i)).unwrap();
                }
            });
        });
    }

    criterion_group!(benches, bench_arrow_access);
    criterion_main!(benches);
}

#[cfg(not(feature = "arrow-integration"))]
fn main() {
    println!("This benchmark requires the arrow-integration feature to be enabled");
}