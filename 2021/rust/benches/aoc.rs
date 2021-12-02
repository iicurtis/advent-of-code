#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::fs;

fn day01(c: &mut Criterion) {
    use advent2021::day01;
    let input = fs::read_to_string("../inputs/day01.txt").expect("Couldn't find file");
    let input = day01::parse(&input);
    c.bench_function("day01 p1", move |b| b.iter(|| day01::part1(&input)));
    let input = fs::read_to_string("../inputs/day01.txt").expect("Couldn't find file");
    let input = day01::parse(&input);
    c.bench_function("day01 p2", move |b| b.iter(|| day01::part2(&input)));
}

fn day02(c: &mut Criterion) {
    use advent2021::day02;
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    let input = day02::parse(&input);
    c.bench_function("day02 p1", move |b| b.iter(|| day02::part1(&input)));
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    let input = day02::parse(&input);
    c.bench_function("day02 p2", move |b| b.iter(|| day02::part2(&input)));
}

criterion_group!(benches, day01, day02);

criterion_main!(benches);
