// Advent of Code
// Copyright (C) 2020  Isaac Curtis

use std::collections::{HashMap, HashSet, VecDeque};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse_input(input: &str) -> Vec<Rule> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line_parts = line.trim().splitn(2, " bags contain ");
            let bag = line_parts.next().unwrap().to_string();
            let bags_inside = line_parts
                .next()
                .unwrap()
                .trim_matches('.')
                .split(',')
                .map(|bag_inside| {
                    // If only I could do it with fold_first:
                    //bag.split_whitespace().skip(1).fold_first(|a, b| a + " " + b)
                    let bag = bag_inside.split_whitespace().collect::<Vec<_>>();
                    let count = bag[0].parse().unwrap_or(0);
                    // since there are only two adjectives
                    (count, bag[1].to_string() + " " + bag[2])
                })
                .collect::<Vec<_>>();
            Rule { bag, bags_inside }
        })
        .collect()
}

pub fn part1(input: &[Rule]) -> usize {
    let mut ruleset: HashMap<String, Vec<&str>> = HashMap::new();
    for rule in input {
        for (_, bag_inside) in &rule.bags_inside {
            ruleset
                .entry(bag_inside.to_string())
                .or_default()
                .push(&rule.bag);
        }
    }
    let mut queue: VecDeque<_> = ruleset["shiny gold"].iter().cloned().collect();
    let mut all_bags = HashSet::new();
    while let Some(front) = queue.pop_front() {
        if all_bags.insert(front.to_string()) {
            queue.extend(
                ruleset
                    .entry(front.to_string())
                    .or_default()
                    .iter()
                    .cloned(),
            );
        }
    }
    all_bags.len()
}

pub fn part2(input: &[Rule]) -> usize {
    let mut ruleset: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for rule in input {
        for (count, bag_inside) in &rule.bags_inside {
            ruleset
                .entry(rule.bag.to_string())
                .or_default()
                .push((*count, bag_inside.to_string()));
        }
    }
    let mut queue: VecDeque<_> = ruleset["shiny gold"].iter().cloned().collect();
    let mut total_count = 0;
    while let Some((count, front)) = queue.pop_front() {
        for (count_inside, bag_inside) in ruleset.entry(front).or_default() {
            queue.push_back((count * *count_inside, bag_inside.to_string()));
        }
        total_count += count;
    }
    total_count
}

#[derive(Debug)]
pub struct Rule {
    bag: String,
    bags_inside: Vec<(usize, String)>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;
        assert_eq!(part1(&parse_input(&input)), 4);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;
        assert_eq!(part2(&parse_input(&input)), 32);
    }

    #[test]
    fn test_part2_1() {
        let input = r#"
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#;
        assert_eq!(part2(&parse_input(&input)), 126);
    }
}
