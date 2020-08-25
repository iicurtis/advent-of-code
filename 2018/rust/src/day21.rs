// Advent of Code
// Copyright (C) 2018  Isaac Curtis

use hashbrown::HashSet;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let (soln1, soln2) = day21(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn day21(input: &str) -> (usize, usize) {
    let mut inputiter = input.trim().lines().skip(8);
    let inputval = inputiter
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let x = 0x1016B;
    let x2 = (x * x) & 0xff_ffff;
    let x3 = (x2 * x) & 0xff_ffff;
    let mut prev_r5;
    let mut r5 = 0;
    let mut seen = HashSet::new();
    let mut soln1 = 0;
    let soln2;

    loop {
        prev_r5 = r5;
        r5 = (((r5 >> 16) | 1) * x + ((r5 >> 8) & 0xff) * x2 + ((r5 & 0xff) + inputval) * x3)
            & 0xff_ffff;
        if seen.is_empty() {
            soln1 = r5;
        }
        if !seen.insert(r5) {
            soln2 = prev_r5;
            break;
        }
    }

    (soln1, soln2)
}

// #[cfg(test)]
// mod test {
// use super::*;

// #[test]
// fn test_part1_0() {
// let input = r#"
// #ip 3
// addi 3 16 3
// seti 1 2 5
// seti 1 3 2
// mulr 5 2 1
// eqrr 1 4 1
// addr 1 3 3
// addi 3 1 3
// addr 5 0 0
// addi 2 1 2
// gtrr 2 4 1
// addr 3 1 3
// seti 2 5 3
// addi 5 1 5
// gtrr 5 4 1
// addr 1 3 3
// seti 1 2 3
// mulr 3 3 3
// addi 4 2 4
// mulr 4 4 4
// mulr 3 4 4
// muli 4 11 4
// addi 1 6 1
// mulr 1 3 1
// addi 1 21 1
// addr 4 1 4
// addr 3 0 3
// seti 0 3 3
// setr 3 4 1
// mulr 1 3 1
// addr 3 1 1
// mulr 3 1 1
// muli 1 14 1
// mulr 1 3 1
// addr 4 1 4
// seti 0 3 0
// seti 0 7 3
// "#;
// assert_eq!(day21(&input), (1056, 10915260));
// }
// }
