use criterion::{black_box, criterion_group, criterion_main, Criterion};
use types::prelude::*;
use chrono::NaiveDate;

fn bench_date_handling(c: &mut Criterion) {
    let date_strings = vec![
        "2020-01-01", "2020-02-15", "2020-03-30",
        "2020-04-12", "2020-05-25", "2020-06-30",
        "2020-07-15", "2020-08-31", "2020-09-15",
        "2020-10-31", "2020-11-30", "2020-12-25",
    ];

    c.bench_function("parse_dates", |b| {
        b.iter(|| {
            for date_str in &date_strings {
                let _: Result<NaiveDate> = black_box(date_str).to_naive_date();
            }
        });
    });

    let dates: Vec<NaiveDate> = date_strings.iter()
        .map(|s| s.to_naive_date().unwrap())
        .collect();

    c.bench_function("date_year_extraction", |b| {
        b.iter(|| {
            for date in &dates {
                let _: Result<i32> = black_box(date).year();
            }
        });
    });
    
    c.bench_function("date_month_extraction", |b| {
        b.iter(|| {
            for date in &dates {
                let _: Result<u32> = black_box(date).month();
            }
        });
    });
    
    c.bench_function("date_day_extraction", |b| {
        b.iter(|| {
            for date in &dates {
                let _: Result<u32> = black_box(date).day();
            }
        });
    });
    
    // Add benchmarks for date comparison operations
    let reference_date = NaiveDate::from_ymd_opt(2020, 6, 15).unwrap();
    
    c.bench_function("date_comparisons", |b| {
        b.iter(|| {
            for date in &dates {
                let _ = black_box(date) < &reference_date;
                let _ = black_box(date) > &reference_date;
                let _ = black_box(date) == &reference_date;
            }
        });
    });
}

criterion_group!(benches, bench_date_handling);
criterion_main!(benches);