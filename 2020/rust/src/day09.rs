// Advent of Code
// Copyright (C) 2020  Isaac Curtis
use std::collections::VecDeque;

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(input, 25);
    let soln2 = part2(input, 25);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(input: &str, preamble: usize) -> usize {
    let input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (preamble..input.len()).find(|&i| {
        !(i-preamble..i).any(|j| { 
            input[j+1..i].iter().any(|y| input[j] + y == input[i])
        })
    }).map(|i| input[i]).unwrap()
}

pub fn part2(input: &str, preamble: usize) -> usize {
    let input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let (idx, invalid) = (preamble..input.len()).find(|&i| {
        !(i-preamble..i).any(|j| { 
            input[j+1..i].iter().any(|y| input[j] + y == input[i])
        })
    }).map(|i| (i, input[i])).unwrap();
    let input = input[..idx].to_vec();
    let mut sum_vec = VecDeque::new();
    sum_vec.push_back(input[0]);
    let mut i = 0;
    let mut sum: usize = sum_vec.iter().sum();
    while sum != invalid {
        sum_vec.push_back(input[i]);
        sum = sum_vec.iter().sum();
        while sum > invalid {
            sum_vec.pop_front();
            sum = sum_vec.iter().sum();
        }
        i += 1;
    }
    sum_vec.make_contiguous().sort_unstable();
    sum_vec.back().unwrap() + sum_vec.front().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#;
        assert_eq!(part1(&input, 5), 127);
    }


    #[test]
    fn test_part2_0() {
        let input = r#"
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#;
        assert_eq!(part2(&input, 5), 62);
    }
}
