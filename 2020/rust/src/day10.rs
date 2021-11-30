// Advent of Code
// Copyright (C) 2020  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(input);
    let soln2 = 0;
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
        println!("diff: {} a: {} a-1: {}", difference, adapter, prev_adapter);
        if difference == 1 {
            jolt1 += 1;
        } else if difference == 3 {
            jolt3 += 1;
        }
        prev_adapter = adapter;
    }
    let part1 = jolt1 * jolt3;
    part1
}

pub fn part2(input: &str) -> usize {
    let mut input: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    input.sort_unstable();

    // calculate n+2 - n difference. If 2 or 3:
    // append n+1 to list
    // calculate permutations of list
    // edge case:
    // 11 14 15 16 17 19
    // if n+2 distance is 2
    //  calculate n+3 - n+1 distance. If 2 or 3
    //  calculate n+3 - n distance. If 3
    //      add n+1 and n+2 to list
    let mut prev_adapter = 0;
    let mut jolt1 = 0;
    let mut jolt3 = 1;
    for adapter in input {
        let difference = adapter - prev_adapter;
        println!("diff: {} a: {} a-1: {}", difference, adapter, prev_adapter);
        if difference == 1 {
            jolt1 += 1;
        } else if difference == 3 {
            jolt3 += 1;
        }
        prev_adapter = adapter;
    }
    let part1 = jolt1 * jolt3;
    part1
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
        assert_eq!(part1(&input), 8);
    }



    
}
