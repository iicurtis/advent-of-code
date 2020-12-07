// Advent of Code
// Copyright (C) 2020  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|group| {
            let mut answers = [false; 26];
            for line in group.lines() {
                line.bytes().for_each(|c| answers[(c - b'a') as usize] = true);
            }
            answers.iter().map(|x| *x as usize).sum::<usize>()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|group| {
            let mut group_all_ans = [true; 26];
            for answers in group.lines() {
                let mut ans = [false; 26];
                answers
                    .bytes()
                    .for_each(|c| ans[(c - b'a') as usize] = true);
                ans.iter()
                    .zip(&mut group_all_ans)
                    .for_each(|(a, b)| *b &= *a);
            }
            group_all_ans.iter().map(|x| *x as usize).sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
"#;
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
"#;
        assert_eq!(part2(&input), 6);
    }
}
