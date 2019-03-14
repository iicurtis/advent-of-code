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

// use packed_simd::i32x16;
use rayon::prelude::*;
use std::fmt::{self, Display};
use std::i32;

type Error = Box<std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    // let soln2 = part2(&input);
    let soln2 = "No SIMD available currently";
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut input_vec = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Point>>();
    input_vec.sort_by_key(|k| k.x);
    input_vec
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn l1dist(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl std::str::FromStr for Point {
    type Err = Box<std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let point = input
            .split(|c| c == ',' || c == ' ')
            .filter_map(|c| c.parse::<i32>().ok())
            .collect::<Vec<_>>();
        Ok(Point {
            x: point[0],
            y: point[1],
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:3},{:3}]", self.x, self.y)
    }
}

fn part1(input: &Vec<Point>) -> usize {
    let max_x = input.iter().max_by_key(|k| k.x).unwrap().x;
    let max_y = input.iter().max_by_key(|k| k.y).unwrap().y;
    let min_x = input.iter().min_by_key(|k| k.x).unwrap().x;
    let min_y = input.iter().min_by_key(|k| k.y).unwrap().y;
    (min_x..max_x + 1)
        .into_par_iter()
        .fold(
            || vec![0; input.len()],
            |mut count, x| {
                let mut infinite = Vec::new();
                for y in min_y..=max_y {
                    let mut mindist = 1 << 16;
                    let mut eq = false;
                    let mut idx = 0;
                    for (i, k) in input.iter().enumerate() {
                        let dist = Point { x, y }.l1dist(*k);
                        if dist < mindist {
                            mindist = dist;
                            eq = false;
                            idx = i;
                        } else if dist == mindist {
                            eq = true;
                        }
                    }
                    if eq == false {
                        count[idx] += 1;
                        if x == min_x || x == max_x || y == min_y || y == max_y {
                            infinite.push(idx);
                        }
                    }
                }
                for x in infinite.into_iter() {
                    count[x] = 0;
                }
                count
            },
        )
        .reduce(
            || vec![0; input.len()],
            |mut acc, counts| {
                acc.iter_mut()
                    .zip(counts.into_iter())
                    .for_each(|(r, n)| *r += n);
                acc
            },
        )
        .into_iter()
        .max()
        .unwrap()
}

// fn simd_dist_l1(x1: i32x16, y1: i32x16, x2: i32x16, y2: i32x16) -> i32x16 {
    // abs_s(x1 - x2) + abs_s(y1 - y2)
// }

// fn abs_s(vec: i32x16) -> i32x16 {
    // let mask = vec >> (std::mem::size_of::<i32>() * 8 - 1) as u32;
    // (vec + mask) ^ mask
// }

// fn part2(input: &Vec<Point>) -> usize {
    // const MAX_DIST: i32x16 = i32x16::splat(10_000);
    // const ONES: i32x16 = i32x16::splat(1);
    // const ZEROS: i32x16 = i32x16::splat(0);
    // let range_32: i32x16 = i32x16::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    // let max_x = input.iter().max_by_key(|k| k.x).unwrap().x;
    // let max_y = input.iter().max_by_key(|k| k.y).unwrap().y;
    // let min_x = input.iter().min_by_key(|k| k.x).unwrap().x;
    // let min_y = input.iter().min_by_key(|k| k.y).unwrap().y;
    // (min_x..max_x + 1)
        // .into_par_iter()
        // .map(|x| {
            // let mut count = 0;
            // let x1_simd = i32x16::splat(x);
            // for y in (min_y..=max_y).step_by(i32x16::lanes()) {
                // let mut dist = i32x16::splat(0);
                // let y1_simd = i32x16::splat(y) + range_32;
                // for k in input {
                    // let x2_simd = i32x16::splat(k.x);
                    // let y2_simd = i32x16::splat(k.y);
                    // dist += simd_dist_l1(x1_simd, y1_simd, x2_simd, y2_simd);
                // }
                // count += dist.lt(MAX_DIST).select(ONES, ZEROS).wrapping_sum() as u32;
            // }
            // count
        // })
        // .sum::<u32>() as usize
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"#;
        assert_eq!(part1(&parse_input(input)), 17);
    }

    // TEST IS BROKEN DUE TO INPUT REQUIREMENTS CHANGING
    // #[test]
    // fn test_part2() {
    // let input = r#"
    // 1, 1
    // 1, 6
    // 8, 3
    // 3, 4
    // 5, 5
    // 8, 9
    // "#;
    // assert_eq!(part2(&parse_input(input)), 16);
    // }

}
