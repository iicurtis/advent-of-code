// Advent of Code
// Copyright (C) 2020  Isaac Curtis
type Error = Box<dyn std::error::Error>;
use num::integer::Integer;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Debug)]
pub struct Schedule {
    time: usize,
    buses: Vec<usize>,
}

pub fn parse(input: &str) -> Schedule {
    let mut input = input.trim().lines();
    let time = input.next().unwrap().parse().unwrap();
    let buses = input
        .next()
        .unwrap()
        .split(',')
        .map(|b| b.parse().unwrap_or(1))
        .collect();
    Schedule { time, buses }
}

pub fn part1(input: &Schedule) -> usize {
    let mut time_waiting = usize::max_value();
    let mut mybus = 0;
    for bus in input.buses.iter() {
        if *bus == 1 {
            continue;
        }
        let wait = input.time.div_ceil(bus) * bus - input.time;
        if time_waiting > wait {
            mybus = *bus;
            time_waiting = wait
        }
    }
    time_waiting * mybus
}

pub fn part2(input: &Schedule) -> usize {
    let mut time = 1;
    let mut product = 1;
    for (i, bus) in input.buses.iter().enumerate() {
        if *bus == 1 {
            continue;
        }
        let mut delta = -1 * i as isize;
        while delta < 0 {
            delta += *bus as isize
        }
        while time % *bus != delta as usize {
            time += product;
        }
        product *= *bus;
    }
    time
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
939
7,13,x,x,59,x,31,19
"#;
        assert_eq!(part1(&parse(input)), 295);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
939
7,13,x,x,59,x,31,19
"#;
        assert_eq!(part2(&parse(input)), 1068781);
    }
}
