// Advent of Code Solutions
// Copyright (C) 2022  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(input);
    let soln2 = part2(input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            line.splitn(4, [',', '-'])
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|splits| {
            splits[0] <= splits[2] && splits[1] >= splits[3]
                || splits[0] >= splits[2] && splits[1] <= splits[3]
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            line.splitn(4, [',', '-'])
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|splits| splits[0] <= splits[3] && splits[1] >= splits[2])
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
        assert_eq!(part2(input), 4);
    }
}
