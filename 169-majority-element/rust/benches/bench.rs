use criterion::{criterion_group, criterion_main, Criterion};
use rust::majority_element;

fn bench_execution(b: &mut Criterion) {
    let long = (0..1024*1024*4-1).map(|i| (i * 7) % 2).collect::<Vec<_>>();
    let mid = long.iter().take(1024*64-1).copied().collect::<Vec<_>>();
    let short = long.iter().take(1024).copied().collect::<Vec<_>>();

    let mut group = b.benchmark_group("Majority element");
    group.sample_size(300);

    group.bench_function("big (4M)", |b| b.iter(|| majority_element(long.clone())));
    group.bench_function("mid (64K)", |b| b.iter(|| majority_element(mid.clone())));
    group.bench_function("small (1024)", |b| b.iter(|| majority_element(short.clone())));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);