use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    let mut sum = 0;

    for i in 0..n {
        sum = a + b;
        (a, b) = (b, sum);
    }

    sum
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);