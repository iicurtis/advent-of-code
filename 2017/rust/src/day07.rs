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

use itertools::Itertools;
use petgraph::algo::toposort;
use petgraph::Graph;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

struct Node {
    name: String,
    weight: u32,
    total: u32,
}

pub fn solve() {
    // Get line from standard input
    let stdin = io::stdin();
    let re = Regex::new("[[:word:]]+").unwrap();

    let input: Vec<Vec<_>> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            re.find_iter(&line)
                .map(|label| label.as_str().to_string())
                .collect()
        })
        .collect();

    let mut indices = HashMap::new();
    let mut graph = Graph::<Node, ()>::new();

    for line in input.iter() {
        let weight: u32 = line[1].parse().unwrap();
        let node = Node {
            name: line[0].to_string(),
            weight: weight,
            total: weight,
        };
        let idx = graph.add_node(node);
        indices.insert(line[0].to_string(), idx);
    }

    for line in input.iter() {
        for child in &line[2..] {
            graph.add_edge(indices[&line[0]], indices[child], ());
        }
    }

    let sorted = toposort(&graph, None).unwrap();

    println!("[Day 07][Part 1] ANS is: {}", graph[sorted[0]].name);

    for &node in sorted.iter().rev() {
        if !graph.neighbors(node).map(|n| graph[n].total).all_equal() {
            let (min, max) = graph
                .neighbors(node)
                .map(|n| graph[n].total)
                .minmax()
                .into_option()
                .unwrap();

            let (left, right): (Vec<_>, Vec<_>) =
                graph.neighbors(node).partition(|&n| graph[n].total == min);

            let unbalanced = if left.len() == 1 {
                &graph[left[0]]
            } else {
                &graph[right[0]]
            };

            let corrected_weight = unbalanced.weight + min - max;
            println!(
                "[Day 07][Part 2] ANS is: {} - {}",
                unbalanced.name,
                corrected_weight.to_string()
            );
            break;
        }

        graph[node].total += graph.neighbors(node).map(|n| graph[n].total).sum::<u32>();
    }
}
