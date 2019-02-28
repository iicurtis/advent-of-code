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
    let input: Vec<i64> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.parse().unwrap())
        .collect();

    let mut instructions = input.clone();

    let mut jmp: i64 = 0;
    let mut escape_steps = 0;
    while jmp < instructions.len() as i64 && jmp >= 0 {
        let tmp = jmp;
        jmp += instructions[jmp as usize];

        instructions[tmp as usize] += 1;
        escape_steps += 1;
    }

    println!("[Day 05][Part 1] ANS is: {}", escape_steps.to_string());

    let mut jmp: i64 = 0;
    let mut escape_steps = 0;
    while jmp < instructions.len() as i64 && jmp >= 0 {
        let tmp = jmp;
        jmp += instructions[jmp as usize];

        if instructions[tmp as usize] > 2 {
            instructions[tmp as usize] -= 1;
        } else {
            instructions[tmp as usize] += 1;
        }
        escape_steps += 1;
    }

    println!("[Day 05][Part 2] ANS is: {}", escape_steps.to_string());
}
