// Advent of Code
// Copyright (C) 2021  Isaac Curtis

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

extern crate advent2021;

use advent2021::*;
use clap::{App, Arg};
use std::io::{self, Write};

type Error = Box<dyn std::error::Error>;
type Soln = fn(&str) -> Result<String, Error>;

const SOLUTIONS: &[Soln] = &[day01::solve, day02::solve, day03::solve, day04::solve];

fn runday(d: usize, soln: Soln) -> Result<(), Error> {
    use std::fs;
    use std::time::Instant;

    println!("Running day: {}", d);
    let inputfile: String = format!("../inputs/day{:02}.txt", d);
    let input = fs::read_to_string(inputfile).expect("Couldn't find file");
    let before = Instant::now();
    let result = soln(&input).map_err(|e| e.to_string())?;
    writeln!(io::stdout(), "{}", result)?;
    println!("[{:4}μs]", before.elapsed().as_micros());
    Ok(())
}

pub fn run(day_s: &str) -> Result<(), Error> {
    use std::time::Instant;
    let day = day_s.parse::<usize>()?;
    if day == 0 {
        let now = Instant::now();
        for (d, soln) in SOLUTIONS.iter().enumerate() {
            runday(d + 1, *soln)?;
        }
        println!("Total time: {}ms", now.elapsed().as_millis());
    } else {
        let solution = SOLUTIONS.get(day - 1).ok_or("Day number out of range")?;
        runday(day, *solution)?;
    }
    Ok(())
}

fn main() {
    let matches = App::new("Advent of Code in Rust 2021")
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
