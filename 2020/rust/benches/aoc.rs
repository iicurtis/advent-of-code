#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::fs;

fn day01(c: &mut Criterion) {
    use advent2020::day01;
    let input = fs::read_to_string("../inputs/day01.txt").expect("Couldn't find file");
    let input = day01::parse_input(&input);
    c.bench_function("day01 p1", move |b| b.iter(|| day01::part1(&input)));
    let input = fs::read_to_string("../inputs/day01.txt").expect("Couldn't find file");
    let input = day01::parse_input(&input);
    c.bench_function("day01 p2", move |b| b.iter(|| day01::part2(&input)));
}

fn day02(c: &mut Criterion) {
    use advent2020::day02;
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    let input = day02::parse_input(&input);
    c.bench_function("day02 p1", move |b| b.iter(|| day02::part1(&input)));
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    let input = day02::parse_input(&input);
    c.bench_function("day02 p2", move |b| b.iter(|| day02::part2(&input)));
}

fn day03(c: &mut Criterion) {
    use advent2020::day03;
    let input = fs::read_to_string("../inputs/day03.txt").expect("Couldn't find file");
    let input = day03::parse_input(&input);
    c.bench_function("day03 p1", move |b| b.iter(|| day03::part1(&input)));
    let input = fs::read_to_string("../inputs/day03.txt").expect("Couldn't find file");
    let input = day03::parse_input(&input);
    c.bench_function("day03 p2", move |b| b.iter(|| day03::part2(&input)));
}

fn day04(c: &mut Criterion) {
    use advent2020::day04;
    let input = fs::read_to_string("../inputs/day04.txt").expect("Couldn't find file");
    c.bench_function("day04 p1", move |b| b.iter(|| day04::part1(&input)));
    let input = fs::read_to_string("../inputs/day04.txt").expect("Couldn't find file");
    c.bench_function("day04 p2", move |b| b.iter(|| day04::part2(&input)));
}

fn day05(c: &mut Criterion) {
    use advent2020::day05;
    let input = fs::read_to_string("../inputs/day05.txt").expect("Couldn't find file");
    c.bench_function("day05 parse", move |b| {
        b.iter(|| day05::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day05.txt").expect("Couldn't find file");
    let input = day05::parse_input(&input);
    c.bench_function("day05 p1", move |b| b.iter(|| day05::part1(&input)));
    let input = fs::read_to_string("../inputs/day05.txt").expect("Couldn't find file");
    let input = day05::parse_input(&input);
    c.bench_function("day05 p2", move |b| b.iter(|| day05::part2(&input)));
}

fn day06(c: &mut Criterion) {
    use advent2020::day06;
    let input = fs::read_to_string("../inputs/day06.txt").expect("Couldn't find file");
    c.bench_function("day06 p1", move |b| b.iter(|| day06::part1(&input)));
    let input = fs::read_to_string("../inputs/day06.txt").expect("Couldn't find file");
    c.bench_function("day06 p2", move |b| b.iter(|| day06::part2(&input)));
}

fn day07(c: &mut Criterion) {
    use advent2020::day07;
    let input = fs::read_to_string("../inputs/day07.txt").expect("Couldn't find file");
    c.bench_function("day07 parse", move |b| {
        b.iter(|| day07::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day07.txt").expect("Couldn't find file");
    let input = day07::parse_input(&input);
    c.bench_function("day07 p1", move |b| b.iter(|| day07::part1(&input)));
    let input = fs::read_to_string("../inputs/day07.txt").expect("Couldn't find file");
    let input = day07::parse_input(&input);
    c.bench_function("day07 p2", move |b| b.iter(|| day07::part2(&input)));
}

fn day08(c: &mut Criterion) {
    use advent2020::day08;
    let input = fs::read_to_string("../inputs/day08.txt").expect("Couldn't find file");
    c.bench_function("day08 parse", move |b| {
        b.iter(|| day08::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day08.txt").expect("Couldn't find file");
    let input = day08::parse_input(&input);
    c.bench_function("day08 p1", move |b| b.iter(|| day08::part1(&input)));
    let input = fs::read_to_string("../inputs/day08.txt").expect("Couldn't find file");
    let input = day08::parse_input(&input);
    c.bench_function("day08 p2", move |b| b.iter(|| day08::part2(&input)));
}

fn day09(c: &mut Criterion) {
    use advent2020::day09;
    let input = fs::read_to_string("../inputs/day09.txt").expect("Couldn't find file");
    c.bench_function("day09 both", move |b| b.iter(|| day09::solve_both(&input, 25)));
}

criterion_group!(benches, day01, day02, day03, day04, day05, day06, day07, day08, day09);

criterion_main!(benches);
