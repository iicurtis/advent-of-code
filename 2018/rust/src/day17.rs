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

use std::fmt::{self, Display};

type Error = Box<std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    // let soln2 = part2(&input);
    let soln2 = "Day 17 part2 not solved yet";
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Element {
    SAND,
    CLAY,
    WATER,
    FLOW,
    VOID,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct World {
    world: Vec<Element>,
    x_offset: usize,
    x_dim: usize,
    y_dim: usize,
}

impl World {
    fn new(points: Vec<Point>, y_max: usize, x_max: usize, x_min: usize) -> World {
        let x_dim = x_max - x_min + 1;
        let y_dim = y_max + 1;
        let mut world = vec![Element::SAND; y_dim * x_dim];
        for point in points.iter() {
            let x = point.x - x_min;
            let y = point.y;
            let index = x + y * x_dim;
            world[index] = Element::CLAY;
        }
        world[500 - x_min] = Element::WATER;
        World {
            world,
            x_offset: x_min,
            x_dim,
            y_dim,
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::SAND => write!(f, "."),
            Element::CLAY => write!(f, "#"),
            Element::WATER => write!(f, "+"),
            Element::FLOW => write!(f, "|"),
            _ => write!(f, "%"),
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.y_dim {
            for x in 0..self.x_dim {
                write!(f, "{}", self.world[x + y * self.x_dim])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> World {
    let mut points = Vec::new();
    let mut x_min = 500; // 500 is where the spring is
    let mut x_max = 0;
    let mut y_max = 0;
    for line in input.trim().lines() {
        let mut coords = line.split(", ");

        let left = coords.next().unwrap();
        let right = coords.next().unwrap();

        let coord = left[2..].parse().unwrap();
        let mut range = right[2..].split("..");

        let range_start = range.next().unwrap().parse().unwrap();
        let range_end = range.next().unwrap().parse().unwrap();

        match line.bytes().next().unwrap() {
            b'x' => {
                for y in range_start..=range_end {
                    if coord > x_max {
                        x_max = coord;
                    } else if coord < x_min {
                        x_min = coord;
                    }
                    if y > y_max {
                        y_max = y;
                    }
                    points.push(Point { x: coord, y: y });
                }
            }
            b'y' => {
                for x in range_start..=range_end {
                    if coord > y_max {
                        y_max = coord;
                    }
                    if x > x_max {
                        x_max = x;
                    } else if x < x_min {
                        x_min = x;
                    }
                    points.push(Point { x: x, y: coord });
                }
            }
            invalid => unreachable!("Invalid coord {}", invalid),
        }
    }
    println!("Parse complete");
    World::new(points, y_max, x_max, x_min)
}

fn part1(input: &World) -> usize {
    println!("{}", input);
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
"#;
        assert_eq!(part1(&parse_input(input)), 27730);
    }
}
