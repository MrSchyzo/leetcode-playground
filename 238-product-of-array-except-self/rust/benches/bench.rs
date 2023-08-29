use criterion::{criterion_group, criterion_main, Criterion};
use rust::product_except_self;

fn bench_execution(b: &mut Criterion) {
    let size = 1024*1024*32;
    let long = (0..size).map(|i| 1*(0-(i%2))*(1 + (i/(size-31)))).collect::<Vec<_>>();
    let mid = long.iter().take(128*1024).copied().collect::<Vec<_>>();

    let mut group = b.benchmark_group("Product except self");
    group.sample_size(150);

    group.bench_function(format!("{size} elements"), |b| b.iter(|| product_except_self(long.clone())));
    group.bench_function(format!("128K elements"), |b| b.iter(|| product_except_self(mid.clone())));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);