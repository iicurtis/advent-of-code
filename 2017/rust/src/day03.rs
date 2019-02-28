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

use crate::utils::Pt;
use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn solve() {
    // Get line from standard input
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let layer = ((input - 1) as f64).sqrt().ceil() as i64 / 2;
    let man_dist = layer + ((input as i64 - 1) % (layer * 2) - layer).abs();
    println!("[Day 03][Part 1] ANS is: {}", man_dist.to_string());
    let sum = solve_part2_matrix(input);
    println!("[Day 03][Part 2] ANS is: {}", sum.to_string());
    let sum = solve_part2_flat(input);
    println!("[Day 03][Part 2] ANS is: {}", sum.to_string());
}

pub fn solve_part2_flat(input: u64) -> u64 {
    // Trying this a second method:
    let mut spiral: [u64; 512] = [0; 512];
    // This method works by assuming the previous ring has already been added
    // to the second. We need to set up the second ring for this to work.
    // The first ring is just 1. It adds 1 to the first six, but the last two

    // < < <
    // 1 1 1 ^
    // 1 o 1 ^
    // 1 2 2
    for x in 0..6 {
        spiral[x] = 1;
    }
    spiral[6] = 2;
    spiral[7] = 2;

    // Iterators. Rust only lets us take out one at a time.
    let mut inner = 0;
    let mut outer = 8;
    let mut len = 2;
    'len_loop: loop {
        // keep track of sides
        for side in 0..4 {
            // Turn corner. This needs to be skipped on first side
            if side != 0 {
                outer += 1;
                spiral[outer + 1] += spiral[inner - 1];
                outer += 1;
                spiral[outer + 1] += spiral[inner - 1];
                spiral[inner] += spiral[inner - 2];
            }

            for _ in 0..len {
                if input < spiral[inner] {
                    return spiral[inner];
                }
                spiral[outer] += spiral[inner];
                outer += 1;
                spiral[outer] += spiral[inner];
                spiral[inner + 1] += spiral[inner];
                spiral[outer + 1] += spiral[inner];
                inner += 1;
            }
        }
        // Advance to next ring
        spiral[outer] += spiral[inner];
        outer += 1;
        spiral[outer] += spiral[inner];
        outer += 1;
        spiral[inner + 1] += spiral[inner - 1];
        len += 2;
    }
}

pub fn solve_part2_matrix(input: u64) -> u64 {
    // Use a grid mapping points to their contents
    let mut grid = HashMap::new();

    // Put in first value at 0, 0
    grid.insert(Pt(0, 0), 1);
    grid.insert(Pt(1, 0), 1);

    // starting at 1,1 facing west
    let mut pos = Pt(1, 1);
    let mut dir = Pt::w();
    let mut len = 2;
    'outer: loop {
        for _ in 0..2 {
            for _ in 0..len {
                let sum = pos.nn8().iter().map(|v| grid.get(v).unwrap_or(&0)).sum();

                grid.insert(pos, sum);
                if sum > input {
                    return sum;
                }
                pos = pos + dir;
            }
            dir = dir.rot90l();
        }
        len += 1;
    }
}
