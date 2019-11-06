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

use hashbrown::HashMap;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> Box<(Vec<bool>, [bool; 32])> {
    let mut lines = input.trim().lines();
    let plants = lines.next().unwrap()[15..]
        .chars()
        .map(|c| (c == '#'))
        .collect();

    let mut rules = [false; 32];
    for l in lines.skip(1) {
        if l.as_bytes()[9] == b'#' {
            let idx = l
                .bytes()
                .take(5)
                .fold(0, |n, c| (n << 1) | ((c == b'#') as usize));
            rules[idx] = true;
        }
    }
    Box::new((plants, rules))
}

pub fn part1((initial, rules): &(Vec<bool>, [bool; 32])) -> i32 {
    let generations = 20;
    let mut state = initial.clone();

    let mut start = 0;
    let mut i = 0;
    while i < generations {
        state = state
            .iter()
            .chain(&vec![false; 4])
            .scan(0usize, |s, p| {
                *s = (((*s as usize) << 1) & 0x1f) | *p as usize;
                Some(rules[*s] == true)
            })
            .collect();
        start += 2;
        i += 1;
    }
    state
        .into_iter()
        .enumerate()
        .map(|(i, p)| (i as i32 - start as i32) * p as i32)
        .sum()
}

pub fn part2((initial, rules): &(Vec<bool>, [bool; 32])) -> i64 {
    let generations: usize = 50_000_000_000;
    let mut state = initial.clone();
    let mut seen = HashMap::new();

    let mut start = 0;
    let mut i = 0;
    while i < generations {
        let skip = state.iter().position(|&p| p == true).unwrap_or(0);
        let rskip = state
            .iter()
            .rposition(|&p| p == true)
            .unwrap_or_else(|| state.len() - 1);
        state = state[skip..=rskip]
            .iter()
            .chain(&vec![false; 4])
            .scan(0usize, |s, p| {
                *s = (((*s as usize) << 1) & 0x1f) | *p as usize;
                Some(rules[*s] == true)
            })
            .collect();
        start += 2 - skip as i64;
        i += 1;
        if let Some((prev_idx, prev_start)) = seen.get(&state) {
            let loop_len = i - prev_idx;
            let loop_occurs = (generations - i) / loop_len;
            let start_diff: i64 = start - prev_start;
            i += loop_len * loop_occurs;
            start += start_diff * loop_occurs as i64;
        }
        seen.insert(state.clone(), (i, start));
    }
    state
        .into_iter()
        .enumerate()
        .map(|(i, p)| (i as i64 - start as i64) * p as i64)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 325);
    }

}
