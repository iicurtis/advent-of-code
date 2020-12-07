// Advent of Code
// Copyright (C) 2020  Isaac Curtis

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> Vec<u16> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' | 'L' => 0,
                    'B' | 'R' => 1,
                    _ => unreachable!(),
                })
                .fold(0, |a, b| a << 1 | b)
        })
        .collect()
}

pub fn part1(input: &[u16]) -> u16 {
    *input.iter().max().unwrap()
}

pub fn part2(input: &[u16]) -> u16 {
    let mut seats = input.to_vec();
    seats.sort_unstable();
    seats
        .windows(2)
        .find(|overlap| overlap[0] + 1 != overlap[1])
        .ok_or("Seat not found")
        .unwrap()[0]
        + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
"#;
        assert_eq!(part1(&parse_input(&input)), 820);
    }
}
