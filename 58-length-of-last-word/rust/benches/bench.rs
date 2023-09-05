use criterion::{criterion_group, criterion_main, Criterion};
use rust::length_of_last_word;

fn bench_execution(b: &mut Criterion) {
    let long_string = (0..1024*1024).map(|i| if i%5==0 {' '} else {'z'}).collect::<String>();

    let mut group = b.benchmark_group("Length last word");
    group.sample_size(300);
    group.bench_function("long string, 1M", |b| b.iter(|| length_of_last_word(long_string.clone())));
    group.finish();
}

criterion_group!(bench, bench_execution);
criterion_main!(bench);