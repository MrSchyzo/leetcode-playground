use criterion::{criterion_group, criterion_main, Criterion};
use rust::RandomizedSet;
use rand::prelude::*;

fn bench_execution(b: &mut Criterion) {
    let size = 1024*1024*32;
    let mut set = RandomizedSet::new();
    let mut rand = rand::thread_rng();

    (0..size).for_each(|_: u32| {
        set.insert((rand.next_u32() % size) as i32);
    });

    let mut group = b.benchmark_group(format!("Randomized set of {size} entries"));
    group.sample_size(16_384);

    group.bench_function("Insertion", |b| b.iter(|| {
        set.insert((rand.next_u32() % (size * 2)) as i32)
    }));
    group.bench_function("Deletion", |b| b.iter(|| {
        set.remove(((rand.next_u32() % (size)) + size) as i32)
    }));
    group.bench_function("Random pick", |b| b.iter(|| {
        set.get_random()
    }));

    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);