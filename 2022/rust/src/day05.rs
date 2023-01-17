// Advent of Code Solutions
// Copyright (C) 2022  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(input: &str) -> String {
    let (stack_input, proc_input) = input.split_once("\n\n").unwrap();
    let stack_lines = stack_input.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();
    let mut stacks = vec![vec![]; (stack_lines.last().unwrap().len() + 1) / 4];
    for &line in &stack_lines[..stack_lines.len()-1] {
        for i in 0..stacks.len() {
            let c = line[i*4+1];
            if c != b' ' {
                stacks[i].push(char::from(c));
            }
        }
    }
    for stack in &mut stacks {  // stacks are upside-down
        stack.reverse();
    }

    for line in proc_input.lines() {
        let command: Vec<&str> = line.split_whitespace().collect();
        let num = command[1].parse::<usize>().unwrap();
        let from = command[3].parse::<usize>().unwrap() - 1;
        let to = command[5].parse::<usize>().unwrap() - 1;
        for _ in 0..num {
            let top = stacks[from].pop().unwrap();
            stacks[to].push(top);
        }
    }
    stacks.iter().map(|c| *c.last().unwrap()).collect::<String>()
}

pub fn part2(input: &str) -> String {
    let (stack_input, proc_input) = input.split_once("\n\n").unwrap();
    let stack_lines = stack_input.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();
    let mut stacks = vec![vec![]; (stack_lines.last().unwrap().len() + 1) / 4];
    for &line in &stack_lines[..stack_lines.len()-1] {
        for i in 0..stacks.len() {
            let c = line[i*4+1];
            if c != b' ' {
                stacks[i].push(char::from(c));
            }
        }
    }
    for stack in &mut stacks {  // stacks are upside-down
        stack.reverse();
    }

    for line in proc_input.lines() {
        let command: Vec<&str> = line.split_whitespace().collect();
        let num = command[1].parse::<usize>().unwrap();
        let from = command[3].parse::<usize>().unwrap() - 1;
        let to = command[5].parse::<usize>().unwrap() - 1;
        // let top = stacks[from].drain(..num).rev().collect();
        for _ in 0..num {
            let top = stacks[from].pop().unwrap();
            stacks[to].push(top);
        }
        let moved = stacks[to].len() - num;
        stacks[to][moved..].reverse();
    }
    stacks.iter().map(|c| *c.last().unwrap()).collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
        assert_eq!(part1(input), "CMZ");
    }

    #[test]
    fn test_part2_0() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
        assert_eq!(part2(input), "MCD");
    }
}
