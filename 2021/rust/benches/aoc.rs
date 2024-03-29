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
    let input = fs::read_to_string("../inputs/day01.txt").expect("Couldn't find file");
    c.bench_function("day01 parse", move |b| b.iter(|| day01::parse(&input)));
}

fn day02(c: &mut Criterion) {
    use advent2021::day02;
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    let input = day02::parse(&input);
    c.bench_function("day02 p1", move |b| b.iter(|| day02::part1(&input)));
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    let input = day02::parse(&input);
    c.bench_function("day02 p2", move |b| b.iter(|| day02::part2(&input)));
    let input = fs::read_to_string("../inputs/day02.txt").expect("Couldn't find file");
    c.bench_function("day02 parse", move |b| b.iter(|| day02::parse(&input)));
}

fn day03(c: &mut Criterion) {
    use advent2021::day03;
    let input = fs::read_to_string("../inputs/day03.txt").expect("Couldn't find file");
    let input = day03::parse(&input);
    c.bench_function("day03 p1", move |b| b.iter(|| day03::part1(&input)));
    let input = fs::read_to_string("../inputs/day03.txt").expect("Couldn't find file");
    let input = day03::parse(&input);
    c.bench_function("day03 p2", move |b| b.iter(|| day03::part2(&input)));
    let input = fs::read_to_string("../inputs/day03.txt").expect("Couldn't find file");
    c.bench_function("day03 parse", move |b| b.iter(|| day03::parse(&input)));
}

fn day04(c: &mut Criterion) {
    use advent2021::day04;
    let input = fs::read_to_string("../inputs/day04.txt").expect("Couldn't find file");
    let input = day04::parse(&input);
    c.bench_function("day04 p1", move |b| b.iter(|| day04::part1(&input)));
    let input = fs::read_to_string("../inputs/day04.txt").expect("Couldn't find file");
    let input = day04::parse(&input);
    c.bench_function("day04 p2", move |b| b.iter(|| day04::part2(&input)));
    let input = fs::read_to_string("../inputs/day04.txt").expect("Couldn't find file");
    c.bench_function("day04 parse", move |b| b.iter(|| day04::parse(&input)));
}

fn day05(c: &mut Criterion) {
    use advent2021::day05;
    let input = fs::read_to_string("../inputs/day05.txt").expect("Couldn't find file");
    let input = day05::parse(&input);
    c.bench_function("day05 p1", move |b| b.iter(|| day05::part1(&input)));
    let input = fs::read_to_string("../inputs/day05.txt").expect("Couldn't find file");
    let input = day05::parse(&input);
    c.bench_function("day05 p2", move |b| b.iter(|| day05::part2(&input)));
    let input = fs::read_to_string("../inputs/day05.txt").expect("Couldn't find file");
    c.bench_function("day05 parse", move |b| b.iter(|| day05::parse(&input)));
}

fn day06(c: &mut Criterion) {
    use advent2021::day06;
    let input = fs::read_to_string("../inputs/day06.txt").expect("Couldn't find file");
    let input = day06::parse(&input);
    c.bench_function("day06 p1", move |b| b.iter(|| day06::part1(&input)));
    let input = fs::read_to_string("../inputs/day06.txt").expect("Couldn't find file");
    let input = day06::parse(&input);
    c.bench_function("day06 p2", move |b| b.iter(|| day06::part2(&input)));
    let input = fs::read_to_string("../inputs/day06.txt").expect("Couldn't find file");
    c.bench_function("day06 parse", move |b| b.iter(|| day06::parse(&input)));
}

fn day07(c: &mut Criterion) {
    use advent2021::day07;
    let input = fs::read_to_string("../inputs/day07.txt").expect("Couldn't find file");
    let input = day07::parse(&input);
    c.bench_function("day07 p1", move |b| b.iter(|| day07::part1(&input)));
    let input = fs::read_to_string("../inputs/day07.txt").expect("Couldn't find file");
    let input = day07::parse(&input);
    c.bench_function("day07 p2", move |b| b.iter(|| day07::part2(&input)));
    let input = fs::read_to_string("../inputs/day07.txt").expect("Couldn't find file");
    c.bench_function("day07 parse", move |b| b.iter(|| day07::parse(&input)));
}

fn day08(c: &mut Criterion) {
    use advent2021::day08;
    let input = fs::read_to_string("../inputs/day08.txt").expect("Couldn't find file");
    c.bench_function("day08 p1", move |b| b.iter(|| day08::part1(&input)));
    let input = fs::read_to_string("../inputs/day08.txt").expect("Couldn't find file");
    c.bench_function("day08 p2", move |b| b.iter(|| day08::part2(&input)));
}

fn day12(c: &mut Criterion) {
    use advent2021::day12;
    let input = fs::read_to_string("../inputs/day12.txt").expect("Couldn't find file");
    c.bench_function("day12 p1", move |b| b.iter(|| day12::part1(&input)));
    let input = fs::read_to_string("../inputs/day12.txt").expect("Couldn't find file");
    c.bench_function("day12 p2", move |b| b.iter(|| day12::part2(&input)));
}

criterion_group!(benches, day01, day02, day03, day04, day05, day06, day07, day08, day12);

criterion_main!(benches);
