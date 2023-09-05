use criterion::{criterion_group, criterion_main, Criterion};
use rust::convert;

fn bench_execution(b: &mut Criterion) {
    let long_string = (0..1024*1024).map(|_| 'a').collect::<String>();

    let mut group = b.benchmark_group("Zig Zag");
    group.sample_size(300);
    group.bench_function("Long string, 1M", |b| b.iter(|| convert(long_string.clone(), 40)));
    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);