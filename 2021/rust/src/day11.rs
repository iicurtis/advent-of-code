// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| b - b'0').collect::<Vec<u8>>())
        .collect()
}

fn flash_stack(input: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let mut stack = vec![(x, y)];
    while let Some((x, y)) = stack.pop() {
        if input[x][y] == u8::MAX {
            continue;
        }
        input[x][y] = u8::MAX;

        // push adjacent to stack
        for x2 in x as isize - 1..=x as isize + 1 {
            for y2 in y as isize - 1..=y as isize + 1 {
                if x2 < 0 || x2 >= input[0].len().try_into().unwrap() {
                    continue;
                }
                if y2 < 0 || y2 >= input.len().try_into().unwrap() {
                    continue;
                }
                if (x == x2.try_into().unwrap()) && (y == y2.try_into().unwrap()) {
                    continue;
                }

                if input[x2 as usize][y2 as usize] <= 9 {
                    input[x2 as usize][y2 as usize] += 1;
                }

                if input[x2 as usize][y2 as usize] == 10 {
                    stack.push((x2.try_into().unwrap(), y2.try_into().unwrap()));
                }
            }
        }
    }
}

pub fn part1(input: &[Vec<u8>]) -> usize {
    let mut input = input.to_owned();
    let mut soln = 0;
    for _iter in 0..100 {
        input.iter_mut().flatten().for_each(|x| *x += 1); // increase each number
        for x in 0..input[0].len() {
            for y in 0..input.len() {
                if input[x][y] == 10 {
                    flash_stack(&mut input, x, y);
                }
            }
        }
        for x in input.iter_mut().flatten() {
            if *x == u8::MAX {
                soln += 1;
                *x = 0;
            }
        }
    }
    soln
}

pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut input = input.to_owned();
    let mut soln = 0;
    let mut iter = 0;
    while soln != input.len() * input[0].len() {
        soln = 0;
        iter += 1;
        input.iter_mut().flatten().for_each(|x| *x += 1); // increase each number
        for x in 0..input[0].len() {
            for y in 0..input.len() {
                if input[x][y] == 10 {
                    flash_stack(&mut input, x, y);
                }
            }
        }
        for x in input.iter_mut().flatten() {
            if *x == u8::MAX {
                soln += 1;
                *x = 0;
            }
        }
    }
    iter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;
        assert_eq!(part1(&parse(input)), 1656);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;
        assert_eq!(part2(&parse(input)), 195);
    }
}
