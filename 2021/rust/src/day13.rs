// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;
use std::fmt::{self, Display};

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Debug)]
pub struct FoldMap {
    grid: std::collections::HashSet<(usize, usize)>,
    splits: Vec<(usize, usize)>,
}

impl Display for FoldMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..6 {
            for x in 0..39 {
                if self.grid.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse(input: &str) -> FoldMap {
    let (coords, directions) = input.trim().split_once("\n\n").unwrap();
    let mut grid = std::collections::HashSet::new();
    for line in coords.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        grid.insert((x, y));
    }
    let mut splits: Vec<(usize, usize)> = Vec::new();
    for line in directions.lines() {
        let (ax, coord) = line.split_once('=').unwrap();
        if ax.ends_with('y') {
            splits.push((0, coord.parse().unwrap()));
        } else {
            splits.push((coord.parse().unwrap(), 0));
        }
    }
    FoldMap { grid, splits }
}

pub fn part1(input: &FoldMap) -> usize {
    let split = input.splits.first().unwrap();
    let mut grid = input.grid.clone();
    for point in grid.clone() {
        let mut newpoint = point;
        if split.0 > newpoint.0 {
            newpoint.0 = split.0 * 2 - point.0;
        }
        if split.1 > newpoint.1 {
            newpoint.1 = split.1 * 2 - point.1;
        }
        grid.remove(&point);
        grid.insert(newpoint);
    }
    grid.len()
}

pub fn part2(input: &FoldMap) -> usize {
    let mut input = input.clone();
    for split in &input.splits {
        for point in input.grid.clone() {
            let mut newpoint = point;
            if split.0 < newpoint.0 && (split.0 != 0) {
                newpoint.0 = split.0 * 2 - point.0;
            }
            if split.1 < newpoint.1 && (split.1 != 0) {
                newpoint.1 = split.1 * 2 - point.1;
            }
            input.grid.remove(&point);
            input.grid.insert(newpoint);
        }
    }
    println!("{}", input);
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;
        assert_eq!(part1(&parse(input)), 17);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;
        assert_eq!(part2(&parse(input)), 0);
    }
}
