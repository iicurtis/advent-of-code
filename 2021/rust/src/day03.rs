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
            msb = line.len() as u8;
            u16::from_str_radix(line, 2).unwrap()
        })
        .collect();
        Bits { vec, msb }
}

pub fn part1(input: &Bits) -> usize {
    let mut bit_count = vec![0; input.msb as usize];
    for n in &input.vec {
        for i in 0..input.msb as usize {
            bit_count[i] += (n >> i) & 1u16;
        }
    }
    let half = input.vec.len() as u16 / 2;
    let mut gamma = 0;
    for i in 0..input.msb as usize {
        if bit_count[i] > half { gamma |= 1 << i; }
    }
    let not_gamma = !gamma & ((1 << input.msb) - 1);
    gamma * not_gamma
}

pub fn part2(input: &Bits) -> usize {
    let mut o2_vec = input.vec.clone();
    for i in (0..input.msb as usize).rev() {
        let mut ones_list = Vec::new();
        let mut zeroes_list = Vec::new();
        if o2_vec.len() == 1 {
            break;
        }
        for (idx, value) in o2_vec.iter().enumerate().rev() {
            if (value >> i) & 1u16 == 1 {
                ones_list.push(idx);
            } else {
                zeroes_list.push(idx);
            }
        }
        if ones_list.len() >= zeroes_list.len() {  // 1 is the most common value
            // eliminate all values in o2_vec that have 0 at index
            for idx in zeroes_list {
                o2_vec.remove(idx);
            }
            // eliminate all value in co2_vec that have 1 at index
        } else {
            for idx in ones_list {
                o2_vec.remove(idx);
            }
        }
    }
    let mut co2_vec = input.vec.clone();
    for i in (0..input.msb as usize).rev() {
        let mut ones_list = Vec::new();
        let mut zeroes_list = Vec::new();
        if co2_vec.len() == 1 {
            break;
        }
        for (idx, value) in co2_vec.iter().enumerate().rev() {
            if (value >> i) & 1u16 == 1 {
                ones_list.push(idx);
            } else {
                zeroes_list.push(idx);
            }
        }
        if ones_list.len() < zeroes_list.len() {  // 1 is the most common value
            // eliminate all values in o2_vec that have 0 at index
            for idx in zeroes_list {
                co2_vec.remove(idx);
            }
            // eliminate all value in co2_vec that have 1 at index
        } else {
            for idx in ones_list {
                co2_vec.remove(idx);
            }
        }
    }
    let o2_rating: usize = *o2_vec.first().unwrap() as usize;
    let co2_rating: usize = *co2_vec.first().unwrap() as usize;
    o2_rating * co2_rating
}

pub struct Bits {
    vec: Vec<u16>,
    msb: u8,
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
