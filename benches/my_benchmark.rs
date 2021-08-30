use criterion::{black_box, criterion_group, criterion_main, Criterion};
use funes::utils;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("filename(large_resource)", |b| {
        b.iter(|| {
            utils::filename(black_box(
                "_______________________________________________________________ \
                 _______________________________________________________________",
            ))
        })
    });

    c.bench_function("calculate_hash", |b| {
        b.iter(|| {
            utils::filename(black_box(
                "_______________________________________________________________ \
                 _______________________________________________________________",
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
