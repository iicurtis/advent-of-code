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

fn to_time(input: &mut Vec<Point>, time: i32) {
    for p in input.iter_mut() {
        p.x_pos += p.x_vel * time;
        p.y_pos += p.y_vel * time;
    }
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
    return (-(numerator / denominator)).round() as usize;
}

fn print_points(points: &[Point]) -> String {
    let minx = points.iter().map(|p| p.x_pos).min().unwrap();
    let maxx = points.iter().map(|p| p.x_pos).max().unwrap();
    let miny = points.iter().map(|p| p.y_pos).min().unwrap();
    let maxy = points.iter().map(|p| p.y_pos).max().unwrap();
    let mut sky = vec![vec![' '; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
    for point in points {
        sky[(point.y_pos - miny) as usize][(point.x_pos - minx) as usize] = '▓';
    }
    format!(
        "\n\n{}\n",
        sky.into_iter()
            .map(|line| line.into_iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

mod parsers {
    use super::Point;
    use nom::*;
    use std::str::FromStr;

    named!(number(&str) -> i32,
    map_res!(recognize!(pair!(opt!(tag!("-")), digit)), i32::from_str));

    named!(point(&str) -> Point,
    do_parse!(
        tag!("position=<") >>
        opt!(tag!(" ")) >>
        x_pos: number >>
        tag!(", ") >>
        opt!(tag!(" ")) >>
        y_pos: number >>
        tag!("> velocity=<") >>
        opt!(tag!(" ")) >>
        x_vel: number >>
        tag!(", ") >>
        opt!(tag!(" ")) >>
        y_vel: number >>
        tag!(">") >>
        (Point { x_pos, y_pos, x_vel, y_vel })
        )
    );

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
    let instructions = input
        .trim()
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Point>, _>>()
        .unwrap();
    return instructions;
}

pub fn part1(input: &[Point]) -> usize {
    let mut input = input.to_owned();
    let time = find_final_time(&input);
    to_time(&mut input, time as i32);
    let ans = print_points(&input);
    println!("{}", ans);
    return time;
}

pub fn part2(input: &[Point]) -> usize {
    let input = input.to_owned();
    let time = find_final_time(&input);
    return time;
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
