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

    let lines: Vec<Vec<u32>> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|el| el.parse::<u32>().ok())
                .collect()
        })
        .collect();

    let sum: u32 = lines
        .iter()
        .map(|linesum| linesum.iter().max().unwrap() - linesum.iter().min().unwrap())
        .sum();

    println!("[Day 02][Part 1] ANS is: {}", sum.to_string());

    let sum: u32 = lines
        .iter()
        .filter_map(|ls| {
            ls.iter()
                .tuple_combinations()
                .filter_map(|(a, b)| {
                    if a % b == 0 {
                        Some(a / b)
                    } else if b % a == 0 {
                        Some(b / a)
                    } else {
                        None
                    }
                })
                .next()
        })
        .sum();

    println!("[Day 02][Part 2] ANS is: {}", sum.to_string());
    let mut sum: u32 = 0;
    for line in lines.iter() {
        'outer: for (i, a) in line.iter().enumerate() {
            for (j, b) in line.iter().enumerate() {
                if i != j {
                    if a % b == 0 {
                        sum += a / b;
                        break 'outer;
                    } else if b % a == 0 {
                        sum += b / a;
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("[Day 02][Part 2] ANS is: {}", sum.to_string());
}
