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

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

#[derive(Clone, Hash, Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn to_node(input: &mut impl Iterator<Item = usize>) -> Node {
    let (num_children, num_metadata) = (input.next().unwrap(), input.next().unwrap());
    Node {
        children: (0..num_children).map(|_| to_node(input)).collect(),
        metadata: (0..num_metadata).map(|_| input.next().unwrap()).collect(),
    }
}

pub fn parse_input(input: &str) -> Box<Node> {
    Box::new(to_node(
        &mut input
            .trim()
            .split(' ')
            .map(|num| num.parse::<usize>().unwrap()),
    ))
}

pub fn part1(input: &Node) -> usize {
    // want: sum of all metadata entries
    let mut metadata = input.metadata.iter().sum();
    for n in input.children.iter() {
        metadata += part1(n);
    }
    return metadata;
}

pub fn part2(input: &Node) -> usize {
    // want: sum of all metadata entries
    let mut value = 0;
    if input.children.is_empty() {
        value += input.metadata.iter().sum::<usize>();
    } else {
        for n in input.metadata.iter() {
            if *n <= input.children.len() && *n > 0 {
                value += part2(&input.children[n - 1])
            }
        }
    }
    return value;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
"#;
        assert_eq!(part1(&parse_input(input)), 138);
    }

    #[test]
    fn test_part2() {
        let input = r#"
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
"#;
        assert_eq!(part2(&parse_input(input)), 66);
    }

}
