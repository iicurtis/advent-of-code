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

extern crate advent2017_rs;

use advent2017_rs::*;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Advent of Code in Rust 2018")
        .author("Isaac Curtis <iicurtis att outlook doot com>")
        .arg(
            Arg::with_name("day")
                .required(true)
                .help("Day of the advent calendar")
                .validator(|str| {
                    str.parse::<u32>()
                        .or(Err("day must be an integer".to_owned()))
                        .and_then(|v| match v {
                            1...25 => Ok(()),
                            _ => Err("day must be between 1 and 25".to_owned()),
                        })
                }),
        )
        .get_matches();
    match matches.value_of("day").unwrap().parse::<u32>().unwrap() {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        7 => day07::solve(),
        8 => day08::solve(),
        _ => (),
    }
}
