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

use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn solve() {
    // Get line from standard input
    let stdin = io::stdin();
    let input: Vec<u32> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|el| el.parse().ok())
        .collect();

    let mut banks = input.clone();
    let mut cycles = 0;
    let len = banks.len();

    let mut visited = HashMap::new();

    while !visited.contains_key(&banks) {
        visited.insert(banks.clone(), cycles);

        if let Some((i, &val)) = banks
            .iter()
            .enumerate()
            .max_by_key(|&(i, val)| (val, -(i as isize)))
        {
            banks[i] = 0;

            for j in 0..(val as usize) {
                banks[(i + j + 1) % len] += 1;
            }
        }
        cycles += 1;
    }
    println!("[Day 06][Part 1] ANS is: {}", cycles.to_string());

    println!(
        "[Day 06][Part 2] ANS is: {}",
        cycles - visited.get(&banks).unwrap()
    );
}
