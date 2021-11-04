use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pure_lines;

pub fn pure_lines_benchmark(c: &mut Criterion) {
    let a = "
    hello
    word";
    c.bench_function("pure lines", |b| b.iter(|| pure_lines::pure(black_box(a))));
}

criterion_group!(benches, pure_lines_benchmark);
criterion_main!(benches);