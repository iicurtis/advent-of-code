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

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let (soln1, soln2) = day17(&input);
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
pub struct World {
    world: Vec<Element>,
    x_offset: usize,
    y_offset: usize,
    x_dim: usize,
    y_dim: usize,
}

impl World {
    fn new(points: Vec<Point>, y_max: usize, y_min: usize, x_max: usize, x_min: usize) -> World {
        let x_dim = x_max - x_min + 1;
        let y_dim = y_max - y_min + 1;
        let mut world = vec![Element::SAND; y_dim * x_dim];
        for point in points.iter() {
            let x = point.x - x_min;
            let y = point.y - y_min;
            let index = x + y * x_dim;
            world[index] = Element::CLAY;
        }
        World {
            world,
            x_offset: x_min,
            y_offset: y_min,
            x_dim,
            y_dim,
        }
    }

    fn at(&self, x: isize, y: isize) -> Element {
        if (x < self.x_dim as isize) & (y < self.y_dim as isize) {
            return self.world[(x as usize).wrapping_add(y as usize * self.x_dim)];
        } else {
            return Element::VOID;
        }
    }

    fn set(&mut self, x: isize, y: isize, element: Element) {
        if (x < self.x_dim as isize) & (y < self.y_dim as isize) {
            self.world[(x as usize).wrapping_add(y as usize * self.x_dim)] = element;
        }
    }

    fn count_water(&self, part2: bool) -> usize {
        let mut water_count = 0;
        for x in 0..self.x_dim {
            for y in 0..self.y_dim {
                let tile = self.at(x as isize, y as isize);
                if tile == Element::WATER || (part2 && tile == Element::FLOW) {
                    water_count += 1;
                }
            }
        }
        return water_count;
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::SAND => write!(f, "."),
            Element::CLAY => write!(f, "#"),
            Element::WATER => write!(f, "~"),
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

pub fn parse_input(input: &str) -> World {
    let mut points = Vec::new();
    let mut x_min = 500; // 500 is where the spring is
    let mut x_max = 0;
    let mut y_max = 0;
    let mut y_min = 1 << 20;
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
                    } else if y < y_min {
                        y_min = y;
                    }
                    points.push(Point { x: coord, y: y });
                }
            }
            b'y' => {
                for x in range_start..=range_end {
                    if coord > y_max {
                        y_max = coord;
                    } else if coord < y_min {
                        y_min = coord;
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
    World::new(points, y_max, y_min, x_max, x_min)
}

fn fill(world: &mut World, x: isize, y: isize, dir: isize) {
    if world.at(x, y) == Element::FLOW {
        fill(world, x + dir, y, dir);
        world.set(x, y, Element::WATER);
    }
}

fn flow(world: &mut World, x: isize, y: isize, dir: isize) -> bool {
    let point = world.at(x, y);
    if point != Element::SAND {
        return (point != Element::CLAY) && (point != Element::WATER);
    }

    world.set(x, y, Element::FLOW);

    // Try to flow down
    let mut leaky = flow(world, x, y + 1, 0);
    if leaky {
        return true;
    }

    // Down is not leaky, flow laterally
    leaky = (dir <= 0) && flow(world, x - 1, y, -1);
    leaky |= (dir >= 0) && flow(world, x + 1, y, 1);
    if leaky {
        return true;
    }

    if dir == 0 {
        // Entire layer is watertight, fill up
        fill(world, x, y, -1);
        fill(world, x + 1, y, 1);
    }

    return false;
}

pub fn day17(input: &World) -> (usize, usize) {
    let mut input = input.clone();
    let spring_x = 500 - input.x_offset as isize;
    let spring_y = std::cmp::max(0, 0 - input.y_offset as isize);
    flow(&mut input, spring_x, spring_y, 0);
    let soln1 = input.count_water(true);
    let soln2 = input.count_water(false);
    return (soln1, soln2);
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
        assert_eq!(day17(&parse_input(input)), (57, 29));
    }
}
