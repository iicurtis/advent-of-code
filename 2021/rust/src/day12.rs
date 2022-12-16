// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;
use std::collections::HashMap;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(input);
    let soln2 = part2(input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

fn cave_paths<'a>(
    caves: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    connections: &mut Vec<&'a str>,
    mut seen_twice: bool,
) -> usize {
    if start == "end" {
        return 1;
    }
    if connections.contains(&start) && start.chars().all(|c| c.is_lowercase()) {
        if seen_twice || start == "start" {
            return 0;
        }
        seen_twice = true;
    }
    connections.push(start);
    let ans = caves[start]
        .iter()
        .map(|i| cave_paths(caves, i, connections, seen_twice))
        .sum();
    connections.pop();
    ans
}

pub fn part1(input: &str) -> usize {
    let mut caves = std::collections::HashMap::new();
    for line in input.trim().lines() {
        let (a, b) = line.split_once('-').unwrap();
        caves.entry(a).or_insert(Vec::new()).push(b);
        caves.entry(b).or_insert(Vec::new()).push(a);
    }
    cave_paths(&caves, "start", &mut Vec::new(), true)
}

pub fn part2(input: &str) -> usize {
    let mut caves = std::collections::HashMap::new();
    for line in input.trim().lines() {
        let (a, b) = line.split_once('-').unwrap();
        caves.entry(a).or_insert(Vec::new()).push(b);
        caves.entry(b).or_insert(Vec::new()).push(a);
    }
    cave_paths(&caves, "start", &mut Vec::new(), false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;
        assert_eq!(part1(input), 10);
    }

    #[test]
    fn test_part1_1() {
        let input = r#"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;
        assert_eq!(part1(input), 226);
    }

    #[test]
    fn test_part1_2() {
        let input = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;
        assert_eq!(part1(input), 19);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;
        assert_eq!(part2(input), 36);
    }

    #[test]
    fn test_part2_1() {
        let input = r#"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;
        assert_eq!(part2(input), 3509);
    }

    #[test]
    fn test_part2_2() {
        let input = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;
        assert_eq!(part2(input), 103);
    }
}
