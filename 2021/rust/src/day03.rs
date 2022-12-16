// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse(input: &str) -> Bits {
    let mut msb = 0;
    let vec = input
        .trim()
        .lines()
        .map(|line| {
            msb = line.len();
            u16::from_str_radix(line, 2).unwrap()
        })
        .collect();
    Bits { vec, msb }
}

fn max_bit(numbers: &[u16], pos: usize) -> u16 {
    let mut c = [0, 0];
    for &n in numbers {
        c[(n as usize >> pos) & 1] += 1
    }
    (c[1] >= c[0]) as u16
}

pub fn part1(input: &Bits) -> usize {
    let gamma: usize = (0..input.msb)
        .map(|idx| (max_bit(&input.vec, idx) << idx) as usize)
        .sum();
    let not_gamma = !gamma & ((1 << input.msb) - 1);
    gamma * not_gamma
}

pub fn part2(input: &Bits) -> usize {
    let mut o2_vec = input.vec.clone();
    for i in (0..input.msb).rev() {
        let keep = max_bit(&o2_vec, i) ^ 1;
        o2_vec.retain(|x| (x >> i) & 1 == keep);
        if o2_vec.len() == 1 {
            break;
        }
    }
    let mut co2_vec = input.vec.clone();
    for i in (0..input.msb).rev() {
        let keep = max_bit(&co2_vec, i);
        co2_vec.retain(|x| (x >> i) & 1 == keep);
        if co2_vec.len() == 1 {
            break;
        }
    }
    let o2_rating = *o2_vec.first().unwrap() as usize;
    let co2_rating = *co2_vec.first().unwrap() as usize;
    o2_rating * co2_rating
}

pub struct Bits {
    vec: Vec<u16>,
    msb: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;
        assert_eq!(part1(&parse(input)), 198);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;
        assert_eq!(part2(&parse(input)), 230);
    }
}
