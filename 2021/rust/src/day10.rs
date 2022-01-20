// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(input);
    let soln2 = part2(input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

fn read_score(b: u8) -> usize {
    match b {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut level = Vec::new();
            line.bytes()
                .map(|c| match c {
                    b'[' => {
                        level.push(c);
                        0
                    }
                    b'(' => {
                        level.push(c);
                        0
                    }
                    b'{' => {
                        level.push(c);
                        0
                    }
                    b'<' => {
                        level.push(c);
                        0
                    }
                    b']' => {
                        let open = level.pop().unwrap();
                        if open != b'[' {
                            read_score(c)
                        } else {
                            0
                        }
                    }
                    b'>' => {
                        let open = level.pop().unwrap();
                        if open != b'<' {
                            read_score(c)
                        } else {
                            0
                        }
                    }
                    b'}' => {
                        let open = level.pop().unwrap();
                        if open != b'{' {
                            read_score(c)
                        } else {
                            0
                        }
                    }
                    b')' => {
                        let open = level.pop().unwrap();
                        if open != b'(' {
                            read_score(c)
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut score = input
        .trim()
        .lines()
        .map(|line| {
            let mut level = Vec::new();
            for c in line.bytes() {
                match c {
                    b'[' => level.push(c),
                    b'(' => level.push(c),
                    b'{' => level.push(c),
                    b'<' => level.push(c),
                    b']' => {
                        if *level.last().unwrap() == b'[' {
                            level.pop();
                        } else {
                            level.clear();
                            break;
                        }
                    }
                    b'>' => {
                        if *level.last().unwrap() == b'<' {
                            level.pop();
                        } else {
                            level.clear();
                            break;
                        }
                    }
                    b')' => {
                        if *level.last().unwrap() == b'(' {
                            level.pop();
                        } else {
                            level.clear();
                            break;
                        }
                    }
                    b'}' => {
                        if *level.last().unwrap() == b'{' {
                            level.pop();
                        } else {
                            level.clear();
                            break;
                        }
                    }
                    _ => unreachable!(),
                };
            }
            let score = level.iter().rev().fold(0, |acc, c| {
                let c = match c {
                    b'(' => 1_usize,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                };
                acc * 5 + c
            });
            score
        })
        .collect::<Vec<usize>>();
    score.retain(|v| *v != 0);
    score.sort_unstable();
    score[score.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;
        assert_eq!(part1(input), 26397);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;
        assert_eq!(part2(input), 288957);
    }
}
