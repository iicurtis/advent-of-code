extern crate advent2017_rs;
#[macro_use]
extern crate criterion;

use advent2017_rs::*;
use criterion::Criterion;

fn bench_day03_pt2_flat(c: &mut Criterion) {
    c.bench_function("day03pt2_flat", |b| {
        b.iter(|| day03::solve_part2_flat(312051))
    });
}

fn bench_day03_pt2_matrix(c: &mut Criterion) {
    c.bench_function("day03pt2_matrix", |b| {
        b.iter(|| day03::solve_part2_matrix(312051))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = bench_day03_pt2_flat, bench_day03_pt2_matrix,
}
criterion_main!(benches);
