// Advent of Code
// Copyright (C) 2018  Isaac Curtis

use std::collections::VecDeque;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let (soln1, soln2) = day20(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

// START HERE

#[derive(Clone)]
struct State {
    cell: u16,
    dist: u16,
}

struct Goldfish {
    xcoord: VecDeque<u8>,
    ycoord: VecDeque<u8>,
    value: u32,
}

impl Goldfish {
    fn remember(&mut self, state: &mut State) -> bool {
        let xv = (state.cell & 0x00ff) as u8;
        let yv = (state.cell >> 8) as u8;
        for i in 0..15 {
            if xv == self.xcoord[i] && yv == self.ycoord[i] {
                let delta = ((state.dist as u32).wrapping_sub(self.value >> i)) & 2;
                state.dist = (state.dist as u32).wrapping_add(delta.wrapping_sub(1)) as u16;
                return true;
            }
        }

        self.xcoord.push_front(xv);
        self.ycoord.push_front(yv);
        self.xcoord.pop_back();
        self.ycoord.pop_back();
        self.value = (self.value << 2) | (state.dist & 3) as u32;
        return false;
    }

    fn new() -> Self {
        Self {
            xcoord: VecDeque::from(vec![0u8; 16]),
            ycoord: VecDeque::from(vec![0u8; 16]),
            value: 0,
        }
    }
}

pub fn day20(input: &str) -> (usize, usize) {
    let inputiter = input.trim().as_bytes();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut stack = Vec::with_capacity(300);
    let mut state = State {
        cell: 0x8080,
        dist: 0,
    };
    let mut goldfish = Goldfish::new();
    for dir in inputiter {
        match dir {
            b'N' => {
                state.cell = state.cell.wrapping_add(0x0100);
                if !goldfish.remember(&mut state) {
                    state.dist += 1;
                    part1 = std::cmp::max(part1, state.dist);
                    part2 += if state.dist >= 1000 { 1 } else { 0 };
                }
            }
            b'S' => {
                state.cell = state.cell.wrapping_add(0xff00);
                if !goldfish.remember(&mut state) {
                    state.dist += 1;
                    part1 = std::cmp::max(part1, state.dist);
                    part2 += if state.dist >= 1000 { 1 } else { 0 };
                }
            }
            b'E' => {
                state.cell = state.cell.wrapping_add(0x0001);
                if !goldfish.remember(&mut state) {
                    state.dist += 1;
                    part1 = std::cmp::max(part1, state.dist);
                    part2 += if state.dist >= 1000 { 1 } else { 0 };
                }
            }
            b'W' => {
                state.cell = state.cell.wrapping_add(0xffff);
                if !goldfish.remember(&mut state) {
                    state.dist += 1;
                    part1 = std::cmp::max(part1, state.dist);
                    part2 += if state.dist >= 1000 { 1 } else { 0 };
                }
            }
            b'(' => stack.push(state.clone()),
            b'|' => {
                state = stack.last().cloned().unwrap();
            }
            b')' => {
                state = stack.pop().unwrap();
            }
            b'^' => continue,
            b'$' => break,
            _ => unreachable!(),
        }
    }
    (part1 as usize, part2 as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"#;
        assert_eq!(day20(&input).0, 18);
    }
}
