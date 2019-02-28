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

use hashbrown;

type Error = Box<std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input: Vec<_> = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

fn part1(input: &[isize]) -> isize {
    return input.iter().sum();
}

fn part2(input: &[isize]) -> isize {
    let mut seen = hashbrown::HashSet::with_capacity(input.len());
    let mut sum = 0;
    seen.insert(sum);

    for freq in input.into_iter().cycle() {
        sum += freq;
        if !seen.insert(sum) {
            break;
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[1, -2, 3, 1]), 3);
        assert_eq!(part1(&[1, 1, 1]), 3);
        assert_eq!(part1(&[1, 1, -2]), 0);
        assert_eq!(part1(&[-1, -2, -3]), -6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[1, -2, 3, 1]), 2);
        assert_eq!(part2(&[1, -1]), 0);
        assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);
    }

}
