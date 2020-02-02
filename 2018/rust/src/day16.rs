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

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Debug, Clone, Copy)]
struct Sample {
    before: [usize; 4],
    instr: [usize; 4],
    after: [usize; 4],
}

#[derive(Debug, Clone)]
pub struct Input {
    instructions: Vec<[usize; 4]>,
    samples: Vec<Sample>,
}

fn number(s: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), usize::from_str)(s)
}

fn before(s: &str) -> IResult<&str, [usize; 4]> {
    let (s, reg0) = preceded(tag("Before: ["), number)(s)?;
    let (s, reg1) = preceded(tag(", "), number)(s)?;
    let (s, reg2) = preceded(tag(", "), number)(s)?;
    let (s, reg3) = preceded(tag(", "), number)(s)?;
    let (s, _) = tag("]\n")(s)?;
    Ok((s, [reg0, reg1, reg2, reg3]))
}

fn after(s: &str) -> IResult<&str, [usize; 4]> {
    let (s, reg0) = preceded(tag("After:  ["), number)(s)?;
    let (s, reg1) = preceded(tag(", "), number)(s)?;
    let (s, reg2) = preceded(tag(", "), number)(s)?;
    let (s, reg3) = preceded(tag(", "), number)(s)?;
    let (s, _) = tag("]\n")(s)?;
    Ok((s, [reg0, reg1, reg2, reg3]))
}

fn instruction(s: &str) -> IResult<&str, [usize; 4]> {
    let (s, op) = terminated(number, tag(" "))(s)?;
    let (s, a) = terminated(number, tag(" "))(s)?;
    let (s, b) = terminated(number, tag(" "))(s)?;
    let (s, c) = number(s)?;
    let (s, _) = opt(tag("\n"))(s)?;
    Ok((s, [op, a, b, c]))
}

fn sample(s: &str) -> IResult<&str, Sample> {
    let (s, before) = before(s)?;
    let (s, instr) = instruction(s)?;
    let (s, after) = after(s)?;
    let (s, _) = tag("\n")(s)?;
    Ok((
        s,
        Sample {
            before,
            instr,
            after,
        },
    ))
}

fn instr(s: &str) -> IResult<&str, Input> {
    let (s, samples) = many1(sample)(s)?;
    let (s, _) = tag("\n\n")(s)?;
    let (s, instructions) = many1(instruction)(s)?;
    Ok((
        s,
        Input {
            samples,
            instructions,
        },
    ))
}

pub fn parse_input(input: &str) -> Input {
    let (_incomplete, parsed) = instr(input).expect("Couldn't parse input");
    parsed
}

const OPS: [fn(&mut [usize; 4], usize, usize, usize); 16] = [
    |r: &mut [usize; 4], a, b, c| r[c] = r[a] + r[b], //addr
    |r: &mut [usize; 4], a, b, c| r[c] = r[a] + b,    //addi
    |r: &mut [usize; 4], a, b, c| r[c] = r[a] * r[b], //mulr
    |r: &mut [usize; 4], a, b, c| r[c] = r[a] * b,    //muli
    |r: &mut [usize; 4], a, b, c| r[c] = r[a] & r[b], //banr
    |r: &mut [usize; 4], a, b, c| r[c] = r[a] & b,    //bani
    |r: &mut [usize; 4], a, b, c| r[c] = r[a] | r[b], //borr
    |r: &mut [usize; 4], a, b, c| r[c] = r[a] | b,    //bori
    |r: &mut [usize; 4], a, _b, c| r[c] = r[a],       //setr
    |r: &mut [usize; 4], a, _b, c| r[c] = a,          //seti
    |r: &mut [usize; 4], a, b, c| if a > r[b] { r[c] = 1 } else { r[c] = 0 }, //gtir
    |r: &mut [usize; 4], a, b, c| if r[a] > b { r[c] = 1 } else { r[c] = 0 }, //gtri
    |r: &mut [usize; 4], a, b, c| if r[a] > r[b] { r[c] = 1 } else { r[c] = 0 }, //gtrr
    |r: &mut [usize; 4], a, b, c| if a == r[b] { r[c] = 1 } else { r[c] = 0 }, //eqir
    |r: &mut [usize; 4], a, b, c| if r[a] == b { r[c] = 1 } else { r[c] = 0 }, //eqri
    |r: &mut [usize; 4], a, b, c| if r[a] == r[b] { r[c] = 1 } else { r[c] = 0 }, //eqrr
];

pub fn part1(input: &Input) -> usize {
    let mut three_or_more = 0;
    for i in input.samples.iter() {
        let ia = i.instr[1];
        let ib = i.instr[2];
        let ic = i.instr[3];
        let mut possibles = 0;
        for f in OPS.iter() {
            let mut reg = i.before;
            f(&mut reg, ia, ib, ic);
            if reg == i.after {
                possibles += 1;
                if possibles >= 3 {
                    three_or_more += 1;
                    break;
                }
            }
        }
    }
    three_or_more
}

pub fn part2(input: &Input) -> usize {
    let mut possibles = [65535u32; 16];
    let mut solutions = [16usize; 16];
    for sample in input.samples.iter() {
        let ia = sample.instr[1];
        let ib = sample.instr[2];
        let ic = sample.instr[3];
        let mut map = possibles[sample.instr[0]];
        let mut possible_ops = 0u32;
        let mut i = 0;
        while map > 0 {
            if (map & 1) == 0 {
                map >>= 1;
                i += 1;
                continue;
            }
            let mut reg = sample.before;
            OPS[i](&mut reg, ia, ib, ic);
            if reg == sample.after {
                possible_ops |= 1 << i;
            }
            map >>= 1;
            i += 1;
        }
        possibles[sample.instr[0]] &= possible_ops;
        if possibles[sample.instr[0]].count_ones() == 1 {
            let n = possibles[sample.instr[0]].trailing_zeros();
            solutions[sample.instr[0]] = n as usize;
            for v in &mut possibles {
                *v &= !(1 << n);
            }
        }
    }

    let mut reg = [0, 0, 0, 0];
    for prog in input.instructions.iter() {
        let ia = prog[1];
        let ib = prog[2];
        let ic = prog[3];
        OPS[solutions[prog[0]]](&mut reg, ia, ib, ic);
    }
    reg[0]
}
