#[cfg(feature = "arrow-integration")]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use arrow_array::{ArrayRef, Int32Array, StringArray};
use arrow_schema::{DataType, Field, Schema};
use std::sync::Arc;
use arrow::record_batch::RecordBatch;

fn create_test_batch() -> RecordBatch {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, true),
    ]);

    let id_array: ArrayRef = Arc::new(Int32Array::from(vec![1, 2, 3, 4, 5]));
    let name_array: ArrayRef = Arc::new(StringArray::from(vec![
        Some("Alice"), Some("Bob"), None, Some("Dave"), Some("Eve"),
    ]));

    RecordBatch::try_new(
        Arc::new(schema),
        vec![id_array, name_array],
    ).unwrap()
}

fn bench_arrow_access(c: &mut Criterion) {
    let batch = create_test_batch();

    c.bench_function("schema_lookup", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let _ = batch.schema().index_of(black_box("id")).unwrap();
                let _ = batch.schema().index_of(black_box("name")).unwrap();
            }
        });
    });

    c.bench_function("column_access", |b| {
        let id_idx = batch.schema().index_of("id").unwrap();
        let name_idx = batch.schema().index_of("name").unwrap();
        
        b.iter(|| {
            for i in 0..5 {
                let id_col = batch.column(id_idx);
                let name_col = batch.column(name_idx);
                
                let id = id_col.as_any().downcast_ref::<Int32Array>().unwrap().value(black_box(i));
                let name = if name_col.is_null(black_box(i)) {
                    None
                } else {
                    Some(name_col.as_any().downcast_ref::<StringArray>().unwrap().value(black_box(i)).to_string())
                };
                
                black_box((id, name));
            }
        });
    });
}

criterion_group!(benches, bench_arrow_access);
criterion_main!(benches);

#[cfg(not(feature = "arrow-integration"))]
fn main() {
    println!("This benchmark requires the arrow-integration feature to be enabled");
}