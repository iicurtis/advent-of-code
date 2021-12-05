// Advent of Code
// Copyright (C) 2020  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(input);
    let soln2 = part2(input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(input: &str) -> usize {
    let mut input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    input.sort_unstable();
    let mut prev_adapter = 0;
    let mut jolt1 = 0;
    let mut jolt3 = 1;
    for adapter in input {
        let difference = adapter - prev_adapter;
        if difference == 1 {
            jolt1 += 1;
        } else if difference == 3 {
            jolt3 += 1;
        }
        prev_adapter = adapter;
    }
    jolt1 * jolt3
}

pub fn part2(input: &str) -> usize {
    let mut input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    input.sort_unstable();
    input.insert(0, 0);
    let mut combinations = vec![0usize; input.last().unwrap() + 4];
    combinations[0] = 1;
    for adapter in input {
        combinations[adapter + 1] += combinations[adapter];
        combinations[adapter + 2] += combinations[adapter];
        combinations[adapter + 3] += combinations[adapter];
    }
    *combinations.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
16
10
15
5
1
11
7
19
6
12
4
"#;
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part1_1() {
        let input = r#"
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"#;
        assert_eq!(part1(&input), 220);
    }

    #[test]
    fn test_part2_1() {
        let input = r#"
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"#;
        assert_eq!(part2(&input), 19208);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
16
10
15
5
1
11
7
19
6
12
4
"#;
        assert_eq!(part2(&input), 8);
    }
}
