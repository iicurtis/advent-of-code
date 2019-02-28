// Advent of Code
// Copyright (C) 2018  Isaac Curtis

use hashbrown::HashMap;
use std::fmt::{self, Display};

type Error = Box<std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Ground {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct World {
    grid: Vec<Ground>,
    width: isize,
    height: isize,
}

impl Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (x + y * self.width) as usize;
                write!(f, "{}", self.grid[idx])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ground::Lumberyard => write!(f, "#"),
            Ground::Trees => write!(f, "|"),
            _ => write!(f, "."),
        }
    }
}

fn parse_input(input: &str) -> Box<World> {
    let mut width = 0;
    let mut height = 0;
    let mut grid = Vec::new();
    let lines = input.trim().lines();
    for (y, line) in lines.enumerate() {
        height = 1 + y as isize;
        for (x, c) in line.chars().enumerate() {
            width = 1 + x as isize;
            grid.push(match c {
                '#' => Ground::Lumberyard,
                '|' => Ground::Trees,
                _ => Ground::Open,
            });
        }
    }

    return Box::new(World {
        grid,
        width,
        height,
    });
}

impl World {
    fn total_count(&mut self) -> (usize, usize) {
        let mut lumber_count = 0;
        let mut wood_count = 0;
        for x in self.grid.iter() {
            if *x == Ground::Lumberyard {
                lumber_count += 1;
            } else if *x == Ground::Trees {
                wood_count += 1;
            }
        }
        (lumber_count, wood_count)
    }

    fn step(&mut self) {
        let old_world = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (x + y * self.width) as usize;
                let mut treecount = 0;
                let mut lumbercount = 0;
                for xa in x - 1..=x + 1 {
                    for ya in y - 1..=y + 1 {
                        if xa == x && ya == y {
                            continue;
                        }
                        if !(xa >= 0 && xa < self.width && ya >= 0 && ya < self.height) {
                            continue;
                        }
                        let adj = (xa + ya * self.width) as usize;
                        if old_world[adj] == Ground::Trees {
                            treecount += 1;
                        }
                        if old_world[adj] == Ground::Lumberyard {
                            lumbercount += 1;
                        }
                    }
                }
                match old_world[idx] {
                    Ground::Open => {
                        if treecount >= 3 {
                            self.grid[idx] = Ground::Trees;
                        }
                    }
                    Ground::Trees => {
                        if lumbercount >= 3 {
                            self.grid[idx] = Ground::Lumberyard;
                        } else {
                            self.grid[idx] = Ground::Trees;
                        }
                    }
                    _ => {
                        if lumbercount >= 1 && treecount >= 1 {
                            self.grid[idx] = Ground::Lumberyard;
                        } else {
                            self.grid[idx] = Ground::Open;
                        }
                    }
                }
            }
        }
    }

    fn run_steps(&mut self, runs: usize) -> usize {
        let mut seen = HashMap::new();
        let mut i = 0;
        while i < runs {
            if let Some(prev_step) = seen.get(&self.grid) {
                let loop_len = i - prev_step;
                i += (runs - i) / loop_len * loop_len;
            } else {
                seen.insert(self.grid.clone(), i);
            }
            self.step();
            i += 1;
        }
        let (lumber, trees) = self.total_count();
        return lumber * trees;
    }
}

fn part1(input: &World) -> usize {
    let mut world = input.clone();
    let ans = world.run_steps(10);
    return ans;
}

fn part2(input: &World) -> usize {
    let mut world = input.clone();
    let ans = world.run_steps(1_000_000_000);
    return ans;
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
        assert_eq!(part1(&parse_input(input)), 1147);
    }

}
