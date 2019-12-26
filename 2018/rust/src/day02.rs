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

use std::str;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    // let soln2 = part2_simd(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(input: &str) -> u32 {
    let mut frequencies = [0u8; 256];
    let (mut twos, mut threes) = (0, 0);
    for line in input.lines() {
        for f in frequencies.iter_mut() {
            *f = 0;
        }
        for b in line.as_bytes().iter().map(|&b| b as usize) {
            frequencies[b] = frequencies[b].saturating_add(1);
        }
        if frequencies.iter().any(|&f| f == 2) {
            twos += 1;
        }
        if frequencies.iter().any(|&f| f == 3) {
            threes += 1;
        }
    }
    twos * threes
}

// pub fn part2_simd<'a>(input: &'a str) -> String {
// use packed_simd::u8x32;
// use std::hint::unreachable_unchecked;
// let lines = input.lines();

// #[repr(align(32))]
// #[derive(Copy, Clone)]
// struct Line([u8; 32]);

// let mut storage = [u8x32::splat(0); 250];
// let mut buf = Line([0; 32]);
// for (storage, line) in storage.iter_mut().zip(lines) {
// let line = line.trim_end();
// buf.0[..line.len()].copy_from_slice(line.as_bytes());
// *storage = u8x32::from_slice_aligned(&buf.0);
// }

// for (i, &a) in storage.iter().enumerate() {
// for &b in &storage[i + 1..] {
// if a.eq(b)
// .select(u8x32::splat(1), u8x32::splat(0))
// .wrapping_sum()
// == 31
// {
// let mut buf = String::with_capacity(25);
// let a: [u8; 32] = a.into();
// let b: [u8; 32] = b.into();
// for (&a, &b) in a.iter().zip(&b) {
// if a == b && a != 0 {
// buf.push(a as char);
// }
// }
// return buf;
// }
// }
// }
// unsafe { unreachable_unchecked() };
// }

pub fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let line_len = 26;
    let mut parts = hashbrown::HashSet::with_capacity(250);

    for split in 0..line_len {
        for line in lines.clone() {
            let line = line.as_bytes();
            let (left, right) = (&line[..split], &line[split + 1..]);
            if !parts.insert((left, right)) {
                let left_str = std::str::from_utf8(left).unwrap();
                let right_str = std::str::from_utf8(right).unwrap();
                return format!("{}{}", left_str, right_str);
            }
        }
    }

    panic!("No solution")
}

// #[cfg(target_arch = "x86_64")]
// #[target_feature(enable = "avx2")]
// pub unsafe fn part2_unsafe<'a>(input: &'a str) -> String {
// #[cfg(target_arch = "x86_64")]
// use std::arch::x86_64::*;
// let lines = input.lines();

// #[repr(align(32))]
// #[derive(Copy, Clone)]
// struct Line([u8; 32]);

// let mut storage: [__m256i; 250] = [_mm256_setzero_si256(); 250];
// let mut buf = Line([0; 32]);
// for (storage, line) in storage.iter_mut().zip(lines) {
// let line = line.trim_end();
// buf.0[..line.len()].copy_from_slice(line.as_bytes());
// *storage = _mm256_load_si256(std::mem::transmute::<&[u8], &__m256i>(&buf.0));
// }

// for (i, &a) in storage.iter().enumerate() {
// for &b in &storage[i + 1..] {
// let cmp_ne = !_mm256_movemask_epi8(_mm256_cmpeq_epi8(a, b));
// let diff1 = _blsi_u32(cmp_ne);
// if (diff1 == cmp_ne) {
// let mut buf = String::with_capacity(25);
// let a: [u8; 32] = a.into();
// let b: [u8; 32] = b.into();
// for (&a, &b) in a.iter().zip(&b) {
// if a == b && a != 0 {
// buf.push(a as char);
// }
// }
// return buf;
// }
// }
// }
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART1: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    const INPUT_PART2: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT_PART1), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT_PART2), "fgij");
    }
}
