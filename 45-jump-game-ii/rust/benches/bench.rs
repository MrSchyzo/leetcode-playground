use criterion::{criterion_group, criterion_main, Criterion};
use rust::jump;

fn bench_execution(b: &mut Criterion) {
    let size = 1024*32;

    let kinda_decreasing = (0..size).map(|i| size - 2 - i + (i/size-2)).collect::<Vec<_>>();
    let ones = (0..size).map(|_| 1).collect::<Vec<_>>();
    let big_leap = vec![size].into_iter().chain((0..size-1).map(|_| 1)).collect::<Vec<_>>();

    let mut group = b.benchmark_group(format!("Min jumps {size} elements"));
    group.sample_size(300);

    group.bench_function("Decreasing", |b| b.iter(|| jump(kinda_decreasing.clone())));
    group.bench_function("Ones", |b| b.iter(|| jump(ones.clone())));
    group.bench_function("Big leap", |b| b.iter(|| jump(big_leap.clone())));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);