// Advent of Code
// Copyright (C) 2018  Isaac Curtis

use hashbrown::HashSet;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fmt::{self, Display};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EntityClass {
    Nothing,
    Wall,
    Elf,
    Goblin,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Entity {
    class: EntityClass,
    attack: i32,
    hp: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct World {
    world: Vec<Entity>,
    xsize: usize,
    ysize: usize,
    goblins: usize,
    elves: usize,
}

#[derive(PartialEq)]
enum RoundResult {
    Complete,
    Incomplete,
}

impl Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.ysize {
            let mut entities = Vec::new();
            for x in 0..self.xsize {
                let pos = self.world[x + y * self.xsize];
                if pos.class == EntityClass::Elf || pos.class == EntityClass::Goblin {
                    entities.push(pos);
                }
                write!(f, "{}", self.world[x + y * self.xsize])?;
            }
            for e in entities.iter() {
                write!(f, "{}({})", e, e.hp)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl World {
    fn get_units_hp(&mut self) -> i32 {
        let mut health = 0;
        for p in 0..(self.xsize * self.ysize) {
            let unit = self.world[p];
            if unit.class == EntityClass::Goblin || unit.class == EntityClass::Elf {
                health += unit.hp;
            }
        }
        health
    }

    fn convert_elf_ap(&mut self, ap: usize) {
        for p in self.world.iter_mut() {
            if p.class == EntityClass::Elf {
                p.attack = ap as i32;
            }
        }
    }

    fn target_in_range(&mut self, pos: usize) -> Option<usize> {
        let enemy_class = self.world[pos].enemy();
        self.get_adjacent(pos)
            .into_iter()
            .filter(|&adjacent| self.world[adjacent].class == enemy_class)
            .min_by_key(|p| self.world[*p].hp)
    }

    fn attack(&mut self, unit: usize, target: usize) {
        self.world[target].hp -= self.world[unit].attack;
        if self.world[target].hp <= 0 {
            match self.world[target].class {
                EntityClass::Elf => self.elves -= 1,
                EntityClass::Goblin => self.goblins -= 1,
                _ => (),
            };
            self.world[target] = Entity::nothing();
        }
    }

    fn get_adjacent(&self, p: usize) -> Vec<usize> {
        let mut valid = Vec::new();
        for delta in [-(self.xsize as i32), -1, 1, self.xsize as i32].iter() {
            valid.push((p as i32 + delta) as usize);
        }
        valid
    }

    fn move_to(&mut self, unit: usize, target: usize) {
        if self.world[target].class != EntityClass::Nothing {
            panic!(
                "WE CAN'T MOVE HERE: {:?} cannot move over {:?}",
                self.world[unit].class, self.world[target].class
            );
        } else {
            let moved_unit = std::mem::replace(&mut self.world[unit], Entity::nothing());
            self.world[target] = moved_unit;
        }
    }

    fn step(&mut self) -> RoundResult {
        let mut processed = HashSet::new();
        for u in 0..self.world.len() {
            if processed.contains(&u) {
                continue;
            }
            match self.world[u].class {
                EntityClass::Elf => (),
                EntityClass::Goblin => (),
                _ => continue,
            };

            // Check if in range already
            if let Some(target) = self.target_in_range(u) {
                self.attack(u, target);
            } else {
                // Else search for best place to move to
                if let Some(valid_pos) = self.move_toward_enemy(u) {
                    self.move_to(u, valid_pos);
                    processed.insert(valid_pos);
                    if let Some(target) = self.target_in_range(valid_pos) {
                        self.attack(valid_pos, target);
                    }
                } else if self.elves == 0 || self.goblins == 0 {
                    return RoundResult::Incomplete;
                }
            }
        }
        RoundResult::Complete
    }

    fn move_toward_enemy(&self, unit: usize) -> Option<usize> {
        struct Node {
            position: usize,
            step_pos: usize,
            distance: usize,
        }

        let mut distance_max = self.xsize + self.ysize;

        let mut solutions = Vec::with_capacity(4);
        let mut open_set = VecDeque::with_capacity(self.world.len());
        let mut visited = HashSet::with_capacity(self.world.len());

        let enemy_class = self.world[unit].enemy();

        visited.insert(unit);

        for position in self.get_adjacent(unit) {
            if self.world[position].class == EntityClass::Nothing {
                visited.insert(position);
                open_set.push_back(Node {
                    position,
                    step_pos: position,
                    distance: 0,
                })
            }
        }

        while let Some(Node {
            position,
            step_pos,
            distance,
        }) = open_set.pop_front()
        {
            if distance > distance_max {
                break;
            }

            for adjacent_pos in self.get_adjacent(position) {
                if visited.contains(&adjacent_pos) {
                    continue;
                }

                match self.world[adjacent_pos].class {
                    v if v == enemy_class => {
                        distance_max = distance;
                        solutions.push((position, step_pos))
                    }
                    EntityClass::Nothing => {
                        visited.insert(adjacent_pos);
                        open_set.push_back(Node {
                            position: adjacent_pos,
                            step_pos,
                            distance: distance + 1,
                        })
                    }
                    _ => (),
                }
            }
        }
        solutions
            .into_iter()
            .min()
            .map(|(_target_pos, start_pos)| start_pos)
    }
}

impl Entity {
    fn goblin() -> Self {
        Entity {
            class: EntityClass::Goblin,
            attack: 3,
            hp: 200,
        }
    }

    fn elf() -> Self {
        Entity {
            class: EntityClass::Elf,
            attack: 3,
            hp: 200,
        }
    }

    fn wall() -> Self {
        Entity {
            class: EntityClass::Wall,
            attack: -1,
            hp: 200_000,
        }
    }

    fn nothing() -> Self {
        Entity {
            class: EntityClass::Nothing,
            attack: -1,
            hp: 200_000,
        }
    }

    fn enemy(&self) -> EntityClass {
        match self.class {
            EntityClass::Elf => EntityClass::Goblin,
            EntityClass::Goblin => EntityClass::Elf,
            _ => EntityClass::Nothing,
        }
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.class {
            EntityClass::Nothing => write!(f, "."),
            EntityClass::Wall => write!(f, "#"),
            EntityClass::Elf => write!(f, "E"),
            EntityClass::Goblin => write!(f, "G"),
        }
    }
}

pub fn parse_input(input: &str) -> Box<World> {
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;
    let mut grid = vec![Entity::nothing(); WIDTH * HEIGHT];
    let mut elfcount = 0;
    let mut gobcount = 0;
    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + y * WIDTH] = match c {
                '#' => Entity::wall(),
                'E' => {
                    elfcount += 1;
                    Entity::elf()
                }
                'G' => {
                    gobcount += 1;
                    Entity::goblin()
                }
                _ => Entity::nothing(),
            };
        }
    }

    Box::new(World {
        world: grid,
        xsize: WIDTH,
        ysize: HEIGHT,
        goblins: gobcount,
        elves: elfcount,
    })
}

pub fn part1(input: &World) -> usize {
    let mut world = input.clone();
    let mut rounds = 0;
    'game: loop {
        if world.step() == RoundResult::Incomplete {
            break 'game;
        }
        rounds += 1;
    }
    let hp_remaining = world.get_units_hp();
    rounds * hp_remaining as usize
}

