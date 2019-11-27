#[macro_use]
extern crate criterion;

use criterion::Criterion;
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
    c.bench_function("day03 parse", move |b| {
        b.iter(|| day03::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day03.txt").expect("Couldn't find file");
    let input = day03::parse_input(&input);
    c.bench_function("day03", move |b| b.iter(|| day03::day03(&input)));
}

fn day04(c: &mut Criterion) {
    use advent2018::day04;
    let input = fs::read_to_string("../inputs/day04.txt").expect("Couldn't find file");
    c.bench_function("day04 parse", move |b| {
        b.iter(|| day04::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day04.txt").expect("Couldn't find file");
    let input = day04::parse_input(&input);
    c.bench_function("day04 p1", move |b| b.iter(|| day04::part1(&input)));
    let input = fs::read_to_string("../inputs/day04.txt").expect("Couldn't find file");
    let input = day04::parse_input(&input);
    c.bench_function("day04 p2", move |b| b.iter(|| day04::part2(&input)));
}

fn day05(c: &mut Criterion) {
    use advent2018::day05;
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
    use advent2018::day06;
    let input = fs::read_to_string("../inputs/day06.txt").expect("Couldn't find file");
    c.bench_function("day06 parse", move |b| {
        b.iter(|| day06::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day06.txt").expect("Couldn't find file");
    let input = day06::parse_input(&input);
    c.bench_function("day06 p1", move |b| b.iter(|| day06::part1(&input)));
    let input = fs::read_to_string("../inputs/day06.txt").expect("Couldn't find file");
    let input = day06::parse_input(&input);
    c.bench_function("day06 p2", move |b| b.iter(|| day06::part2(&input)));
}

fn day07(c: &mut Criterion) {
    use advent2018::day07;
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
    use advent2018::day08;
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
    use advent2018::day09;
    let input = fs::read_to_string("../inputs/day09.txt").expect("Couldn't find file");
    c.bench_function("day09 parse", move |b| {
        b.iter(|| day09::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day09.txt").expect("Couldn't find file");
    let input = day09::parse_input(&input);
    c.bench_function("day09 p1", move |b| b.iter(|| day09::part1(&input)));
    let input = fs::read_to_string("../inputs/day09.txt").expect("Couldn't find file");
    let input = day09::parse_input(&input);
    c.bench_function("day09 p2", move |b| b.iter(|| day09::part2(&input)));
}

fn day10(c: &mut Criterion) {
    use advent2018::day10;
    let input = fs::read_to_string("../inputs/day10.txt").expect("Couldn't find file");
    c.bench_function("day10 parse", move |b| {
        b.iter(|| day10::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day10.txt").expect("Couldn't find file");
    let input = day10::parse_input(&input);
    c.bench_function("day10 p1", move |b| b.iter(|| day10::part1(&input)));
    let input = fs::read_to_string("../inputs/day10.txt").expect("Couldn't find file");
    let input = day10::parse_input(&input);
    c.bench_function("day10 p2", move |b| b.iter(|| day10::part2(&input)));
}

fn day11(c: &mut Criterion) {
    use advent2018::day11;
    let input = fs::read_to_string("../inputs/day11.txt").expect("Couldn't find file");
    c.bench_function("day11 p1", move |b| b.iter(|| day11::part1(&input)));
    let input = fs::read_to_string("../inputs/day11.txt").expect("Couldn't find file");
    c.bench_function("day11 p2", move |b| b.iter(|| day11::part2(&input)));
}

fn day12(c: &mut Criterion) {
    use advent2018::day12;
    let input = fs::read_to_string("../inputs/day12.txt").expect("Couldn't find file");
    c.bench_function("day12 parse", move |b| {
        b.iter(|| day12::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day12.txt").expect("Couldn't find file");
    let input = day12::parse_input(&input);
    c.bench_function("day12 p1", move |b| b.iter(|| day12::part1(&input)));
    let input = fs::read_to_string("../inputs/day12.txt").expect("Couldn't find file");
    let input = day12::parse_input(&input);
    c.bench_function("day12 p2", move |b| b.iter(|| day12::part2(&input)));
}

fn day13(c: &mut Criterion) {
    use advent2018::day13;
    let input = fs::read_to_string("../inputs/day13.txt").expect("Couldn't find file");
    c.bench_function("day13 parse", move |b| {
        b.iter(|| day13::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day13.txt").expect("Couldn't find file");
    let input = day13::parse_input(&input);
    c.bench_function("day13 p1", move |b| b.iter(|| day13::part1(&input)));
    let input = fs::read_to_string("../inputs/day13.txt").expect("Couldn't find file");
    let input = day13::parse_input(&input);
    c.bench_function("day13 p2", move |b| b.iter(|| day13::part2(&input)));
}

fn day14(c: &mut Criterion) {
    use advent2018::day14;
    let input = fs::read_to_string("../inputs/day14.txt").expect("Couldn't find file");
    c.bench_function("day14 p1", move |b| b.iter(|| day14::part1(&input)));
    let input = fs::read_to_string("../inputs/day14.txt").expect("Couldn't find file");
    c.bench_function("day14 p2", move |b| b.iter(|| day14::part2(&input)));
}

fn day15(c: &mut Criterion) {
    use advent2018::day15;
    let input = fs::read_to_string("../inputs/day15.txt").expect("Couldn't find file");
    c.bench_function("day15 parse", move |b| {
        b.iter(|| day15::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day15.txt").expect("Couldn't find file");
    let input = day15::parse_input(&input);
    c.bench_function("day15 p1", move |b| b.iter(|| day15::part1(&input)));
    let input = fs::read_to_string("../inputs/day15.txt").expect("Couldn't find file");
    let input = day15::parse_input(&input);
    c.bench_function("day15 p2", move |b| b.iter(|| day15::part2(&input)));
}

fn day16(c: &mut Criterion) {
    use advent2018::day16;
    let input = fs::read_to_string("../inputs/day16.txt").expect("Couldn't find file");
    c.bench_function("day16 parse", move |b| {
        b.iter(|| day16::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day16.txt").expect("Couldn't find file");
    let input = day16::parse_input(&input);
    c.bench_function("day16 p1", move |b| b.iter(|| day16::part1(&input)));
    let input = fs::read_to_string("../inputs/day16.txt").expect("Couldn't find file");
    let input = day16::parse_input(&input);
    c.bench_function("day16 p2", move |b| b.iter(|| day16::part2(&input)));
}

fn day17(c: &mut Criterion) {
    use advent2018::day17;
    let input = fs::read_to_string("../inputs/day17.txt").expect("Couldn't find file");
    c.bench_function("day17 parse", move |b| {
        b.iter(|| day17::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day17.txt").expect("Couldn't find file");
    let input = day17::parse_input(&input);
    c.bench_function("day17", move |b| b.iter(|| day17::day17(&input)));
}

fn day18(c: &mut Criterion) {
    use advent2018::day18;
    let input = fs::read_to_string("../inputs/day18.txt").expect("Couldn't find file");
    c.bench_function("day18 parse", move |b| {
        b.iter(|| day18::parse_input(&input))
    });
    let input = fs::read_to_string("../inputs/day18.txt").expect("Couldn't find file");
    let input = day18::parse_input(&input);
    c.bench_function("day18 p1", move |b| b.iter(|| day18::part1(&input)));
    let input = fs::read_to_string("../inputs/day18.txt").expect("Couldn't find file");
    let input = day18::parse_input(&input);
    c.bench_function("day18 p2", move |b| b.iter(|| day18::part2(&input)));
}

fn day19(c: &mut Criterion) {
    use advent2018::day19;
    let input = fs::read_to_string("../inputs/day19.txt").expect("Couldn't find file");
    c.bench_function("day19", move |b| b.iter(|| day19::day19(&input)));
}

criterion_group!(
    benches, day01, day02, day03, day04, day05, day06, day07, day08, day10, day11, day12, day13,
    day16, day17, day19
);

criterion_group! {
    name = slower_benches;
    config = Criterion::default().sample_size(10);
    targets = day09, day14, day15, day18
}
criterion_main!(benches, slower_benches);
