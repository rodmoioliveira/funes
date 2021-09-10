use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("funes::utils::hash", |b| {
        b.iter(|| funes::utils::hash(black_box(&20)))
    });

    c.bench_function("funes::api::map_range", |b| {
        b.iter(|| {
            funes::api::map_range(
                (black_box(0.0), black_box(100.0)),
                (black_box(0.038), black_box(3.12)),
                black_box(20.0),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
