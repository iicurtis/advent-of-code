// Advent of Code
// Copyright (C) 2020  Isaac Curtis

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let op = &line[..3];
            let value = line[4..].parse::<isize>().unwrap();
            let mut accumulator = 0;
            let mut next = i as isize + 1;
            let mut modified = next;
            match op {
                "acc" => {
                    accumulator = value;
                }
                "jmp" => {
                    next = next + value - 1;
                }
                "nop" => {
                    modified = next + value - 1;
                }
                _ => panic!(),
            };
            Instruction {
                accumulator,
                next,
                modified,
                executed: false,
            }
        })
        .collect()
}

pub fn part1(input: &[Instruction]) -> isize {
    let mut input = input.to_vec();
    let mut accumulator = 0;
    let mut i = 0;
    loop {
        let instr = &mut input[i as usize];
        if instr.executed {
            break accumulator;
        }
        instr.executed = true;
        accumulator += instr.accumulator;
        i = instr.next;
    }
}

pub fn part2(input: &[Instruction]) -> isize {
    let mut input = input.to_vec();
    let mut modified_stack = vec![(0isize, 0)];
    let mut accumulator = 0;
    let mut i = 0;
    // loop over original unmodified instructions once
    // to build stack of modified locations
    loop {
        let instr = &mut input[i as usize];
        if instr.executed {
            break;
        }
        instr.executed = true;
        accumulator += instr.accumulator;
        i = instr.next;
        if instr.next != instr.modified {
            modified_stack.push((instr.modified, accumulator));
        }
    }

    while let Some((i, mut accumulator)) = modified_stack.pop() {
        if i as usize >= input.len() {
            return accumulator;
        };
        let instr = &mut input[i as usize];
        if instr.executed {
            continue;
        }
        instr.executed = true;
        accumulator += instr.accumulator;
        modified_stack.push((instr.next, accumulator));
    }
    panic!("Answer not found in stack")
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    next: isize,
    modified: isize,
    accumulator: isize,
    executed: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#;
        assert_eq!(part1(&parse_input(&input)), 5);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#;
        assert_eq!(part2(&parse_input(&input)), 8);
    }
}
