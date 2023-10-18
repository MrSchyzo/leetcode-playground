use criterion::{criterion_group, criterion_main, Criterion};

fn bench_execution(b: &mut Criterion) {
    //TODO: benchmark
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);