use criterion::{criterion_group, criterion_main, Criterion};
use rust::merge;

fn bench_execution(b: &mut Criterion) {
    let mut vec1 = (0..1024*1024*20).map(|i| i * 7).collect::<Vec<_>>();
    let len1 = vec1.len() as i32;
    let mut vec2 = (0..1024*1024*20).map(|i| i * 2).collect::<Vec<_>>();
    let len2 = vec2.len() as i32;

    vec1.extend::<Vec<_>>(vec2.iter().map(|_| 0).collect());

    b.bench_function("very big (20M x 2) merge", |b| b.iter(|| merge(&mut vec1, len1, &mut vec2, len2)));
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);