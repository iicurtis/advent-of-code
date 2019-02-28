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

use itertools::Itertools;
use std::io::{self, BufRead};

pub fn solve() {
    // Get line from standard input
    let stdin = io::stdin();
    let input: Vec<Vec<_>> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.split_whitespace().map(|w| w.to_string()).collect())
        .collect();

    let mut valid_count = 0;
    for line in input.iter() {
        if line.iter().unique().count() == line.len() {
            valid_count += 1;
        }
    }

    println!("[Day 04][Part 1] ANS is: {}", valid_count.to_string());

    let mut valid_count = 0;
    for line in input.iter() {
        let words: Vec<_> = line.iter().map(|word| word.chars().sorted()).collect();
        if words.iter().unique().count() == words.len() {
            valid_count += 1;
        }
    }

    println!("[Day 04][Part 2] ANS is: {}", valid_count.to_string());
}
