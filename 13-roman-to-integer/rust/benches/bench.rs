use criterion::{criterion_group, criterion_main, Criterion};
use rust::roman_to_int;

fn bench_execution(b: &mut Criterion) {
    let mmmdccclxxxviii = "MMMDCCCLXXXVIII".to_owned();

    let mut group = b.benchmark_group("Roman to int");
    group.sample_size(300);
    group.bench_function("1M times 3888", |b| b.iter(|| roman_to_int(mmmdccclxxxviii.clone())));
    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);