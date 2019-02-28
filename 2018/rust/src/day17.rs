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

// use nom::types::CompleteStr;
// use nom::*;
// use std::str::FromStr;

type Error = Box<std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    // let input = parse_input(input);
    // let soln1 = part1(&input);
    // let soln2 = part2(&input);
    let soln1 = "Day17 not solved yet";
    let soln2 = "Day17 not solved yet";
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

// #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
// struct Point {
    // x: usize,
    // y: usize,
// }

// #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
// struct PointRange {
    // x0: usize,
    // y0: usize,
    // x1: usize,
    // y1: usize,
// }

// named!(number(CompleteStr) -> usize, map_res!(recognize!(digit), |CompleteStr(s)| usize::from_str(s)));

// named!(scanline(CompleteStr) -> PointRange,
// do_parse!(
    // axis: alt!(tag!("x") | tag!("y")) >>
    // tag!("=") >>
    // coord: number >>
    // tag!(", ") >>
    // alt!(tag!("x=") | tag!("y=")) >>
    // range_start: number >>
    // tag!("..") >>
    // range_end: number >>
    // tag!("\n") >> ( {
        // if axis == CompleteStr("x") {
            // PointRange{x0: coord, x1: coord, y0: range_start, y1: range_end}
        // } else {
            // PointRange{y0: coord, y1: coord, x0: range_start, x1: range_end}
        // }
    // })));

// #[derive(Debug, Clone)]
// pub struct ParseError;

// impl std::str::FromStr for PointRange {
    // type Err = ParseError;

    // fn from_str(input: &str) -> Result<Self, Self::Err> {
        // match scanline(CompleteStr(input)) {
            // Ok((CompleteStr(""), points)) => Ok(points),
            // _ => Err(ParseError),
        // }
    // }
// }

// #[aoc_generator(day17)]
// fn parse_input(input: &str) -> Vec<Point> {
    // let mut points = input
        // .lines()
        // .map(|line| line.trim().parse())
        // .collect::<Result<Vec<Point>, _>>()
        // .unwrap();
    // let (_incomplete, parsed) = instr(CompleteStr(input)).expect("Couldn't parse input");
    // return parsed;
// }


// #[aoc(day17, part1)]
// fn part1(input: &Input) -> usize {
    // let mut three_or_more = 0;
    // for i in input.samples.iter() {
        // let a = i.instr[1];
        // let b = i.instr[2];
        // let c = i.instr[3];
        // let mut possibles = 0;
        // for f in OPS.iter() {
            // let mut reg = i.before.clone();
            // f(&mut reg, a, b, c);
            // if reg == i.after {
                // possibles += 1;
                // if possibles >= 3 {
                    // three_or_more += 1;
                    // break;
                // }
            // }
        // }
    // }
    // return three_or_more;
// }


