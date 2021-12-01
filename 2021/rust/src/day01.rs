// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(input);
    let soln2 = part2(input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(input: &str) -> usize {
    let input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut increased_count = 0;
    let mut prev_depth = usize::max_value();
    for depth_measurement in input {
        if depth_measurement > prev_depth {
            increased_count += 1;
        }
        prev_depth = depth_measurement;
    }
    increased_count
}

pub fn part2(input: &str) -> usize {
    let input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut increased_count = 0;
    let mut prev_sum = usize::max_value();
    for i in 0..input.len() - 2 {
        let window_sum = input[i] + input[i + 1] + input[i + 2];
        if window_sum > prev_sum {
            increased_count += 1;
        }
        prev_sum = window_sum;
    }
    increased_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
"#;
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
"#;
        assert_eq!(part2(&input), 5);
    }
}
