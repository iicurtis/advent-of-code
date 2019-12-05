// Advent of Code
// Copyright (C) 2018  Isaac Curtis

use hashbrown::HashMap;
use std::fmt::{self, Display};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Terrain {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Map {
    grid: Vec<u16>,
    width: usize,
    height: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (x + y * self.width) as usize;
                write!(f, "{}", self.grid[idx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Terrain::Wet => write!(f, "="),
            Terrain::Narrow => write!(f, "|"),
            _ => write!(f, "."),
        }
    }
}

fn part1(input: &str) -> usize {
    let xmax: usize = 80;

    let mut input = input.trim().lines();
    let depth = input.next().unwrap()[7..].parse().unwrap();
    let mut xy_split = input.next().unwrap()[8..].split(',');
    let tx: usize = xy_split.next().unwrap().parse().unwrap();
    let ty: usize = xy_split.next().unwrap().parse().unwrap();
    let ymax = ty + 8;
    let total = xmax * ymax;
    let mut grid = Vec::with_capacity(total);
    let mut r = Vec::with_capacity(xmax);
    let mut prev: u32 = depth;
    for _ in 0..xmax {
        grid.push(prev % 3);
        prev += 16807;
        prev -= if prev >= 20183 { 20183 } else { 0 };
        r.push(prev as u32);
    }

    let mut x0 = depth + 7905;
    let tx1 = tx - 1;
    for idy in 1..ymax {
        prev = x0;
        for idx in 0..xmax {
            grid.push(prev % 3);
            if idy == ty && idx == tx1 { prev = 0; }
            prev = (r[idx] * prev + depth) % 20183;
            r[idx] = prev;
        }
        x0 += 7905;
        x0 -= if x0 >= 20183 { 20183 } else { 0 };
    }


    // Part 1
    let mut soln1 = 0;
    for idy in 0..=ty {
        for idx in 0..=tx {
            soln1 += grid[idx+idy*xmax];
        }
    }
    soln1 as usize
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
"#;
        assert_eq!(part1(input), 1147);
    }
}
