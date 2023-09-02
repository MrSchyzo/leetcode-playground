use criterion::{criterion_group, criterion_main, Criterion};
use rust::candy;

fn bench_execution(b: &mut Criterion) {
    let long = (0..1024*1024*32).collect::<Vec<_>>();
    let mid = long.iter().take(1024*128-1).copied().collect::<Vec<_>>();
    let short = long.iter().take(1024).copied().collect::<Vec<_>>();

    let mut group = b.benchmark_group("Candy");
    group.sample_size(300);

    group.bench_function("big (32M)", |b| b.iter(|| candy(long.clone())));
    group.bench_function("mid (128K)", |b| b.iter(|| candy(mid.clone())));
    group.bench_function("small (1024)", |b| b.iter(|| candy(short.clone())));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);