// Advent of Code
// Copyright (C) 2018  Isaac Curtis

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use hashbrown::{HashMap, HashSet};
use std::collections::{BTreeSet, BinaryHeap};
use std::fmt::{self, Display};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Instruction {
    a: char,
    b: char,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Step {} requires {}", self.a, self.b)
    }
}

mod parsers {
    use super::Instruction;
    use nom::*;
    use std::str::FromStr;

    named!(instruction(&str) -> Instruction,
    do_parse!(
        tag!("Step ") >>
        a: map_res!(alpha, char::from_str) >>
        tag!(" must be finished before step ") >>
        b: map_res!(alpha, char::from_str) >>
        tag!(" can begin.") >>
        (Instruction { a, b })
        )
    );

    #[derive(Debug, Clone)]
    pub struct ParseError;

    impl std::str::FromStr for Instruction {
        type Err = ParseError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match instruction(input) {
                Ok(("", instruction)) => Ok(instruction),
                _ => Err(ParseError),
            }
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()
        .unwrap()
}

pub fn part1(input: &[Instruction]) -> String {
    let mut finished: Vec<char> = Vec::new();
    let mut dependents = HashMap::new();
    let mut precursors: HashMap<char, HashSet<char>> = HashMap::new();
    // Build dependency tree
    for l in input {
        dependents.entry(l.a).or_insert_with(Vec::new).push(l.b);
        precursors
            .entry(l.b)
            .or_insert_with(HashSet::new)
            .insert(l.a);
        precursors.entry(l.a).or_insert_with(HashSet::new);
    }

    // Push all instructions that have no dependencies.
    // Use BTree because it is self sorting
    let mut todo = precursors
        .iter()
        .filter_map(|(&a, b)| if b.is_empty() { Some(a) } else { None })
        .collect::<BTreeSet<_>>();

    for r in todo.iter() {
        precursors.remove(r);
    }

    while !todo.is_empty() {
        let next = *todo.iter().next().unwrap();
        todo.remove(&next);
        finished.push(next);
        // Look at each instruction dependent on next
        if let Some(dep) = dependents.get(&next) {
            for &job in dep {
                let pre = precursors.get_mut(&job).unwrap();
                pre.remove(&next);
                if pre.is_empty() {
                    todo.insert(job);
                }
            }
        }
    }
    finished.into_iter().collect()
}

pub fn part2(input: &[Instruction]) -> i32 {
    let mut dependents = HashMap::new();
    let mut precursors: HashMap<char, HashSet<char>> = HashMap::new();
    // Build dependency tree
    for l in input {
        dependents.entry(l.a).or_insert_with(Vec::new).push(l.b);
        precursors
            .entry(l.b)
            .or_insert_with(HashSet::new)
            .insert(l.a);
        precursors.entry(l.a).or_insert_with(HashSet::new);
    }

    // Push all instructions that have no dependencies.
    // Use BTree because it is self sorting
    let mut todo = precursors
        .iter()
        .filter_map(|(&a, b)| if b.is_empty() { Some(a) } else { None })
        .collect::<BTreeSet<_>>();

    for r in todo.iter() {
        precursors.remove(r);
    }

    let mut time = 0;
    let nworkers = 5;
    let time_offset = 60;
    let mut workers = BinaryHeap::new();
    while !(todo.is_empty() && workers.is_empty()) {
        while workers.len() < nworkers && !todo.is_empty() {
            let next = *todo.iter().next().unwrap();
            todo.remove(&next);
            let completion_time = time - time_offset - (next as i32 - i32::from(b'A') + 1);
            workers.push((completion_time, next));
        }
        let (t, next) = workers.pop().unwrap();
        time = t;
        // Look at each instruction dependent on next
        if let Some(dep) = dependents.get(&next) {
            for &job in dep {
                let pre = precursors.get_mut(&job).unwrap();
                pre.remove(&next);
                if pre.is_empty() {
                    todo.insert(job);
                }
            }
        }
    }
    -time
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
"#;
        assert_eq!(part1(&parse_input(input)), "CABDFE");
    }

    // PART 2 BROKEN. NUM WORKERS WRONG
    // #[test]
    // fn test_part2() {
    // let input = r#"
    // Step C must be finished before step A can begin.
    // Step C must be finished before step F can begin.
    // Step A must be finished before step B can begin.
    // Step A must be finished before step D can begin.
    // Step B must be finished before step E can begin.
    // Step D must be finished before step E can begin.
    // Step F must be finished before step E can begin.
    // "#;
    // assert_eq!(part2(&parse_input(input)), 15);
    // }
}
