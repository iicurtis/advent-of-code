// Advent of Code Solutions
// Copyright (C) 2022  Isaac Curtis
type Error = Box<dyn std::error::Error>;
use itertools::Itertools;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

// taken from https://github.com/ttencate/aoc2022/blob/main/src/bin/03.rs
mod seen {
    pub struct Seen(u128);

    impl TryFrom<&[u8]> for Seen {
        type Error = color_eyre::Report;

        fn try_from(values: &[u8]) -> Result<Self, Self::Error> {
            let mut bits = 0;
            for &c in values {
                bits |= 1 << c;
            }
            Ok(Seen(bits))
        }
    }

    impl std::ops::BitAnd for Seen {
        type Output = Self;
        fn bitand(self, other: Self) -> Self {
            Self(self.0 & other.0)
        }
    }

    impl Seen {
        pub fn priority(&self) -> usize {
            let c = self.0.trailing_zeros() as u8;
            debug_assert!(self.0 == 1 << c);
            match c {
                b'a'..=b'z' => 1 + c - b'a',
                b'A'..=b'Z' => 27 + c - b'A',
                _ => unreachable!(),
            }
            .into()
        }
    }
}

use seen::Seen;

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let (first, second) = bytes.split_at(line.len() / 2);
            (Seen::try_from(first).unwrap() & Seen::try_from(second).unwrap()).priority()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|three_lines| {
            three_lines
                .map(|line| Seen::try_from(line.as_bytes()).unwrap())
                .reduce(|a, b| a & b)
                .unwrap()
                .priority()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
        assert_eq!(part1(input), 157);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
        assert_eq!(part2(input), 70);
    }
}
