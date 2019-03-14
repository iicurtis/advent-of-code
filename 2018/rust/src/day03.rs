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

type Error = Box<std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Debug)]
pub struct LandClaim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

pub fn parse_input(input: &str) -> Vec<LandClaim> {
    let mut input_vec: Vec<LandClaim> = Vec::new();
    for line in input.trim().lines() {
        let all_the_things = line
            .split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
            .filter_map(|c| c.parse::<u16>().ok())
            .collect::<Vec<_>>();
        let id = all_the_things[0];
        let xpos = all_the_things[1];
        let ypos = all_the_things[2];
        let xsize = all_the_things[3];
        let ysize = all_the_things[4];
        input_vec.push(LandClaim {
            id: id,
            x: xpos,
            y: ypos,
            width: xsize,
            height: ysize,
        })
    }
    return input_vec;
}

pub fn part1(input: &Vec<LandClaim>) -> usize {
    let mut claims: Vec<u16> = vec![0; 1 << 20];
    let fabric_width = 1024;

    for claim in input.iter() {
        for x in claim.x..claim.x+claim.width {
            for y in claim.y..claim.y + claim.height {
                let idx = y as usize * fabric_width + x as usize;
                claims[idx] = claims[idx] + 1;
            }
        }
    }
    let intersected_claims = claims.iter().filter(|v| **v > 1).count();
    return intersected_claims;
}

pub fn part2(input: &Vec<LandClaim>) -> usize {
    // minimum size is 1000x1000 large; 1024x1024 is close
    let mut claims: Vec<u16> = vec![0; 1 << 20];
    let fabric_width = 1024;

    for claim in input.iter() {
        for x in claim.x..claim.x+claim.width {
            for y in claim.y..claim.y + claim.height {
                let idx = y as usize * fabric_width + x as usize;
                claims[idx] = claims[idx] + 1;
            }
        }
    }

    'outer: for claim in input.iter() {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                if claims[y as usize * fabric_width + x as usize] > 1 {
                    continue 'outer;
                }
            }
        }
        return claim.id as usize;
    }
    panic!("No solution")
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
        assert_eq!(part1(&parse_input(input)), 4);
    }

    #[test]
    fn test_part2() {
        let input = r#"
    #1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2
    "#;
        assert_eq!(part2(&parse_input(input)), 3);
    }

}
