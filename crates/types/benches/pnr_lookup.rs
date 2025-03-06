use criterion::{black_box, criterion_group, criterion_main, Criterion};
use types::prelude::*;
use types::models::pnr::{Pnr, PnrPool};

fn create_test_pool(size: usize) -> PnrPool {
    let mut pool = PnrPool::with_capacity(size);
    for i in 0..size {
        pool.get_or_insert(format!("{:010}", i));
    }
    pool
}

fn bench_pnr_lookup(c: &mut Criterion) {
    let small_pool = create_test_pool(100);
    let medium_pool = create_test_pool(1_000);
    let large_pool = create_test_pool(10_000);

    c.bench_function("pnr_lookup_small", |b| {
        b.iter(|| {
            for i in 0..100 {
                let _ = small_pool.get_or_insert(black_box(format!("{:010}", i % 100)));
            }
        });
    });

    c.bench_function("pnr_lookup_medium", |b| {
        b.iter(|| {
            for i in 0..100 {
                let _ = medium_pool.get_or_insert(black_box(format!("{:010}", i % 1_000)));
            }
        });
    });

    c.bench_function("pnr_lookup_large", |b| {
        b.iter(|| {
            for i in 0..100 {
                let _ = large_pool.get_or_insert(black_box(format!("{:010}", i % 10_000)));
            }
        });
    });
}

criterion_group!(benches, bench_pnr_lookup);
criterion_main!(benches);