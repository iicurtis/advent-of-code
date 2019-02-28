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

use std::io::{self, BufRead};

pub fn solve() {
    // Get line from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let vals: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    // pair up digits using zip, and a cycled iterator skipped by 1
    let captcha1: u32 = vals
        .iter()
        .zip(vals.iter().cycle().skip(1))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum();

    println!("[Day 01][Part 1] ANS is: {}", captcha1.to_string());

    let captcha2: u32 = vals
        .iter()
        .zip(vals.iter().cycle().skip(vals.len() / 2))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum();

    println!("[Day 01][Part 2] ANS is: {}", captcha2.to_string());
}
