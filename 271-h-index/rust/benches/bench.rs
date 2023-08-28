use criterion::{criterion_group, criterion_main, Criterion};
use rust::h_index;

fn bench_execution(b: &mut Criterion) {
    let size = 1024*1024*32;

    let kinda_decreasing = (0..size).map(|i| i).collect::<Vec<_>>();
    let ones = (0..size).map(|_| 1).collect::<Vec<_>>();

    let mut group = b.benchmark_group(format!("H-Index"));
    group.sample_size(300);

    group.bench_function("Decreasing", |b| b.iter(|| h_index(kinda_decreasing.clone())));
    group.bench_function("Ones", |b| b.iter(|| h_index(ones.clone())));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);