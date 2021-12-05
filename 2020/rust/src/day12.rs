// Advent of Code
// Copyright (C) 2020  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Clone, Debug)]
pub struct Instruction {
    action: Action,
    value: isize,
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|l| {
            let value = l[1..].parse().unwrap();
            let action = match l.as_bytes()[0] {
                b'F' => Action::Forward,
                b'N' => Action::North,
                b'S' => Action::South,
                b'E' => Action::East,
                b'W' => Action::West,
                b'L' => Action::Left,
                b'R' => Action::Right,
                _ => unreachable!(),
            };
            Instruction { action, value }
        })
        .collect()
}

pub fn part1(input: &[Instruction]) -> isize {
    let mut pos = (0, 0);
    let mut dir = (1, 0);
    for instr in input {
        match instr.action {
            Action::North => pos.1 += instr.value,
            Action::South => pos.1 -= instr.value,
            Action::East => pos.0 += instr.value,
            Action::West => pos.0 -= instr.value,
            Action::Left => {
                for _ in 0..instr.value / 90 {
                    dir = (-dir.1, dir.0)
                }
            }
            Action::Right => {
                for _ in 0..instr.value / 90 {
                    dir = (dir.1, -dir.0)
                }
            }
            Action::Forward => {
                pos.0 += dir.0 * instr.value;
                pos.1 += dir.1 * instr.value;
            }
        }
    }
    pos.0.abs() + pos.1.abs()
}

pub fn part2(input: &[Instruction]) -> isize {
    let mut dir = (10_isize, 1_isize);
    let mut pos = (0, 0);
    for instr in input {
        match instr.action {
            Action::North => dir.1 += instr.value,
            Action::South => dir.1 -= instr.value,
            Action::East => dir.0 += instr.value,
            Action::West => dir.0 -= instr.value,
            Action::Left => {
                for _ in 0..instr.value / 90 {
                    dir = (-dir.1, dir.0)
                }
            }
            Action::Right => {
                for _ in 0..instr.value / 90 {
                    dir = (dir.1, -dir.0)
                }
            }
            Action::Forward => {
                pos.0 += dir.0 * instr.value;
                pos.1 += dir.1 * instr.value;
            }
        }
    }
    pos.0.abs() + pos.1.abs()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
F10
N3
F7
R90
F11
"#;
        assert_eq!(part1(&parse(input)), 25);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
F10
N3
F7
R90
F11
"#;
        assert_eq!(part2(&parse(input)), 286);
    }
}
