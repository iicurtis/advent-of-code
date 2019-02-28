// Advent of Code
// Copyright (C) 2018  Isaac Curtis

use std::fmt::{self, Display};

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

struct CPU {
    instructions: Vec<fn(&mut [usize; 6], usize, usize, usize)>,
    ip: u8,
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Box<CPU> {
    let lines = input.trim().lines();
    let line = lines.next().unwrap();
    let ip = line.as_bytes()[4] & 3;

    for line in lines.iter() {
        instr = line.split(' ');
    }

    return Box::new(CPU { instructions, ip });
}

#[aoc(day19, part1)]
fn part1(input: &CPU) -> usize {
    let mut world = input.clone();
    let ans = world.run_steps(10);
    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
"#;
        assert_eq!(part1(&parse_input(input)), 7);
    }

}
