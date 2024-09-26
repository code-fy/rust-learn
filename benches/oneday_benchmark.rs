// benches/oneday_benchmark.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_learn::oneday::oneday::oneday; // 从库中引入模块

fn benchmark_oneday(c: &mut Criterion) {
    c.bench_function("oneday", |b| b.iter(|| oneday()));
}

criterion_group!(benches, benchmark_oneday);
criterion_main!(benches);
