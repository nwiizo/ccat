use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_parser(c: &mut Criterion) {
    c.bench_function("parse_claude_md", |b| {
        b.iter(|| {
            // TODO: Add actual parser benchmarks
            black_box(42)
        })
    });
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
