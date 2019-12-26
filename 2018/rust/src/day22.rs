// Advent of Code
// Copyright (C) 2018  Isaac Curtis

use std::fmt::{self, Display};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let map = parse(&input);
    let soln1 = part1(&map);
    let soln2 = part2(&map);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Terrain {
    Rocky = 1,
    Wet = 2,
    Narrow = 4,
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
            _ => unreachable!(),
        };
        Self {
            terrain,
            erosion_level,
            tool_done: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Map {
    grid: Vec<Region>,
    width: usize,
    height: usize,
    tx: usize,
    ty: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    cost: usize,
    x: usize,
    y: usize,
    tool: u8,
}

const TORCH: u8 = 2;

impl Node {
    fn check_neighbor(
        &self,
        x: usize,
        y: usize,
        away: bool,
        tool: u8,
        region: &Region,
    ) -> Option<(usize, Node)> {
        let mut neighbor = Node {
            x,
            y,
            cost: self.cost + 1,
            tool: self.tool,
        };
        let mut delta = if away { 2 } else { 0 };
        if region.terrain as u8 == self.tool {
            neighbor.tool ^= tool;
            neighbor.cost += 7;
            delta += 7 & if self.tool == TORCH { -1 } else { 0 };
            delta += 7 & if neighbor.tool != TORCH { -1 } else { 0 };
        }
        if (region.tool_done & neighbor.tool) == 0 {
            Some((delta as usize, neighbor))
        } else {
            None
        }
    }
}

pub fn parse(input: &str) -> Map {
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
            let up = grid[idx + (idy - 1) * xmax].erosion_level;
            erosion_level = (prev_erosion_level * up + depth) % EROSION_MOD;
            if idy == ty && idx == tx_1 {
                erosion_level = 0
            };
            grid.push(Region::new(prev_erosion_level, erosion_level));
            prev_erosion_level = erosion_level;
        }
        x0 += 7905;
        x0 %= EROSION_MOD;
    }
    Map {
        grid,
        width: xmax,
        height: ymax,
        tx,
        ty,
    }
}

pub fn part1(input: &Map) -> usize {
    let mut soln1 = 0;
    for idy in 0..=input.ty {
        for idx in 0..=input.tx {
            soln1 += (input.grid[idx + idy * input.width].terrain as usize) >> 1;
        }
    }
    soln1
}

pub fn part2(input: &Map) -> usize {
    // Part 2 - A* search
    let mut input = input.to_owned();
    let soln2;
    let mut queue = vec![Vec::with_capacity(2048); 17];
    queue[0].push(Node {
        cost: 0,
        x: 0,
        y: 0,
        tool: TORCH,
    });

    'main: loop {
        while let Some(current_node) = queue[0].pop() {
            let valid_tool;
            {
                let cur_region = &mut input.grid[current_node.x + current_node.y * input.width];
                if (cur_region.tool_done & current_node.tool) != 0 {
                    continue;
                }
                cur_region.tool_done |= current_node.tool;
                valid_tool = 7 ^ cur_region.terrain as u8;
            }

            if (current_node.x == input.tx) & (current_node.y == input.ty) {
                soln2 = if current_node.tool == TORCH {
                    current_node.cost
                } else {
                    current_node.cost + 7
                };
                break 'main;
            }

            if current_node.x.wrapping_sub(1) < input.width {
                let neigh_reg = &input.grid[current_node.x + current_node.y * input.width - 1];
                if let Some((delta, neighbor)) = current_node.check_neighbor(
                    current_node.x - 1,
                    current_node.y,
                    current_node.x <= input.tx,
                    valid_tool,
                    neigh_reg,
                ) {
                    queue[delta].push(neighbor);
                }
            }
            if current_node.x.wrapping_add(1) < input.width {
                let neigh_reg = &input.grid[current_node.x + current_node.y * input.width + 1];
                if let Some((delta, neighbor)) = current_node.check_neighbor(
                    current_node.x + 1,
                    current_node.y,
                    current_node.x >= input.tx,
                    valid_tool,
                    neigh_reg,
                ) {
                    queue[delta].push(neighbor);
                }
            }
            if current_node.y.wrapping_sub(1) < input.height {
                let neigh_reg = &input.grid[current_node.x + (current_node.y - 1) * input.width];
                if let Some((delta, neighbor)) = current_node.check_neighbor(
                    current_node.x,
                    current_node.y - 1,
                    current_node.y <= input.ty,
                    valid_tool,
                    neigh_reg,
                ) {
                    queue[delta].push(neighbor);
                }
            }
            if current_node.y.wrapping_add(1) < input.height {
                let neigh_reg = &input.grid[current_node.x + (current_node.y + 1) * input.width];
                if let Some((delta, neighbor)) = current_node.check_neighbor(
                    current_node.x,
                    current_node.y + 1,
                    current_node.y >= input.ty,
                    valid_tool,
                    neigh_reg,
                ) {
                    queue[delta].push(neighbor);
                }
            }
        }
        queue.rotate_left(1);
    }

    soln2
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
        assert_eq!(part1(&parse(input)), 114);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
depth: 510
target: 10,10
"#;
        assert_eq!(part2(&parse(input)), 45);
    }

    #[test]
    fn test_part1_1() {
        let input = r#"
depth: 10914
target: 9,739
"#;
        assert_eq!(part1(&parse(input)), 7380);
    }

    #[test]
    fn test_part2_1() {
        let input = r#"
depth: 10914
target: 9,739
"#;
        assert_eq!(part2(&parse(input)), 1013);
    }
}
