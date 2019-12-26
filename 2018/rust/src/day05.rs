// Advent of Code
// Copyright (C) 2018  Isaac Curtis

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use rayon::prelude::*;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> Vec<u8> {
    input.trim().as_bytes().to_owned()
}

fn collapse(input: &mut Vec<u8>) -> usize {
    let mut new_len = 0;
    for i in 0..input.len() {
        if new_len > 0 && input[new_len - 1] ^ 0x20 == input[i] {
            new_len -= 1;
        } else {
            input[new_len] = input[i];
            new_len += 1;
        }
    }
    new_len
}

pub fn part1(input: &[u8]) -> usize {
    let mut input = input.to_owned();
    collapse(&mut input)
}

pub fn part2(input: &[u8]) -> usize {
    let mut input = input.to_owned();
    let reduced_len = collapse(&mut input);
    // I was using for loops before I stole this style from CryZe
    (b'a'..=b'z')
        .into_par_iter()
        .map(|letter| {
            let mut new_len = 0;
            let mut input_clone = [0; 1 << 16];
            for item in input.iter().take(reduced_len) {
                if item | 0x20 == letter {
                    continue;
                }

                if new_len > 0 && input_clone[new_len - 1] ^ 0x20 == *item {
                    new_len -= 1;
                } else {
                    input_clone[new_len] = *item;
                    new_len += 1;
                }
            }
            new_len
        })
        .min()
        .expect("Couldn't get min")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"dabAcCaCBAcCcaDA
"#;
        assert_eq!(part1(&parse_input(input)), 10);
    }

    #[test]
    fn test_part2() {
        let input = r#"dabAcCaCBAcCcaDA
"#;
        assert_eq!(part2(&parse_input(input)), 4);
    }
}
