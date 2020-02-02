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

use std::fmt::{self, Display};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Point {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {} Y: {}", self.x_pos, self.y_pos)
    }
}

fn to_time(input: &mut Vec<Point>, time: i32) -> (i32, i32) {
    let mut left_bound = i32::max_value();
    let mut upper_bound = i32::max_value();
    for p in input.iter_mut() {
        p.x_pos += p.x_vel * time;
        p.y_pos += p.y_vel * time;
        left_bound = std::cmp::min(left_bound, p.x_pos);
        upper_bound = std::cmp::min(upper_bound, p.y_pos);
    }
    (left_bound, upper_bound)
}

fn find_final_time(input: &[Point]) -> usize {
    // Change center of mass to be fixed at origin
    // Galilean transform: lazy mode
    let mut input = input.to_owned();
    let mut mean_x: f32 = input.iter().map(|p| p.x_pos as f32).sum();
    let mut mean_y: f32 = input.iter().map(|p| p.y_pos as f32).sum();
    let mut mean_xv: f32 = input.iter().map(|p| p.x_vel as f32).sum();
    let mut mean_yv: f32 = input.iter().map(|p| p.y_vel as f32).sum();
    mean_x /= input.len() as f32;
    mean_y /= input.len() as f32;
    mean_xv /= input.len() as f32;
    mean_yv /= input.len() as f32;
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    // Find time where distance to origin is minimized
    for p in input.iter_mut() {
        let xp = p.x_pos as f32 - mean_x;
        let yp = p.y_pos as f32 - mean_y;
        let xv = p.x_vel as f32 - mean_xv;
        let yv = p.y_vel as f32 - mean_yv;
        numerator += xp * xv + yp * yv;
        denominator += xv * xv + yv * yv;
    }
    (-(numerator / denominator)).round() as usize
}

mod parsers {
    use super::Point;
    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map_res, opt, recognize},
        sequence::{pair, separated_pair},
        IResult,
    };
    use std::str::FromStr;

    fn number(s: &str) -> IResult<&str, i32> {
        let (s, _) = opt(tag(" "))(s)?;
        map_res(recognize(pair(opt(tag("-")), digit1)), i32::from_str)(s)
    }

    fn point(s: &str) -> IResult<&str, Point> {
        let (s, _) = tag("position=<")(s)?;
        let (s, (x_pos, y_pos)) = separated_pair(number, tag(", "), number)(s)?;
        let (s, _) = tag("> velocity=<")(s)?;
        let (s, (x_vel, y_vel)) = separated_pair(number, tag(", "), number)(s)?;
        let (s, _) = tag(">")(s)?;
        Ok((
            s,
            Point {
                x_pos,
                y_pos,
                x_vel,
                y_vel,
            },
        ))
    }

    #[derive(Debug, Clone)]
    pub struct ParseError;

    impl std::str::FromStr for Point {
        type Err = ParseError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match point(input) {
                Ok(("", instruction)) => Ok(instruction),
                _ => Err(ParseError),
            }
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Point>, _>>()
        .unwrap()
}

pub fn part1(input: &[Point]) -> String {
    let glyphs: hashbrown::HashMap<u64, char> = [
        (0x861_861f_e186_148c, 'A'),
        (0x7e1_8618_5f86_185f, 'B'),
        (0x7a1_0410_4104_185e, 'C'),
        (0xfc1_0410_5f04_107f, 'E'),
        (0x041_0410_5f04_107f, 'F'),
        (0xbb1_861e_4104_185e, 'G'),
        (0x861_8618_7f86_1861, 'H'),
        (0x391_4504_1041_0438, 'J'),
        (0x851_2450_c314_9461, 'K'),
        (0xfc1_0410_4104_1041, 'L'),
        (0x871_c69a_6596_38e1, 'N'),
        (0x041_0410_5f86_185f, 'P'),
        (0x861_4512_5f86_185f, 'R'),
        (0x861_4923_0c49_2861, 'X'),
        (0xfc1_0421_0842_083f, 'Z'),
    ]
    .iter()
    .cloned()
    .collect();
    let mut input = input.to_owned();
    let time = find_final_time(&input);
    let (left_bound, upper_bound) = to_time(&mut input, time as i32);
    let mut text: [u64; 8] = [0; 8];
    for p in input.iter_mut() {
        p.x_pos -= left_bound;
        p.y_pos -= upper_bound;
        text[(p.x_pos as usize / 8) % 8] |= 0x1 << (6 * p.y_pos + p.x_pos % 8);
    }
    let mut soln1 = "".to_string();
    for n in text.iter() {
        soln1.push(glyphs[n]);
    }
    soln1
}

pub fn part2(input: &[Point]) -> usize {
    let input = input.to_owned();
    find_final_time(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
"#;
        assert_eq!(part2(&parse_input(input)), 3);
    }
}
