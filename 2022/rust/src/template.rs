// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;
use itertools::Itertools;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = 0;
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse(input: &str) -> Grid {
    let mut xsize = 0;
    let mut ysize = 0;
    let grid = input
        .trim()
        .lines()
        .map(|l| {
            xsize = l.len();
            ysize += 1;
            l.as_bytes().iter().map(|b| b - b'0').collect::<Vec<u8>>()
        })
        .flatten()
        .collect();
    Grid { xsize, ysize, grid }
}

pub fn part1(input: &Grid) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
"#;
        assert_eq!(part1(&parse(input)), 0);
    }

}
