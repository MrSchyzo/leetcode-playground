use criterion::{criterion_group, criterion_main, Criterion};
use rust::remove_duplicates;

fn bench_execution(b: &mut Criterion) {
    let mut long = (0..1024*1024*32).map(|i| i/7).collect::<Vec<_>>();
    let mut mid = long.iter().take(1024*128).copied().collect::<Vec<_>>();
    let mut short = long.iter().take(1024).copied().collect::<Vec<_>>();

    let mut group = b.benchmark_group("Remove Duplicates");
    group.sample_size(300);

    group.bench_function("big (32M) remove", |b| b.iter(|| remove_duplicates(&mut long)));
    group.bench_function("mid (128K) remove", |b| b.iter(|| remove_duplicates(&mut mid)));
    group.bench_function("small (1024) remove", |b| b.iter(|| remove_duplicates(&mut short)));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);