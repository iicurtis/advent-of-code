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
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Copy, Debug)]
enum Track {
    Blank,
    TrackVertical,
    TrackHorizontal,
    TurnCounter,
    TurnClock,
    Intersection,
}

#[derive(Clone, Debug)]
pub struct Grid {
    grid: Vec<Track>,
    xsize: usize,
    ysize: usize,
    carts: Vec<Cart>,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Cart {
    pos: Point,
    direction: usize,
    turning: usize,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Point {
    y: usize,
    x: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Cart {
    fn move_forward(&mut self) {
        self.pos = match self.direction {
            0 => Point {
                x: self.pos.x + 1,
                y: self.pos.y,
            },
            1 => Point {
                x: self.pos.x,
                y: self.pos.y - 1,
            },
            2 => Point {
                x: self.pos.x - 1,
                y: self.pos.y,
            },
            _ => Point {
                x: self.pos.x,
                y: self.pos.y + 1,
            },
        }
    }

    fn turn(&mut self, track: Track) {
        let (dir, turn) = match track {
            Track::TurnClock => ([1, 0, 3, 2][self.direction], self.turning),
            Track::TurnCounter => ([3, 2, 1, 0][self.direction], self.turning),
            Track::Intersection => match self.turning {
                0 => ((self.direction + 1) % 4, 1),
                1 => (self.direction, 2),
                _ => ((self.direction + 3) % 4, 0),
            },
            _ => (self.direction, self.turning),
        };
        self.direction = dir;
        self.turning = turn;
    }
}

impl Grid {
    fn get(&self, pos: Point) -> Track {
        return self.grid[pos.x + pos.y * self.xsize];
    }

    fn last_cart(&self) -> Option<Point> {
        if self.carts.len() == 1 {
            return Some(self.carts[0].pos);
        }
        None
    }

    fn step(&mut self) -> Option<Point> {
        self.carts.sort();
        let mut others = self.carts.clone();
        let mut crashed = Vec::new();
        for (i, cart) in self.carts.iter_mut().enumerate() {
            if crashed.contains(&i) {
                continue;
            }
            cart.move_forward();
            cart.turn(self.grid[cart.pos.x + cart.pos.y * self.xsize]);
            for (j, other) in others.iter().enumerate() {
                if cart.pos == other.pos {
                    crashed.push(i);
                    crashed.push(j);
                }
            }
            others[i] = *cart;
        }
        if crashed.is_empty() {
            None
        } else {
            let first_crashed = self.carts[crashed[0]].pos;
            crashed.sort();
            for c in crashed.iter().rev() {
                self.carts.remove(*c);
            }
            Some(first_crashed)
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                let mut cart_on_point = false;
                for c in self.carts.iter() {
                    if c.pos.x == x && c.pos.y == y {
                        cart_on_point = true;
                    }
                }
                if cart_on_point {
                    write!(f, "▓")?
                } else {
                    match self.get(Point { x, y }) {
                        Track::Blank => write!(f, " ")?,
                        Track::Intersection => write!(f, "+")?,
                        Track::TrackVertical => write!(f, "|")?,
                        Track::TrackHorizontal => write!(f, "-")?,
                        Track::TurnClock => write!(f, "/")?,
                        Track::TurnCounter => write!(f, "\\")?,
                    };
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

pub fn parse_input(input: &str) -> Box<Grid> {
    const WIDTH: usize = 150;
    const HEIGHT: usize = 150;
    let mut grid = vec![Track::Blank; WIDTH * HEIGHT];
    let mut carts = Vec::new();
    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + y * WIDTH] = match c {
                '|' => Track::TrackVertical,
                '-' => Track::TrackHorizontal,
                '/' => Track::TurnClock,
                '\\' => Track::TurnCounter,
                '+' => Track::Intersection,
                '^' | 'v' | '<' | '>' => {
                    carts.push(Cart {
                        pos: Point { x, y },
                        direction: match c {
                            '^' => 1,
                            '<' => 2,
                            'v' => 3,
                            _ => 0,
                        },
                        turning: 0,
                    });
                    Track::TrackVertical
                }
                _ => Track::Blank,
            };
        }
    }

    return Box::new(Grid {
        grid,
        xsize: WIDTH,
        ysize: HEIGHT,
        carts,
    });
}

pub fn part1(input: &Grid) -> Point {
    let mut track = input.clone();
    loop {
        if let Some(crash) = track.step() {
            break crash;
        }
    }
}

pub fn part2(input: &Grid) -> Point {
    let mut track = input.clone();
    loop {
        track.step();
        if let Some(last) = track.last_cart() {
            break last;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/ "#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), Point { x: 7, y: 3 });
    }

    #[test]
    fn test_part2() {
        let input = r#"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;
        assert_eq!(part2(&parse_input(input)), Point { x: 6, y: 4 });
    }

}
