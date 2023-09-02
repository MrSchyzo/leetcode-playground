use criterion::{criterion_group, criterion_main, Criterion};
use rust::can_complete_circuit;

fn bench_execution(b: &mut Criterion) {
    let gas = (0..1024*1024*32).map(|i| i % 10).collect::<Vec<_>>();
    let cost = (0..1024*1024*32).map(|i| (i+3) % 10).collect::<Vec<_>>();

    let mut group = b.benchmark_group("Gas stations");
    group.sample_size(300);

    group.bench_function("long circuit (32M)", |b| b.iter(|| can_complete_circuit(gas.clone(), cost.clone())));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);