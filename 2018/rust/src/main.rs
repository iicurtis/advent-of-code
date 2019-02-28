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

extern crate advent2018_rs;

use advent2018_rs::*;
use clap::{App, Arg};
use std::io::{self, Write};

type Error = Box<std::error::Error>;

const SOLUTIONS: &[fn(&str) -> Result<String, Error>] = &[
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day13::solve,
    day14::solve,
    day15::solve,
    day16::solve,
    day17::solve,
    day18::solve,
];

pub fn run(day_s: &str) -> Result<(), Error> {
    use std::fs;
    let day = day_s.parse::<usize>()?;
    if day == 0 {
        for (d, soln) in SOLUTIONS.iter().enumerate() {
            println!("Running day: {}", d + 1);
            let inputfile: String = format!("../inputs/day{:02}.txt", d + 1);
            let input = fs::read_to_string(inputfile).expect("Couldn't find file");
            let result = soln(&input).map_err(|e| e.to_string())?;
            writeln!(io::stdout(), "{}", result)?;
        }
    } else {
        println!("Running day: {}", day);
        let inputfile: String = format!("../inputs/day{:02}.txt", day);
        let input = fs::read_to_string(inputfile).expect("Couldn't find file");
        let solution = SOLUTIONS.get(day - 1).ok_or("Day number out of range")?;
        writeln!(
            io::stdout(),
            "{}",
            (solution)(&input).map_err(|e| e.to_string())?
        )?;
    }
    Ok(())
}

fn main() {
    let matches = App::new("Advent of Code in Rust 2018")
        .author("Isaac Curtis <iicurtis att outlook doot com>")
        .arg(
            Arg::with_name("day")
                .short("d")
                .default_value("0")
                .help("Day of the advent calendar")
                .value_name("DAY")
                .takes_value(true),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap_or_default();
    // DETECT stdin
    // LOAD inputX.txt
    match run(day) {
        Ok(()) => (),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
