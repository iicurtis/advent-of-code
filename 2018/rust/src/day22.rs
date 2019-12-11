// Advent of Code
// Copyright (C) 2018  Isaac Curtis

use hashbrown::HashMap;
use std::fmt::{self, Display};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let (soln1, soln2) = day22(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Terrain {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Region {
    terrain: Terrain,
    erosion_level: usize,
    tool_done: u8,
}

impl Region {
    fn new(prev_erosion_level: usize, erosion_level: usize) -> Self {
        let terrain = match prev_erosion_level % 3 {
            0 => Terrain::Rocky,
            1 => Terrain::Wet,
            2 => Terrain::Narrow,
            _ => unreachable!()
        };
        Self{ terrain, erosion_level, tool_done: 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Map {
    grid: Vec<Region>,
    width: usize,
    height: usize,
}

struct Node {
    cost: usize,
    x: usize,
    y: usize,
    tool: u8,
}

const TORCH: u8 = 2;

impl Node {
    fn check_neighbor(
        self,
        x: usize,
        y: usize,
        away: isize,
        tool: u8,
        region: Region,
    ) -> Option<(usize, Node)> {
        let mut neighbor = Node { x, y, cost: self.cost, tool: self.tool };
        let mut delta = away * 2;
        if region.terrain as u8 == self.tool {
            neighbor.tool ^= tool;
            neighbor.cost += 7;
            delta += 7 & if self.tool == TORCH { -1 } else { 0 };
            delta += 7 & if neighbor.tool != TORCH { -1 } else {0};
        }
        if (region.tool_done & neighbor.tool) == 0 {
            Some((delta as usize, neighbor))
        } else {
            None
        }
    }
}


pub fn day22(input: &str) -> (usize, usize) {
    let xmax: usize = 80;

    let mut input = input.trim().lines();
    let depth = input.next().unwrap()[7..].parse().unwrap();
    let mut xy_split = input.next().unwrap()[8..].split(',');
    let tx: usize = xy_split.next().unwrap().parse().unwrap();
    let ty: usize = xy_split.next().unwrap().parse().unwrap();
    let ymax = ty + 8;
    let total = xmax * ymax;
    const EROSION_MOD: usize = 20_183;
    let mut grid = Vec::with_capacity(total);
    let mut prev_erosion_level = depth;
    let mut erosion_level;
    for _ in 0..xmax {
        erosion_level = (prev_erosion_level + 16_807) % EROSION_MOD;
        grid.push(Region::new(prev_erosion_level, erosion_level));
        prev_erosion_level = erosion_level;
    }

    let mut x0 = depth + 7_905;
    let tx_1 = tx - 1;
    for idy in 1..ymax {
        prev_erosion_level = x0;
        for idx in 0..xmax {
            let up = grid[idx+(idy-1)*xmax].erosion_level;
            erosion_level = (prev_erosion_level * up + depth) % EROSION_MOD;
            if idy == ty && idx == tx_1 { erosion_level = 0 };
            grid.push(Region::new(prev_erosion_level, erosion_level));
            prev_erosion_level = erosion_level;
        }
        x0 += 7905;
        x0 %= EROSION_MOD;
    }


    // Part 1
    let mut soln1 = 0;
    for idy in 0..=ty {
        for idx in 0..=tx {
            soln1 += grid[idx+idy*xmax].terrain as usize;
        }
    }

    // Part 2 - A* search
    let mut queue = [Vec::with_capacity(2048); 17];
    queue[0].push( Node { cost: 0, x: 0, y: 0, tool: TORCH } );

    loop {
        let mut top = queue[0];
    }
    // if Some(check_neighbor) -> queue[delta].push(check_neighbor)

    let soln2 = 0;
    (soln1, soln2)
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (x + y * self.width) as usize;
                write!(f, "{}", self.grid[idx].terrain)?;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
depth: 510
target: 10,10
"#;
        assert_eq!(day22(input), (114, 45));
    }
}
