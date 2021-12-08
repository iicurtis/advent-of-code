// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn fuel(crabs: &[isize], point: isize) -> isize {
    crabs.iter().map(|x| ((x - point) * (x-point) + (x-point).abs())/2).sum()
}

pub fn part1(input: &[isize]) -> isize {
    // median point should be optimal
    // isn't this basically lsd?
    let mut input = input.clone().to_vec();
    input.sort();
    let median = input[input.len()/2];
    let fuel_used = input.iter().map(|x| (x - median).abs()).sum();
    fuel_used
}

pub fn part2(input: &[isize]) -> isize {
    // median point should be optimal
    let mean = input.iter().sum::<isize>() / input.len() as isize;
    let fuel_used = std::cmp::min(fuel(&input, mean + 1), fuel(&input, mean));
    fuel_used
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
16,1,2,0,4,2,7,1,2,14
"#;
        assert_eq!(part1(&parse(input)), 37);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
16,1,2,0,4,2,7,1,2,14
"#;
        assert_eq!(part2(&parse(input)), 168);
    }

}
