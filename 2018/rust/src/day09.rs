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

#[derive(Clone, Debug, Default)]
struct Player {
    points: usize,
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Points: {}", self.points)
    }
}

fn play_game(players: &mut [Player], marbles: usize) {
    let mut game = Game::new();
    for (player_id, _) in (0..players.len()).cycle().zip(0..marbles) {
        game.turn(&mut players[player_id]);
    }
}

pub fn parse_input(input: &str) -> Box<(usize, usize)> {
    let mut words = input.trim().split(' ');
    Box::new((
        words.next().unwrap().parse().unwrap(),
        words.nth(5).unwrap().parse().unwrap(),
    ))
}

pub fn part1(input: &(usize, usize)) -> usize {
    let mut players = vec![Player::default(); input.0];
    play_game(&mut players, input.1);
    return players.iter().map(|p| p.points).max().unwrap();
}

pub fn part2(input: &(usize, usize)) -> usize {
    let mut players = vec![Player::default(); input.0];
    play_game(&mut players, input.1 * 100);
    return players.iter().map(|p| p.points).max().unwrap();
}

struct Game {
    marbles: Vec<Marble>,
    current: usize,
    turn: usize,
}

struct Marble {
    id: usize,
    prev: usize,
    next: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            marbles: vec![Marble {
                id: 0,
                prev: 0,
                next: 0,
            }],
            current: 0,
            turn: 1,
        }
    }

    fn turn(&mut self, player: &mut Player) {
        let marble_id = self.add_marble();
        if self.turn % 23 != 0 {
            let insert_at = self.clockwise(1);
            self.insert_after(marble_id, insert_at);
            self.current = marble_id;
        } else {
            player.points += self.turn;
            let to_remove = self.counter_clockwise(7);
            player.points += self.marbles[to_remove].id;
            self.remove(to_remove);
            self.current = self.counter_clockwise(6);
        }
        self.turn += 1;
    }

    fn add_marble(&mut self) -> usize {
        let id = self.marbles.len();
        self.marbles.push(Marble::new(self.turn));
        return id;
    }

    fn clockwise(&mut self, mut num: usize) -> usize {
        let mut id = self.current;
        while num > 0 {
            id = self.marbles[id].next;
            num -= 1;
        }
        return id;
    }

    fn counter_clockwise(&mut self, mut num: usize) -> usize {
        let mut id = self.current;
        while num > 0 {
            id = self.marbles[id].prev;
            num -= 1;
        }
        return id;
    }

    fn remove(&mut self, id: usize) {
        let Marble { id: _, prev, next } = self.marbles[id];
        self.marbles[prev].next = next;
        self.marbles[next].prev = prev;
    }

    fn insert_after(&mut self, to_insert: usize, after: usize) {
        let last_next = self.marbles[after].next;
        self.marbles[after].next = to_insert;
        self.marbles[last_next].prev = to_insert;
        self.marbles[to_insert].prev = after;
        self.marbles[to_insert].next = last_next;
    }
}

impl Marble {
    fn new(id: usize) -> Marble {
        Marble {
            id,
            prev: 0,
            next: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
10 players; last marble is worth 1618 points
"#;
        assert_eq!(part1(&parse_input(input)), 8317);
    }

    #[test]
    fn test_part1_1() {
        let input = r#"
13 players; last marble is worth 7999 points
"#;
        assert_eq!(part1(&parse_input(input)), 146373);
    }

    #[test]
    fn test_part1_2() {
        let input = r#"
17 players; last marble is worth 1104 points
"#;
        assert_eq!(part1(&parse_input(input)), 2764);
    }

    #[test]
    fn test_part1_3() {
        let input = r#"
21 players; last marble is worth 6111 points
"#;
        assert_eq!(part1(&parse_input(input)), 54718);
    }

    #[test]
    fn test_part1_4() {
        let input = r#"
30 players; last marble is worth 5807 points
"#;
        assert_eq!(part1(&parse_input(input)), 37305);
    }

}
