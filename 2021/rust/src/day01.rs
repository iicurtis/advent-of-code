// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part1(input: &[usize]) -> usize {
    // its pretty but 5% slower than the loop version (only when parsing is done in function?)
    input.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

pub fn part2(input: &[usize]) -> usize {
    input.windows(4).filter(|n| n[0] < n[3]).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
"#;
        assert_eq!(part1(&parse(input)), 7);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
"#;
        assert_eq!(part2(&parse(input)), 5);
    }
}
