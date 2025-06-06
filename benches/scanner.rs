use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_scanner(c: &mut Criterion) {
    c.bench_function("scan_directory", |b| {
        b.iter(|| {
            // TODO: Add actual scanner benchmarks
            black_box(42)
        })
    });
}

criterion_group!(benches, bench_scanner);
criterion_main!(benches);
