use criterion::{black_box, criterion_group, criterion_main, Criterion};
use types::prelude::*;
use types::models::covariate::builders::*;

fn bench_covariate_builders(c: &mut Criterion) {
    c.bench_function("build_demographics", |b| {
        b.iter(|| {
            black_box(DemographicsBuilder::new(2, 101, "nuclear")
                .with_age(42)
                .with_gender("M")
                .with_civil_status("married")
                .with_citizenship("DK")
                .with_children_count(2)
                .build())
        });
    });

    c.bench_function("build_education", |b| {
        b.iter(|| {
            black_box(EducationBuilder::new("higher")
                .with_isced_code("6")
                .with_years(16.0)
                .build())
        });
    });
    
    // Add benchmark for covariate access methods
    let demographics = DemographicsBuilder::new(2, 101, "nuclear")
        .with_age(42)
        .with_gender("M")
        .with_civil_status("married")
        .with_citizenship("DK")
        .with_children_count(2)
        .build();
        
    c.bench_function("demographics_access", |b| {
        b.iter(|| {
            // Access all fields to benchmark getter performance
            let _ = black_box(&demographics).age();
            let _ = black_box(&demographics).gender();
            let _ = black_box(&demographics).civil_status();
            let _ = black_box(&demographics).citizenship();
            let _ = black_box(&demographics).children_count();
            let _ = black_box(&demographics).family_type();
        });
    });
}

criterion_group!(benches, bench_covariate_builders);
criterion_main!(benches);