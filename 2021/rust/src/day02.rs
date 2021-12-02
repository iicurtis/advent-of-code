// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub struct Command {
    command: Direction,
    value: usize,
}

enum Direction {
    Forward,
    Down,
    Up,
}

pub fn parse(input: &str) -> Vec<Command> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            let command = splits.next().unwrap();
            let command = match command {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => unreachable!(),
            };
            let value = splits.next().unwrap().parse().unwrap();
            Command { command, value }
        })
        .collect()
}

pub fn part1(input: &[Command]) -> usize {
    let mut posx = 0;
    let mut posy = 0;
    for command in input {
        match command.command {
            Direction::Forward => posx += command.value,
            Direction::Down => posy += command.value,
            Direction::Up => posy -= command.value,
        }
    }
    posx * posy
}

pub fn part2(input: &[Command]) -> usize {
    let mut posx = 0;
    let mut posy = 0;
    let mut aim = 0;
    for command in input {
        match command.command {
            Direction::Forward => {
                posy += command.value;
                posx += aim * command.value;
            },
            Direction::Down => aim += command.value,
            Direction::Up => aim -= command.value,
        }
    }
    posx * posy
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
        assert_eq!(part1(&parse(input)), 150);
    }


    #[test]
    fn test_part2_0() {
        let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
        assert_eq!(part2(&parse(input)), 900);
    }



}
