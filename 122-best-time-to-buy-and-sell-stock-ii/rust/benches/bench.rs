use criterion::{criterion_group, criterion_main, Criterion};
use rust::max_profit;
use rust::max_profit_cooler;

fn bench_execution(b: &mut Criterion) {
    let long = (0..1024*1024*4).collect::<Vec<_>>();
    let mid = long.iter().take(1024*128-1).copied().collect::<Vec<_>>();
    let short = long.iter().take(1024).copied().collect::<Vec<_>>();

    let mut group = b.benchmark_group("Max profit");
    group.sample_size(300);

    group.bench_function("big (4M)", |b| b.iter(|| max_profit(long.clone())));
    group.bench_function("mid (128K)", |b| b.iter(|| max_profit(mid.clone())));
    group.bench_function("small (1024)", |b| b.iter(|| max_profit(short.clone())));

    group.finish();

    let mut group = b.benchmark_group("Cheeky max profix");
    group.sample_size(300);

    group.bench_function("big (4M)", |b| b.iter(|| max_profit_cooler(long.clone())));
    group.bench_function("mid (128K)", |b| b.iter(|| max_profit_cooler(mid.clone())));
    group.bench_function("small (1024)", |b| b.iter(|| max_profit_cooler(short.clone())));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);