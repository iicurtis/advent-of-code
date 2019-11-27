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

use hashbrown::HashMap;
use std::fmt::{self, Display};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Action {
    ShiftStart(usize),
    Asleep,
    Wake,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Event {
    date: Date,
    action: Action,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Date {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{} {}:{}",
            self.year, self.month, self.day, self.hour, self.min
        )
    }
}

mod parsers {
    use super::{Action, Date, Event};
    use nom::*;
    use std::str::FromStr;

    named!(date(&str) -> Date,
    do_parse!(
        tag!("[") >>
        year: map_res!(digit, usize::from_str) >>
        tag!("-") >>
        month: map_res!(digit, usize::from_str) >>
        tag!("-") >>
        day: map_res!(digit, usize::from_str) >>
        tag!(" ") >>
        hour: map_res!(digit, usize::from_str) >>
        tag!(":") >>
        min: map_res!(digit, usize::from_str) >>
        tag!("]") >>
        (Date { year, month, day, hour, min })
        )
    );

    named!(guard_id(&str) -> usize,
    do_parse!(
        tag!("#") >>
        id: map_res!(digit, usize::from_str) >>
        (id)
        )
    );

    named!(event(&str) -> Event,
    do_parse!(
        date: date >>
        tag!(" ") >>
        action: alt!(
            value!(Action::Asleep, tag!("falls asleep")) |
            value!(Action::Wake, tag!("wakes up")) |
            map!(
                delimited!(tag!("Guard "), guard_id, tag!(" begins shift")),
                Action::ShiftStart
                )
            ) >>
        (Event { date, action })
        )
    );

    #[derive(Debug, Clone)]
    pub struct ParseError;

    impl std::str::FromStr for Event {
        type Err = ParseError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match event(input) {
                Ok(("", event)) => Ok(event),
                _ => Err(ParseError),
            }
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Event> {
    let mut events = input
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<Event>, _>>()
        .unwrap();
    events.sort_by_key(|event| event.date);
    return events;
}

#[derive(Clone, Copy)]
struct Guard {
    slept: usize,
    days: [usize; 60],
}

type SleepMin = HashMap<usize, Guard>;

fn get_sleepmin(input: &[Event]) -> SleepMin {
    let mut guards = SleepMin::new();
    let mut sleepstart = 0;
    let mut guard = 0;
    for event in input {
        match event.action {
            Action::ShiftStart(id) => {
                guard = id;
            }
            Action::Asleep => {
                sleepstart = event.date.min;
            }
            Action::Wake => {
                let v = guards.entry(guard).or_insert(Guard {
                    slept: 0,
                    days: [0; 60],
                });
                for n in &mut v.days[sleepstart..event.date.min] {
                    *n += 1;
                }
                v.slept += event.date.min - sleepstart;
            }
        };
    }
    return guards;
}

pub fn part1(input: &[Event]) -> usize {
    let guards_sleepmin = get_sleepmin(input);
    let (guard_id, sleep_sched) = guards_sleepmin
        .into_iter()
        .max_by_key(|(_, c)| c.slept)
        .expect("Guard list empty");
    let mostmin = sleep_sched
        .days
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .map(|(i, _)| i)
        .expect("No max found");
    return mostmin * guard_id;
}

pub fn part2(input: &[Event]) -> usize {
    let guards_sleepmin = get_sleepmin(input);

    let (freqsleep, freqmin) = guards_sleepmin
        .clone()
        .into_iter()
        .map(|(id, guard)| {
            (
                id,
                guard
                    .days
                    .iter()
                    .cloned()
                    .enumerate()
                    .max_by_key(|&(_, v)| v) // max by value of minutes asleep in day
                    .map(|(i, _)| i) // but we only care what  minute it was (the enumeration)
                    .expect("Minute list has no max"),
            )
        })
        .max_by_key(|&(_, c)| c)
        .expect("We couldn't get the freq");

    return freqsleep * freqmin;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;
        assert_eq!(part1(&parse_input(input)), 240);
    }

    #[test]
    fn test_part2() {
        let input = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#;
        assert_eq!(part2(&parse_input(input)), 4455);
    }
}
