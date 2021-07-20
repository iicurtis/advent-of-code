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
        .map(|line| {
            let value = line[4..].parse::<isize>().unwrap();
            let op = &line[..3];
            match op {
                "acc" => Instruction::Acc(value),
                "jmp" => Instruction::Jmp(value),
                "nop" => Instruction::Nop(value),
                _ => panic!(),
            }
        })
        .collect()
}

fn is_loop(input: &[Instruction]) -> (bool, isize) {
    let mut instr_executed = input.iter().map(|line| (false, line)).collect::<Vec<_>>();
    let mut accumulator = 0;
    let mut i = 0;
    loop {
        let (executed, instr) = &mut instr_executed[i as usize];
        if *executed { break (true, accumulator) }
        *executed = true;
        match instr {
            Instruction::Jmp(val) => i += val,
            Instruction::Acc(val) => { accumulator += val; i += 1 },
            _ => i += 1,
        }
        if i as usize >= input.len() {
            break (false, accumulator)
        }
    }
}

pub fn part1(input: &[Instruction]) -> isize {
    is_loop(input).1
}

pub fn part2(input: &[Instruction]) -> isize {
    let mut input_modified = input.to_vec();
    for i in 0..input.len() {
        let old = input[i];
        input_modified[i] = match input[i] {
            Instruction::Jmp(val) => Instruction::Nop(val),
            Instruction::Nop(val) => Instruction::Jmp(val),
            _ => input[i]
        };
        let result = is_loop(&input_modified);
        if !result.0 { return result.1 }
        input_modified[i] = old;
    }
    0
}

#[derive(Debug,Clone,Copy)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
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
