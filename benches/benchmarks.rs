use criterion::{Criterion, criterion_group, criterion_main};
use pido_rs::{is_even, is_odd, parallel_check};

fn bench_sequential(c: &mut Criterion) {
    let numbers: Vec<i32> = (0..100000).collect();
    c.bench_function("sequential_check", |b| {
        b.iter(|| {
            numbers
                .iter()
                .map(|&n| (is_even(n), is_odd(n)))
                .collect::<Vec<_>>()
        })
    });
}

fn bench_parallel(c: &mut Criterion) {
    let numbers: Vec<i32> = (0..100000).collect();
    c.bench_function("parallel_check", |b| b.iter(|| parallel_check(&numbers)));
}

criterion_group!(benches, bench_sequential, bench_parallel);
criterion_main!(benches);
