// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

// most other answers rely on shifting a single array left and updating the end the beginning.
// This is 8 lookups (but this array is so small it should be cached)
// Mine has just two lookups but 3 modulus calculations
pub fn part1(input: &[u8]) -> usize {
    let state = input.to_vec();
    let mut offsets = vec![0; 8];
    for f in state {
        offsets[f as usize + 1] += 1; // add 1 because we have 0-index before reset
    }
    let mut next_offsets = offsets.clone();
    for day in 1..=80 {
        next_offsets[(day + 2) % 7] += offsets[(day % 7)];
        offsets[(day + 3) % 7] = next_offsets[(day + 3) % 7]; // update skipping one cycle
    }
    next_offsets.iter().sum()
}

pub fn part2(input: &[u8]) -> usize {
    let state = input.to_vec();
    let mut offsets = vec![0; 8];
    for f in state {
        offsets[f as usize + 1] += 1;
    }
    let mut next_offsets = offsets.clone();
    for day in 1..=256 {
        next_offsets[(day + 2) % 7] += offsets[(day % 7)];
        offsets[(day + 3) % 7] = next_offsets[(day + 3) % 7]; // update skipping one value
    }
    next_offsets.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
3,4,3,1,2
"#;
        assert_eq!(part1(&parse(input)), 5934);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
3,4,3,1,2
"#;
        assert_eq!(part2(&parse(input)), 26984457539);
    }
}
