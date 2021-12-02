use criterion::{criterion_group, criterion_main, Criterion};

use aoc2021::solutions::build_runner;

fn criterion_benchmark(c: &mut Criterion) {
    let runner = build_runner();
    for key in runner.list() {
        let solution = runner.get(key).unwrap();
        c.bench_function(key, |b| b.iter(|| solution.run()));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