pub fn part2(input: &World) -> usize {
    let initial_elves_count = input.elves;

    let starting_ap = 4_usize;
    let ending_ap = 200;

    (starting_ap..=ending_ap)
        .into_par_iter()
        .filter_map(|ap| {
            let mut world = input.clone();
            world.convert_elf_ap(ap);

            let end_turn = (0..)
                .skip_while(|_| {
                    let is_complete = world.step() == RoundResult::Complete;
                    let had_elf_casualties = world.elves < initial_elves_count;
                    is_complete && !had_elf_casualties
                })
                .next()
                .unwrap();

            if world.elves == initial_elves_count {
                Some(end_turn * world.get_units_hp())
            } else {
                None
            }
        })
        .find_first(|_| true)
        .unwrap() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
"#;
        assert_eq!(part1(&parse_input(input)), 27730);
    }

    #[test]
    fn test_part1_1() {
        let input = r#"
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
"#;
        assert_eq!(part1(&parse_input(input)), 36334);
    }

    #[test]
    fn test_part1_2() {
        let input = r#"
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
"#;
        assert_eq!(part1(&parse_input(input)), 39514);
    }

    #[test]
    fn test_part1_3() {
        let input = r#"
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
"#;
        assert_eq!(part1(&parse_input(input)), 27755);
    }

    #[test]
    fn test_part1_4() {
        let input = r#"
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
"#;
        assert_eq!(part1(&parse_input(input)), 28944);
    }
}
