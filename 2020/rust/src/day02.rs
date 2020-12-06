// Advent of Code
// Copyright (C) 2020  Isaac Curtis

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> Vec<Policy> {
    let input: Vec<Policy> = input
        .trim()
        .lines()
        .map(|line| {
            let mut split_line = line.split('-');
            let lower = split_line.next().unwrap().parse().unwrap();
            let mut split_line = split_line.next().unwrap().split(": ");
            let mut upper_split = split_line.next().unwrap().split(' ');
            let upper = upper_split.next().unwrap().parse().unwrap();
            let letter = upper_split.next().unwrap().parse().unwrap();
            let password = split_line.next().unwrap().chars().collect();
            Policy {
                lower,
                upper,
                letter,
                password,
            }
        })
        .collect();
    // sort and remove inputs larger than target
    input
}

pub fn part1(input: &[Policy]) -> usize {
    input.iter().filter(|pass| pass.is_valid()).count()
}

pub fn part2(input: &[Policy]) -> usize {
    input.iter().filter(|pass| pass.is_valid_p2()).count()
}

#[derive(Debug)]
pub struct Policy {
    lower: usize,
    upper: usize,
    letter: char,
    password: Vec<char>,
}

impl Policy {
    fn is_valid(&self) -> bool {
        let policy_characters = self.password.iter().filter(|c| **c == self.letter).count();
        policy_characters >= self.lower && policy_characters <= self.upper
    }

    fn is_valid_p2(&self) -> bool {
        (self.password[self.lower - 1] == self.letter)
            ^ (self.password[self.upper - 1] == self.letter)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;
        assert_eq!(part1(&parse_input(&input)), 2);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;
        assert_eq!(part2(&parse_input(&input)), 1);
    }
}
