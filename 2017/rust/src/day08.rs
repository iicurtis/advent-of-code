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

use nom::types::CompleteByteSlice;
use nom::{alpha, digit, space};
use std::cmp;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str;

#[derive(Debug)]
struct Instruction<'a> {
    reg1: &'a str,
    op1: &'a str,
    val1: i64,
    reg2: &'a str,
    op2: &'a str,
    val2: i64,
}

named!(register<CompleteByteSlice, &str>, map_res!(alpha, |CompleteByteSlice(s)| str::from_utf8(s) ));

named!(
    number<CompleteByteSlice, i64>,
    map_res!(
        map_res!(recognize!(pair!(opt!(tag!("-")), digit)), |CompleteByteSlice(s)| str::from_utf8(s) ),
        str::parse
    )
);

named!(
    cmpop<CompleteByteSlice, &str>,
    map_res!(
        alt!(tag_s!("<=") | tag_s!("<") | tag_s!(">=") | tag_s!(">") | tag_s!("==") | tag_s!("!=")),
        |CompleteByteSlice(s)| str::from_utf8(s)
    )
);

named!(
    parse_line<CompleteByteSlice, Instruction>,
    do_parse!(
        reg: register
        >> space
        >> act: register
        >> space
        >> val: number
        >> tag!(" if ")
        >> oth_reg: register
        >> space
        >> cmp: cmpop
        >> space
        >> oth_val: number
        >> (Instruction {
            reg1: reg,
            op1: act,
            val1: val,
            reg2: oth_reg,
            op2: cmp,
            val2: oth_val
        })
)
);

pub fn solve() {
    // Get line from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines();

    let mut registers = HashMap::<String, i64>::new();
    let mut maxseen = 0;

    for line in input {
        let line = line.unwrap();
        let (_, instructions) = parse_line(CompleteByteSlice(line.as_bytes())).ok().unwrap();
        // println!("{:?}", instructions);

        let reg2 = *registers.get(instructions.reg2).unwrap_or(&0);

        let cond = match instructions.op2 {
            "<=" => reg2 <= instructions.val2,
            "<" => reg2 < instructions.val2,
            ">=" => reg2 >= instructions.val2,
            ">" => reg2 > instructions.val2,
            "==" => reg2 == instructions.val2,
            "!=" => reg2 != instructions.val2,
            _ => panic!("unknown operator"),
        };

        if cond {
            let reg1 = registers.entry(instructions.reg1.to_string()).or_insert(0);

            match instructions.op1 {
                "inc" => *reg1 += instructions.val1,
                "dec" => *reg1 -= instructions.val1,
                _ => panic!("unknown operator"),
            }
            maxseen = cmp::max(maxseen, *reg1);
        }
    }

    println!(
        "[Day 08][Part 1] ANS is: {}",
        registers.values().cloned().max().unwrap()
    );

    println!("[Day 08][Part 2] ANS is: {}", maxseen);
}
