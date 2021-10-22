// Advent of Code
// Copyright (C) 2020  Isaac Curtis

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> World {
    let input = input.trim().lines();
    let width = input.clone().next().unwrap().bytes().count();
    let rows: Vec<Vec<usize>> = input
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| i)
                .collect()
        })
        .collect();
    World { width, rows }
}

fn count_trees(world: &World, down: usize, right: usize) -> usize {
    let mut ntrees = 0;
    let mut col = 0;
    for row in world.rows.iter().step_by(down) {
        if row.contains(&col) {
            ntrees += 1
        };
        col = (col + right) % world.width;
    }
    ntrees
}

pub fn part1(input: &World) -> usize {
    count_trees(input, 1, 3)
}

pub fn part2(input: &World) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|(right, down)| count_trees(input, *down, *right))
        .product()
}

pub struct World {
    width: usize,
    rows: Vec<Vec<usize>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;
        assert_eq!(part1(&parse_input(&input)), 7);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;
        assert_eq!(part2(&parse_input(&input)), 336);
    }
}
