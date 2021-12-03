use core::fmt;

// Advent of Code
// Copyright (C) 2020  Isaac Curtis
use std::fmt::Display;
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Seat {
    Occupied,
    Empty,
    Floor,
}

#[derive(Clone, Debug)]
pub struct Grid {
    grid: Vec<Seat>,
    xsize: usize,
    ysize: usize,
}

pub fn parse(input: &str) -> Grid {
    let input = input.trim().lines();
    let ysize = input.clone().count();
    let xsize = input.clone().next().unwrap().len();
    let mut grid = vec![Seat::Floor; ysize*xsize];
    for (y, line) in input.enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + y * xsize] = match c {
                'L' => Seat::Empty,
                '.' => Seat::Floor,
                _ => unreachable!(),
            }
        }
    }
    Grid { grid, xsize, ysize }
}

pub fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    loop {
        let mut next_grid = grid.clone();
        let mut occupied = 0;
        for y in 0..input.ysize {
            for x in 0..input.xsize {
                let coord = x + y * input.xsize;
                if grid.grid[coord] == Seat::Floor {
                    continue;
                }
                let mut occupied_neighbors = 0;
                for dy in y.saturating_sub(1)..=y+1 {
                    for dx in x.saturating_sub(1)..=x+1 {
                        if (dx==x && dy==y) || dy >= grid.ysize || dx >= grid.xsize {
                            continue;
                        }
                        if grid.grid[dx + dy * grid.xsize] == Seat::Occupied {
                            occupied_neighbors += 1;
                        }
                    }
                }
                if occupied_neighbors == 0 && grid.grid[coord] == Seat::Empty {
                    next_grid.grid[coord] = Seat::Occupied;
                } else if occupied_neighbors >= 4 && grid.grid[coord] == Seat::Occupied {
                    next_grid.grid[coord] = Seat::Empty;
                }
                if grid.grid[coord] == Seat::Occupied {
                    occupied += 1;
                }
            }
        }

        if grid.grid == next_grid.grid {
            break occupied
        }
        grid = next_grid;
    }
}

pub fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    loop {
        let mut next_grid = grid.clone();
        let mut occupied = 0;
        for y in 0..input.ysize {
            for x in 0..input.xsize {
                let coord = x + y * input.xsize;
                if grid.grid[coord] == Seat::Floor {
                    continue;
                }
                let mut occupied_neighbors = 0;
                for dy in -1isize..=1 {
                    for dx in -1isize..=1 {
                        if dx==0 && dy==0 {
                            continue;
                        }
                        let mut neighbor_y = y as isize;
                        let mut neighbor_x = x as isize;
                        loop {
                            neighbor_y += dy;
                            neighbor_x += dx;
                            if neighbor_y as usize >= grid.ysize || neighbor_x as usize >= grid.xsize {
                                break;
                            }
                            match grid.grid[neighbor_x as usize + neighbor_y as usize * grid.xsize] {
                                Seat::Occupied => {
                                    occupied_neighbors += 1;
                                    break;
                                },
                                Seat::Empty => break,
                                _ => (),
                            }
                        }
                    }
                }
                if occupied_neighbors == 0 && grid.grid[coord] == Seat::Empty {
                    next_grid.grid[coord] = Seat::Occupied;
                } else if occupied_neighbors >= 5 && grid.grid[coord] == Seat::Occupied {
                    next_grid.grid[coord] = Seat::Empty;
                } else {
                    next_grid.grid[coord] = grid.grid[coord];
                }
                if grid.grid[coord] == Seat::Occupied {
                    occupied += 1;
                }
            }
        }

        if grid.grid == next_grid.grid {
            break occupied
        }
        std::mem::swap(&mut grid, &mut next_grid);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                let idx = (x + y * self.xsize) as usize;
                write!(f, "{}", self.grid[idx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Seat::Empty => write!(f, "L"),
            Seat::Floor => write!(f, "."),
            Seat::Occupied => write!(f, "#"),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;
        assert_eq!(part1(&parse(input)), 37);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;
        assert_eq!(part2(&parse(input)), 26);
    }



    
}
