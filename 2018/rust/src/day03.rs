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

// use packed_simd::u16x32;
use std::str;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let (soln1, soln2) = day03(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct LandClaim {
    y: u16,
    id: u16,
    h: u16,
    idx: u8,
    mask0: u64,
    mask1: u64,
    no_collision: bool,
}

pub fn parse_input(input: &str) -> Vec<LandClaim> {
    let mut input_vec: Vec<LandClaim> = Vec::with_capacity(1500);
    for line in input.trim().lines() {
        let all_the_things = line
            .split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
            .filter_map(|c| c.parse::<u16>().ok())
            .collect::<Vec<_>>();
        let id = all_the_things[0];
        let x = all_the_things[1];
        let y = all_the_things[2];
        let w = all_the_things[3];
        let h = all_the_things[4];
        let idx = (x / 64) as u8;
        let shift = x % 64;
        let mask: u64 = (1 << w) - 1;
        let mask0 = mask << shift;
        let mask1 = if shift != 0 { mask >> (64 - shift) } else { 0 };
        input_vec.push(LandClaim {
            id,
            idx,
            y,
            h,
            mask0,
            mask1,
            no_collision: true,
        })
    }
    // reverse sort
    input_vec.sort_unstable_by(|a, b| b.cmp(a));
    return input_vec;
}

pub fn day03(input: &[LandClaim]) -> (usize, usize) {
    let mut input = input.to_owned();
    let mut part1: usize = 0;
    let mut part2: usize = 0;

    // sweep vertically down fabric, one row at a time,
    // checking for collisions between claims
    let mut row: [u64; 17];
    let mut collide: [u64; 17];
    while !input.is_empty() {
        row = [0; 17];
        collide = [0; 17];

        let y = input.last().unwrap().y;
        // For all claims that exist at row y
        // mark the row bits where occupied
        // mark the collide bit where row was previously occupied
        for claim in input.iter().rev() {
            if claim.y != y {
                break;
            }
            let idx = claim.idx as usize;
            collide[idx] |= row[idx] & claim.mask0;
            row[idx] |= claim.mask0;
            collide[idx + 1] |= row[idx + 1] & claim.mask1;
            row[idx + 1] |= claim.mask1;
        }

        // mark all claims on row that have collision
        // then move to next row
        for cidx in (0..input.len()).rev() {
            if input[cidx].y != y {
                break; // break out if we've left the row
            }
            let idx = input[cidx].idx as usize;
            if input[cidx].no_collision && (((collide[idx] & input[cidx].mask0) != 0)
                    || ((collide[idx + 1] & input[cidx].mask1) != 0))
                {
                    input[cidx].no_collision = false;
                }

            input[cidx].y += 1;
            input[cidx].h -= 1;
            if input[cidx].h == 0 {
                // reached end of the input[cidx];
                // check for part 2 solution
                if input[cidx].no_collision {
                    part2 = input[cidx].id as usize;
                }
                input.swap_remove(cidx);
            }
        }

        for c in collide.iter() {
            part1 += c.count_ones() as usize;
        }
    }

    return (part1, part2);
}

// This is GALAXY BRAIN MODE
// fn part1(input: &Vec<LandClaim>) -> usize {
// let mut claims: Vec<u16> = vec![0; 1 << 20];

// // claim mask: this only adds 1 to the part of the 32 vec we just
// // grabbed that is actually on the claim
// let claim_mask = claim_masks();

// for claim in input.iter() {
// for x in (claim.x..claim.x + claim.width).step_by(32) {
// for y in claim.y..claim.y + claim.height {
// let start_idx = y as usize * 1024 + x as usize;
// let mask = claim_mask[min(31, claim.x + claim.width - x - 1) as usize];
// let values = u16x32::from_slice_unaligned(&claims[start_idx..]);
// (values + mask).write_to_slice_unaligned(&mut claims[start_idx..]);
// }
// }
// }
// let intersected_claims = claims.iter().filter(|v| **v > 1).count();
// return intersected_claims;
// }

// fn part2(input: &Vec<LandClaim>) -> usize {
// let cloth_masks = claim_masks();

// let mut claims = vec![0; 1 << 20];

// // u16x32 idea borrowed from Globidev
// // This method is actually slower than my original method, but with SIMD it comes out nearly 4x
// // faster. Unfortunately theres no good way to bulk replace ids
// for claim in input.iter() {
// for x in (claim.x..claim.x + claim.width).step_by(32) {
// for y in claim.y..claim.y + claim.height {
// let start_idx = y as usize * 1024 + x as usize;
// let mask = cloth_masks[min(31, claim.x + claim.width - x - 1) as usize];
// let values = u16x32::from_slice_unaligned(&claims[start_idx..]);
// (values + mask).write_to_slice_unaligned(&mut claims[start_idx..]);
// }
// }
// }

// 'outer: for claim in input.iter() {
// for x in claim.x..claim.x + claim.width {
// for y in claim.y..claim.y + claim.height {
// if claims[y as usize * 1024 + x as usize] > 1 {
// continue 'outer;
// }
// }
// }
// return claim.id as usize;
// }

// panic!("No solution")
// }

// fn claim_masks() -> Vec<u16x32> {
// (0..32)
// .map(|stop| {
// let mut slice = [0; 32];
// for i in 0..stop + 1 {
// slice[i] = 1
// }
// u16x32::from_slice_unaligned(&slice)
// })
// .collect()
// // [
// //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
// //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
// // ];
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
    #1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2
    "#;
        assert_eq!(day03(&parse_input(input)), (4, 3));
    }
}
