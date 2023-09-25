// Advent of Code Solutions
// Copyright (C) 2022  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse(input: &str) -> Vec<(u8, u8)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let abc = line[0] - b'A';
            let xyz = line[2] - b'X';
            (abc, xyz)
        })
        .collect()
}

pub fn part1(input: &[(u8, u8)]) -> usize {
    input
        .iter()
        .map(|(abc, xyz)| match (xyz + 3 - abc) % 3 {
            0 => *xyz as usize + 1 + 3,
            1 => *xyz as usize + 1 + 6,
            2 => *xyz as usize + 1,
            _ => unreachable!(),
        })
        .sum()
}

pub fn part2(input: &[(u8, u8)]) -> usize {
    input
        .iter()
        .map(|(abc, xyz)| ((*abc + *xyz + 2) % 3 + 1 + xyz * 3) as usize)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
A Y
B X
C Z
"#;
        assert_eq!(part1(&parse(input)), 15);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
A Y
B X
C Z
"#;
        assert_eq!(part2(&parse(input)), 12);
    }
}
