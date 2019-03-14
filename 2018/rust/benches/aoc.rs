#[macro_use]
extern crate criterion;

use criterion::{Criterion, Fun};
use std::fs;

fn day01(c: &mut Criterion) {
    use advent2018::day01;
    let input = fs::read_to_string("../inputs/day01.txt").expect("Couldn't find file");
    let input = day01::parse_input(&input);
    c.bench_function("day01 p1", move |b| b.iter(|| day01::part1(&input)));
    let input = fs::read_to_string("../inputs/day01.txt").expect("Couldn't find file");
    let input = day01::parse_input(&input);
    c.bench_function("day01 p2", move |b| b.iter(|| day01::part2(&input)));
}

fn day02(c: &mut Criterion) {
    use advent2018::day02;
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    c.bench_function("day02 p1", move |b| b.iter(|| day02::part1(&input)));
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    c.bench_function("day02 p2", move |b| b.iter(|| day02::part2(&input)));
}

fn day03(c: &mut Criterion) {
    use advent2018::day03;
    let input = fs::read_to_string("../inputs/day03.txt").expect("Couldn't find file");
    let input = day03::parse_input(&input);
    c.bench_function("day03 p1", move |b| b.iter(|| day03::part1(&input)));
    let input = fs::read_to_string("../inputs/day03.txt").expect("Couldn't find file");
    let input = day03::parse_input(&input);
    c.bench_function("day03 p2", move |b| b.iter(|| day03::part2(&input)));
}

criterion_group!(benches,
    day01, day02, day03, // day04, day05, day06, day07, day08, day09, day10,
    // day11, day12, day13, day16, day17, day23
);

// criterion_group!{
    // name = slower_benches;
    // config = Criterion::default().sample_size(10);
    // targets = day01_2, day09_2, day11_2, day14, day15, day18, day19, day20,
    // day21, day22, day24, day25
// }
criterion_main!(benches);
