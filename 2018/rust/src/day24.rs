// Advent of Code
// Copyright (C) 2018  Isaac Curtis

type Error = Box<dyn std::error::Error>;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fmt::{self, Display};

pub fn solve(input: &str) -> Result<String, Error> {
    let (immune, infection) = parse(&input);
    let soln1 = part1(&immune, &infection);
    let soln2 = part2(&immune, &infection);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(immune: &[Group], infection: &[Group]) -> u32 {
    match fight(immune, infection, 0) {
        FightResult::WinningArmy(_, units_remain) => units_remain,
        _ => panic!("The fight never ended."),
    }
}

pub fn part2(immune: &Vec<Group>, infection: &Vec<Group>) -> u32 {
    for boost in 1..1 << 10 {
        if let Some(result) = match fight(immune, infection, boost) {
            FightResult::WinningArmy(Army::Immune, units_remain) => Some(units_remain),
            _ => None,
        } {
            return result;
        }
    }
    return 0;
}

fn fight(immune: &[Group], infection: &[Group], boost: u32) -> FightResult {
    let mut immune = immune.to_vec();
    let mut infection = infection.to_vec();

    immune.iter_mut().for_each(|group| group.dmg += boost);

    loop {
        // target selection
        immune.sort_by_key(|group| (Reverse(group.effective_power()), Reverse(group.initiative)));
        infection
            .sort_by_key(|group| (Reverse(group.effective_power()), Reverse(group.initiative)));

        let mut fights = Vec::new();
        let mut immune_target = HashSet::new();

        for (attacking_idx, attacking_group) in immune.iter().enumerate() {
            let available_targets =
                infection
                    .iter()
                    .enumerate()
                    .filter(|(defending_idx, defending_group)| {
                        !immune_target.contains(defending_idx)
                            && attacking_group.effective_damage(defending_group) > 0
                    });
            let target = available_targets.max_by_key(|(_, defending_group)| {
                (
                    attacking_group.effective_damage(defending_group),
                    defending_group.effective_power(),
                    defending_group.initiative,
                )
            });
            if let Some((defending_idx, _)) = target {
                fights.push((
                    attacking_idx,
                    defending_idx,
                    attacking_group.initiative,
                    Army::Immune,
                ));
                immune_target.insert(defending_idx);
            }
        }

        let mut infection_target = HashSet::new();

        for (attacking_idx, attacking_group) in infection.iter().enumerate() {
            let available_targets =
                immune
                    .iter()
                    .enumerate()
                    .filter(|(defending_idx, defending_group)| {
                        !infection_target.contains(defending_idx)
                            && attacking_group.effective_damage(defending_group) > 0
                    });
            let target = available_targets.max_by_key(|(_, defending_group)| {
                (
                    attacking_group.effective_damage(defending_group),
                    defending_group.effective_power(),
                    defending_group.initiative,
                )
            });
            if let Some((defending_idx, _)) = target {
                fights.push((
                    attacking_idx,
                    defending_idx,
                    attacking_group.initiative,
                    Army::Infection,
                ));
                infection_target.insert(defending_idx);
            }
        }

        // attack
        fights.sort_by_key(|&(_, _, initiative, _)| Reverse(initiative));
        let mut total_deaths = 0;

        for (attacking_group_idx, defending_group_idx, _, army) in fights {
            let (attacking_group, defending_group) = match army {
                Army::Immune => (
                    &immune[attacking_group_idx],
                    &mut infection[defending_group_idx],
                ),
                Army::Infection => (
                    &infection[attacking_group_idx],
                    &mut immune[defending_group_idx],
                ),
            };

            let deaths = attacking_group.effective_damage(defending_group) / defending_group.hp;
            total_deaths += deaths;
            defending_group.units = defending_group.units.saturating_sub(deaths);
        }

        if total_deaths == 0 {
            break FightResult::Draw;
        }

        immune.retain(|group| group.units > 0);
        infection.retain(|group| group.units > 0);

        let remaining_immune = immune.iter().map(|group| group.units).sum();
        let remaining_infection = infection.iter().map(|group| group.units).sum();

        if remaining_immune == 0 {
            break FightResult::WinningArmy(Army::Infection, remaining_infection);
        } else if remaining_infection == 0 {
            break FightResult::WinningArmy(Army::Immune, remaining_immune);
        }
    }
}

pub enum Army {
    Infection,
    Immune,
}

pub enum FightResult {
    WinningArmy(Army, u32),
    Draw,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Group {
    initiative: u32,
    units: u32,
    hp: u32,
    weak: Vec<AttackType>,
    immune: Vec<AttackType>,
    dmg: u32,
    atktype: AttackType,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units * self.dmg
    }

    fn effective_damage(&self, other: &Group) -> u32 {
        let is_immune = other.immune.contains(&self.atktype);
        let is_weak = other.weak.contains(&self.atktype);

        let modifier = match (is_immune, is_weak) {
            (true, _) => 0,
            (_, true) => 2,
            _ => 1,
        };

        self.effective_power() * modifier
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Group({} units, {} hp, {:?} immune, {:?} {} atk, {} initiative",
            self.units, self.hp, self.immune, self.atktype, self.dmg, self.initiative
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl From<&str> for AttackType {
    fn from(input: &str) -> Self {
        match input {
            "bludgeoning" => AttackType::Bludgeoning,
            "cold" => AttackType::Cold,
            "fire" => AttackType::Fire,
            "radiation" => AttackType::Radiation,
            "slashing" => AttackType::Slashing,
            unknown => panic!("Unknown attack type: '{}'", unknown),
        }
    }
}

pub fn parse(input: &str) -> (Vec<Group>, Vec<Group>) {
    let mut armies = input.split("\n\n");
    let immune = armies
        .next()
        .unwrap()
        .trim()
        .lines()
        .skip(1)
        .map(|line| parse_group(line))
        .collect::<Vec<Group>>();
    let infection = armies
        .next()
        .unwrap()
        .trim()
        .lines()
        .skip(1)
        .map(|line| parse_group(line))
        .collect::<Vec<Group>>();
    (immune, infection)
}

fn parse_group(input: &str) -> Group {
    let mut s = input.split(" units each with ");
    let units = s.next().unwrap().parse().unwrap();

    let mut s = s.next().unwrap().split(" hit points ");
    let hp = s.next().unwrap().parse().unwrap();

    let mut s = s.next().unwrap().split("with an attack that does ");
    let resistances = s.next().unwrap();

    let mut weak = Vec::new();
    let mut immune = Vec::new();

    if resistances.starts_with("(") {
        let mut resistances = resistances[1..resistances.len() - 2].split("; ");

        while let Some(resistance) = resistances.next() {
            if resistance.starts_with("immune") {
                immune.extend(resistance[10..].split(", ").map(AttackType::from));
            } else {
                weak.extend(resistance[8..].split(", ").map(AttackType::from));
            }
        }
    }

    let mut s = s.next().unwrap().split(" damage at initiative ");
    let mut attack_str = s.next().unwrap().split(' ');

    let dmg = attack_str.next().unwrap().parse().unwrap();
    let atktype = attack_str.next().unwrap().into();

    let initiative = s.next().unwrap().parse().unwrap();

    Group {
        units,
        hp,
        immune,
        weak,
        dmg,
        atktype,
        initiative,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE0: &str = r#"
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
"#;

    #[test]
    fn test_part1_0() {
        let (immune, infection) = parse(EXAMPLE0);
        assert_eq!(part1(&immune, &infection), 5216);
    }
}
