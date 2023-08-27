use criterion::{criterion_group, criterion_main, Criterion};
use rust::rotate;

fn bench_execution(b: &mut Criterion) {
    let mut long = (0..1024*1024*32).collect::<Vec<_>>();
    let mut mid = long.iter().take(1024*128-1).copied().collect::<Vec<_>>();
    let mut short = long.iter().take(1024).copied().collect::<Vec<_>>();

    let mut group = b.benchmark_group("Rotate");
    group.sample_size(300);

    group.bench_function("big (32M)", |b| b.iter(|| rotate(&mut long, 1024*128)));
    group.bench_function("mid (128K)", |b| b.iter(|| rotate(&mut mid, 1024)));
    group.bench_function("small (1024)", |b| b.iter(|| rotate(&mut short, 512)));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);