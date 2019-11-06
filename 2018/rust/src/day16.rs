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

use nom::types::CompleteStr;
use nom::*;
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

named!(number(CompleteStr) -> usize, map_res!(recognize!(digit), |CompleteStr(s)| usize::from_str(s)));

named!(before(CompleteStr) -> [usize; 4], do_parse!( tag!("Before: [") >>
                                                     reg0: number >> tag!(", ") >> reg1: number >> tag!(", ") >> reg2: number >>
                                                     tag!(", ") >> reg3: number >> tag!("]\n") >>
                                                     ([reg0, reg1, reg2, reg3])));

named!(after(CompleteStr) -> [usize; 4],
do_parse!(
    tag!("After:  [") >>
    reg0: number >>
    tag!(", ") >>
    reg1: number >>
    tag!(", ") >>
    reg2: number >>
    tag!(", ") >>
    reg3: number >>
    tag!("]\n") >>
    ([reg0, reg1, reg2, reg3])));

named!(instruction(CompleteStr) -> [usize; 4],
do_parse!(
    op: number >>
    tag!(" ") >>
    a: number >>
    tag!(" ") >>
    b: number >>
    tag!(" ") >>
    c: number >>
    opt!(tag!("\n")) >>
    ([op, a, b, c])));

named!(sample(CompleteStr) -> Sample,
do_parse!(
    before: before >>
    instr: instruction >>
    after: after >>
    tag!("\n") >>
    (Sample {before, instr, after})
    ));

named!(instr(CompleteStr) -> Input,
do_parse!(
    samples: many1!( sample ) >>
    tag!("\n\n") >>
    instructions: many1!( instruction ) >>
    ( Input{ samples, instructions } )
    ));

pub fn parse_input(input: &str) -> Input {
    let (_incomplete, parsed) = instr(CompleteStr(input)).expect("Couldn't parse input");
    return parsed;
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
        let a = i.instr[1];
        let b = i.instr[2];
        let c = i.instr[3];
        let mut possibles = 0;
        for f in OPS.iter() {
            let mut reg = i.before.clone();
            f(&mut reg, a, b, c);
            if reg == i.after {
                possibles += 1;
                if possibles >= 3 {
                    three_or_more += 1;
                    break;
                }
            }
        }
    }
    return three_or_more;
}

pub fn part2(input: &Input) -> usize {
    let mut possibles = [65535u32; 16];
    let mut solutions = [16usize; 16];
    for sample in input.samples.iter() {
        let a = sample.instr[1];
        let b = sample.instr[2];
        let c = sample.instr[3];
        let mut map = possibles[sample.instr[0]];
        let mut possible_ops = 0u32;
        let mut i = 0;
        while map > 0 {
            if (map & 1) == 0 {
                map >>= 1;
                i += 1;
                continue;
            }
            let mut reg = sample.before.clone();
            OPS[i](&mut reg, a, b, c);
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
        let a = prog[1];
        let b = prog[2];
        let c = prog[3];
        OPS[solutions[prog[0]]](&mut reg, a, b, c);
    }
    return reg[0];
}

