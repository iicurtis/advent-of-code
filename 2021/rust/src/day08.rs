// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub struct SDisplay {
    input: Vec<String>,
    output: Vec<String>,
}

pub fn parse(input: &str) -> Vec<SDisplay> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut l = l.split(" | ");
            let input = l
                .next()
                .unwrap()
                .split_whitespace()
                .map(|b| b.to_string())
                .collect();
            let output = l
                .next()
                .unwrap()
                .split_whitespace()
                .map(|b| b.to_string())
                .collect();
            SDisplay { input, output }
        })
        .collect()
}

pub fn part1(input: &[SDisplay]) -> usize {
    let mut count = 0;
    for message in input {
        for num in &message.output {
            match num.len() {
                2 => count += 1,
                3 => count += 1,
                4 => count += 1,
                7 => count += 1,
                _ => (),
            };
        }
    }
    count
}

fn find_map(line: &[String]) -> Vec<char> {
    let mut map = vec!['.'; 8];
    // if contains
    // len == 2 => { find len 6 with only one of them }
    // len == 4               => 4
    // len == 3               => 7
    // len == 7               => 8
    // len == 5 && map[2, !5] => 2
    // len == 5 && map[1]     => 5
    // len == 5 && map[2, 5]  => 3
    // len == 6 && map[!2, 5] => 6
    // len == 6 && map[2, 3]  => 9
    // len == 6 && map[!3]    => 0
    let one: Vec<char> = line
        .iter()
        .find(|x| x.len() == 2)
        .unwrap()
        .chars()
        .collect();
    let six = line
        .iter()
        .find(|x| x.len() == 6 && (x.contains(one[0]) ^ x.contains(one[1])))
        .unwrap();
    if six.contains(one[0]) {
        map[5] = one[0];
        map[2] = one[1];
    } else {
        map[2] = one[0];
        map[5] = one[1];
    }
    let mut four: Vec<char> = line
        .iter()
        .find(|x| x.len() == 4)
        .unwrap()
        .chars()
        .collect();
    four.retain(|&v| v != one[0] && v != one[1]);
    let zero = line
        .iter()
        .find(|x| x.len() == 6 && (x.contains(four[0]) ^ x.contains(four[1])))
        .unwrap();
    if zero.contains(four[0]) {
        map[1] = four[0];
        map[3] = four[1];
    } else {
        map[3] = four[0];
        map[1] = four[1];
    }
    map
}

fn decode(line: &[String], map: &[char]) -> usize {
    let mut result = 0;
    for (i, num_string) in line.iter().enumerate() {
        result += 10_usize.pow(3 - i as u32)
            * match num_string.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 => {
                    if num_string.contains(map[1]) {
                        5
                    }
                    // 5 has upper left bar
                    else if num_string.contains(map[5]) {
                        3
                    }
                    // 3 has lower right bar but not upper left
                    else {
                        2
                    }
                }
                6 => {
                    if !num_string.contains(map[2]) {
                        6
                    }
                    // six doesn't have upper right bar
                    else if num_string.contains(map[3]) {
                        9
                    }
                    // contains middle, not 0
                    else {
                        0
                    }
                }
                _ => unreachable!(),
            }
    }
    result
}

pub fn part2(input: &[SDisplay]) -> usize {
    let mut count = 0;
    for message in input {
        let map = find_map(&message.input);
        count += decode(&message.output, &map);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;
        assert_eq!(part1(&parse(input)), 26);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;
        assert_eq!(part2(&parse(input)), 61229);
    }
}
