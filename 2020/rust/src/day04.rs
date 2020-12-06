// Advent of Code
// Copyright (C) 2020  Isaac Curtis

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    //let input = parse_input(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .filter(|field| !field.starts_with("cid:"))
        })
        .filter(|field| field.clone().count() == 7)
        .count()
}

pub fn part2(input: &str) -> usize {
    let passports = input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .filter(|field| !field.starts_with("cid:"))
        })
        .filter(|field| field.clone().count() == 7);

    passports
        .map(|mut passport| {
            passport.all(|field| {
                let mut parts = field.split(':');
                let field_name = parts.next().unwrap();
                let field_value = parts.next().unwrap();
                let valid = match field_name {
                    "byr" => (1920..=2002).contains(&field_value.parse::<usize>().unwrap()),
                    "iyr" => (2010..=2020).contains(&field_value.parse::<usize>().unwrap()),
                    "eyr" => (2020..=2030).contains(&field_value.parse::<usize>().unwrap()),
                    "hgt" => {
                        if let Some(hgt) = field_value.strip_suffix("in") {
                            (59..=76).contains(&hgt.parse::<usize>().unwrap())
                        } else if let Some(hgt) = field_value.strip_suffix("cm") {
                            (150..=193).contains(&hgt.parse::<usize>().unwrap())
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        field_value.len() == 7
                            && field_value.starts_with('#')
                            && field_value[1..].chars().all(|c| c.is_digit(16))
                    }
                    "ecl" => {
                        matches!(
                            field_value,
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                        )
                    }
                    "pid" => field_value.len() == 9 && field_value.chars().all(|c| c.is_digit(10)),
                    _ => unreachable!(),
                };
                valid
            })
        })
        .filter(|&p| p)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_1() {
        let input = r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
        assert_eq!(part2(&input), 4);
    }
}
