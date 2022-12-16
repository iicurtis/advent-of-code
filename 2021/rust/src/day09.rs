// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;
use itertools::Itertools;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub struct Grid {
    xsize: usize,
    ysize: usize,
    grid: Vec<u8>,
}

pub fn parse(input: &str) -> Grid {
    let mut xsize = 0;
    let mut ysize = 0;
    let grid = input
        .trim()
        .lines()
        .flat_map(|l| {
            xsize = l.len();
            ysize += 1;
            l.as_bytes().iter().map(|b| b - b'0').collect::<Vec<u8>>()
        })
        .collect();
    Grid { xsize, ysize, grid }
}

pub fn part1(input: &Grid) -> usize {
    let mut sum: usize = 0;
    // naive check each point
    for y in 0..input.ysize {
        for x in 0..input.xsize {
            let idx = x + y * input.xsize;
            // check point before, after
            if x > 0 && input.grid[idx] >= input.grid[idx - 1] {
                continue;
            }
            if x < input.xsize - 1 && input.grid[idx] >= input.grid[idx + 1] {
                continue;
            }
            if y > 0 && input.grid[idx] >= input.grid[idx - input.xsize] {
                continue;
            }
            if y < input.ysize - 1 && input.grid[idx] >= input.grid[idx + input.xsize] {
                continue;
            }
            sum += 1 + input.grid[idx] as usize;
        }
    }
    sum
}

pub fn part2(input: &Grid) -> usize {
    let mut low_points: Vec<usize> = Vec::new();
    // naive check each point
    for y in 0..input.ysize {
        for x in 0..input.xsize {
            let idx = x + y * input.xsize;
            // check point before, after
            if x > 0 && input.grid[idx] >= input.grid[idx - 1] {
                continue;
            }
            if x < input.xsize - 1 && input.grid[idx] >= input.grid[idx + 1] {
                continue;
            }
            if y > 0 && input.grid[idx] >= input.grid[idx - input.xsize] {
                continue;
            }
            if y < input.ysize - 1 && input.grid[idx] >= input.grid[idx + input.xsize] {
                continue;
            }
            low_points.push(idx);
        }
    }
    low_points
        .into_iter()
        .map(|idx| {
            let xs = input.xsize;
            let mut stack = vec![idx];
            let mut visited = std::collections::HashSet::new();
            while let Some(i) = stack.pop() {
                if !visited.insert(i) {
                    continue;
                }
                if i % xs > 0 && input.grid[i - 1] != 9 && input.grid[i - 1] > input.grid[i] {
                    stack.push(i - 1);
                }
                if i % xs < xs - 1 && input.grid[i + 1] != 9 && input.grid[i + 1] > input.grid[i] {
                    stack.push(i + 1);
                }
                if i / xs > 0 && input.grid[i - xs] != 9 && input.grid[i - xs] > input.grid[i] {
                    stack.push(i - xs);
                }
                if i / xs < input.ysize - 1
                    && input.grid[i + xs] != 9
                    && input.grid[i + xs] > input.grid[i]
                {
                    stack.push(i + xs);
                }
            }
            visited
        })
        .map(|basin| basin.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;
        assert_eq!(part1(&parse(input)), 15);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;
        assert_eq!(part2(&parse(input)), 1134);
    }
}
