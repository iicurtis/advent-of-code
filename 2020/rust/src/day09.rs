// Advent of Code
// Copyright (C) 2020  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let (soln1, soln2) = solve_both(input, 25);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn solve_both(input: &str, preamble: usize) -> (usize, usize) {
    let input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let (idx, part1) = (preamble..input.len()).find(|&i| {
        !(i-preamble..i).any(|j| { 
            input[j+1..i].iter().any(|y| input[j] + y == input[i])
        })
    }).map(|i| (i, input[i])).unwrap();
    let input = input[..idx].to_vec();
    let mut sum: usize = input[0];
    let mut top = 0;
    let mut bot = 0;
    while sum != part1 {
        if sum < part1 {
            top += 1;
            sum += input[top];
        } else {
            sum -= input[bot];
            bot += 1;
        }
    }
    let input = input[bot..top].to_vec();
    let part2 = input.iter().max().unwrap() + input.iter().min().unwrap();
    (part1, part2)
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
        assert_eq!(solve_both(&input, 5), (127, 62));
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
        assert_eq!(solve_both(&input, 5), (127, 62));
    }
}
