// Advent of Code
// Copyright (C) 2020  Isaac Curtis

use std::cmp::Ordering;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> Vec<usize> {
    let mut input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    // sort and remove inputs larger than target
    input.sort_unstable();
    let smallest = *input.first().unwrap();
    input.retain(move |&n| n == smallest || n + smallest <= 2020);
    input
}

pub fn part1(input: &[usize]) -> usize {
    let target = 2020;
    for (i, &x0) in input.iter().enumerate() {
        let other = target - x0;
        for &x1 in input.iter().skip(i + 1) {
            if x1 == other {
                return x0 * x1;
            }
        }
    }
    0
}

pub fn part2(input: &[usize]) -> usize {
    let target = 2020;
    for (i, &x0) in input.iter().enumerate() {
        let sum_x1_x2 = target - x0;
        for (j, &x1) in input.iter().enumerate().skip(i + 1) {
            if x1 > sum_x1_x2 {
                break;
            }
            let value2 = sum_x1_x2 - x1;
            for &x2 in input.iter().skip(j + 1) {
                match x2.cmp(&value2) {
                    Ordering::Greater => break,
                    Ordering::Equal => return x1 * x0 * x2,
                    _ => (),
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
1721
979
366
299
675
1456
"#;
        assert_eq!(part1(&parse_input(&input)), 514579);
    }
    #[test]
    fn test_part2_0() {
        let input = r#"
1721
979
366
299
675
1456
"#;
        assert_eq!(part2(&parse_input(&input)), 241861950);
    }
}
