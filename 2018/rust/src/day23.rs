// Advent of Code
// Copyright (C) 2018  Isaac Curtis

type Error = Box<dyn std::error::Error>;
use std::cmp::{Ordering, Reverse};
use std::ops::{Add, Div};

pub fn solve(input: &str) -> Result<String, Error> {
    let nanobots = parse(&input);
    let soln1 = part1(&nanobots);
    let soln2 = part2(&nanobots);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Nanobot {
    pos: Position,
    radius: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Cuboid(Position, Position);

const ORIGIN: Position = Position { x: 0, y: 0, z: 0 };

impl Cuboid {
    fn corners(self) -> [Position; 8] {
        [
            Position::new(self.0.x, self.0.y, self.0.z),
            Position::new(self.1.x, self.0.y, self.0.z),
            Position::new(self.0.x, self.1.y, self.0.z),
            Position::new(self.0.x, self.0.y, self.1.z),
            Position::new(self.1.x, self.1.y, self.0.z),
            Position::new(self.1.x, self.0.y, self.1.z),
            Position::new(self.0.x, self.1.y, self.1.z),
            Position::new(self.1.x, self.1.y, self.1.z),
        ]
    }

    fn distance_to_origin(self) -> usize {
        self.corners()
            .iter()
            .map(|c| c.manhatten(ORIGIN))
            .min()
            .unwrap()
    }

    fn subdivide(self) -> impl Iterator<Item = Cuboid> {
        let (a, b) = (self.0, self.1);
        let midpoint = (self.0 + self.1) / 2;
        if a == midpoint || b == midpoint {
            return self
                .corners()
                .iter()
                .map(|&c| Cuboid(c, c))
                .collect::<Vec<_>>()
                .into_iter();
        }
        vec![
            Cuboid(Position::new(a.x, a.y, a.z), midpoint),
            Cuboid(Position::new(b.x, a.y, a.z), midpoint),
            Cuboid(Position::new(a.x, b.y, a.z), midpoint),
            Cuboid(Position::new(a.x, a.y, b.z), midpoint),
            Cuboid(Position::new(a.x, b.y, b.z), midpoint),
            Cuboid(Position::new(b.x, b.y, b.z), midpoint),
            Cuboid(Position::new(b.x, a.y, b.z), midpoint),
            Cuboid(Position::new(b.x, b.y, a.z), midpoint),
        ]
        .into_iter()
    }

    fn intersects(self, bot: Nanobot) -> bool {
        let closest_pos = Position {
            x: clamp(bot.pos.x, self.0.x, self.1.x),
            y: clamp(bot.pos.y, self.0.y, self.1.y),
            z: clamp(bot.pos.z, self.0.z, self.1.z),
        };
        bot.within_range(closest_pos)
    }

    fn num_bots_intersecting(self, bots: &[Nanobot]) -> usize {
        bots.iter().filter(|&&bot| self.intersects(bot)).count()
    }
}

fn clamp(pos: isize, a: isize, b: isize) -> isize {
    use std::cmp::{max, min};
    let (min_p, max_p) = (min(a, b), max(a, b));
    if pos < min_p {
        min_p
    } else if pos > max_p {
        max_p
    } else {
        pos
    }
}

impl<'a, T: IntoIterator<Item = &'a Nanobot>> From<T> for Cuboid {
    fn from(bots: T) -> Cuboid {
        let mut max = 0;
        for bot in bots {
            max = std::cmp::max(max, bot.pos.x.abs() + bot.radius as isize);
            max = std::cmp::max(max, bot.pos.y.abs() + bot.radius as isize);
            max = std::cmp::max(max, bot.pos.z.abs() + bot.radius as isize);
        }
        let mut i = 1;
        while i < max {
            i *= 2;
        }
        Cuboid(Position::new(-i, -i, -i), Position::new(i, i, i))
    }
}

pub fn parse(input: &str) -> Vec<Nanobot> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut split_line = line[5..].split(">, r=");
            let mut pos_str = split_line.next().unwrap().split(',');
            let radius = split_line.next().unwrap().parse().unwrap();
            let x = pos_str.next().unwrap().parse().unwrap();
            let y = pos_str.next().unwrap().parse().unwrap();
            let z = pos_str.next().unwrap().parse().unwrap();

            Nanobot {
                pos: Position { x, y, z },
                radius,
            }
        })
        .collect()
}

pub fn part1(nanobots: &[Nanobot]) -> usize {
    let largest_nanobot = nanobots
        .iter()
        .max_by_key(|x| x.radius)
        .expect("No nanobots");
    nanobots
        .iter()
        .filter(|x| x.pos.manhatten(largest_nanobot.pos) <= largest_nanobot.radius)
        .count()
}

pub fn part2(nanobots: &[Nanobot]) -> usize {
    use std::collections::BinaryHeap;
    let mut queue = BinaryHeap::with_capacity(5000);
    let universe = Cuboid::from(nanobots);
    queue.push((nanobots.len(), Reverse(universe)));
    while let Some((_, Reverse(cuboid))) = queue.pop() {
        if cuboid.0 == cuboid.1 {
            return cuboid.0.manhatten(ORIGIN);
        }
        queue.extend(
            cuboid
                .subdivide()
                .map(|c| (c.num_bots_intersecting(nanobots), Reverse(c))),
        );
    }
    unreachable!()
}

impl Ord for Cuboid {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.manhatten(self.1).cmp(&other.0.manhatten(other.1)) {
            Ordering::Equal => self.distance_to_origin().cmp(&other.distance_to_origin()),
            x => x,
        }
    }
}

impl PartialOrd for Cuboid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Nanobot {
    fn within_range(self, pos: Position) -> bool {
        pos.manhatten(self.pos) <= self.radius
    }
}

impl Position {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Position { x, y, z }
    }

    fn manhatten(&self, other: Self) -> usize {
        (self.x - other.x).abs() as usize
            + (self.y - other.y).abs() as usize
            + (self.z - other.z).abs() as usize
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Div<isize> for Position {
    type Output = Self;

    fn div(self, scalar: isize) -> Self {
        Position {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE0: &str = r#"
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"#;

    static EXAMPLE1: &str = r#"
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"#;

    #[test]
    fn test_part1_0() {
        assert_eq!(part1(&parse(EXAMPLE0)), 7);
    }

    #[test]
    fn test_part2_0() {
        assert_eq!(part2(&parse(EXAMPLE1)), 36);
    }
}
