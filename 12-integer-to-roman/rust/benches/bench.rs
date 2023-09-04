use criterion::{criterion_group, criterion_main, Criterion};
use rust::int_to_roman;

fn bench_execution(b: &mut Criterion) {
    let all_ints = (0..4000).collect::<Vec<_>>();

    let mut group = b.benchmark_group("Roman to int");
    group.sample_size(300);
    group.bench_function("0 to 3999", |b| b.iter(|| all_ints.iter().copied().map(int_to_roman).collect::<Vec<_>>()));
    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);