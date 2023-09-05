use criterion::{criterion_group, criterion_main, Criterion};
use rust::longest_common_prefix;

fn bench_execution(b: &mut Criterion) {
    let big_string = (0..1024*2).map(|_| 'a').collect::<String>();
    let four_million_chars = (0..1024*2).map(|_| big_string.clone()).collect::<Vec<_>>();

    let mut group = b.benchmark_group("Longest common prefix");
    group.sample_size(300);
    group.bench_function("2K strings that are long 2K", |b| b.iter(|| longest_common_prefix(four_million_chars.clone())));
    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);