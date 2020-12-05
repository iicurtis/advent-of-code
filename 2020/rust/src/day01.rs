// Advent of Code
// Copyright (C) 2020  Isaac Curtis

type Error = Box<dyn std::error::Error>;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> HashSet<usize> {
    let input: Vec<usize> = input.trim().lines().map(|line| line.parse().unwrap()).collect();
    HashSet::from_iter(input)
}

pub fn part1(input: &HashSet<usize>) -> usize {
    let target = 2020;
    for x in input {
        if let Some(y) = find_sum(&input, target, *x) {
            return x * y
        }
    }
    0
}

pub fn part2(input: &HashSet<usize>) -> usize {
    let target = 2020;
    for x in input {
        if let Some((y, z)) = find_triple_sum(&input, target, *x) {
            return x * y * z
        }
    }
    0
}

fn find_triple_sum(input: &HashSet<usize>, target: usize, num: usize) -> Option<(usize, usize)> {
    let other = target - num;
    for x in input {
        if x > &other { continue }
        if let Some(y) = find_sum(&input, other, *x) {
            return Some((*x, y))
        }
    }
    None
}

fn find_sum(input: &HashSet<usize>, target: usize, num: usize) -> Option<usize> {
    // if target - num exists: return multiple; else don't
    let other = target - num;
    if input.contains(&other) {
        return Some(other)
    }
    None
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
