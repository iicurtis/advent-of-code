// Advent of Code
// Copyright (C) 2018  Isaac Curtis

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let (soln1, soln2) = day19(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn day19(input: &str) -> (usize, usize) {
    let mut inputiter = input.trim().lines();

    let ip_reg = inputiter.next().unwrap().as_bytes()[4] - b'0';

    // parse instructions
    let instructions: Vec<[i8; 4]> = inputiter
        .map(|line| {
            let mut current_instr = [0; 4];
            let mut tokens = line.split(' ');
            let mnemonic = tokens.next().unwrap();
            current_instr[0] = match mnemonic {
                "addi" => -7,
                "addr" => 2,
                "eqrr" => 66,
                "muli" => -51,
                "mulr" => -42,
                "seti" => 77,
                "setr" => 86,
                "gtrr" => 4,
                invalid => panic!("Invalid op code: {:?}", invalid),
            };
            current_instr[1] = tokens.next().unwrap().parse::<i8>().unwrap();
            current_instr[2] = tokens.next().unwrap().parse::<i8>().unwrap();
            current_instr[3] = tokens.next().unwrap().parse::<i8>().unwrap();
            current_instr
        })
        .collect();

    let mut soln = [0, 0];
    for part in [0, 1].iter() {
        let mut register = [0; 8];
        register[0] = *part;
        while register[ip_reg as usize] < instructions.len() {
            let instr = instructions[register[ip_reg as usize]];
            let instr_a = instr[1] as usize;
            let instr_b = instr[2] as usize;
            let instr_c = instr[3] as usize;
            match instr[0] {
                -7 => register[instr_c & 7] = register[instr_a & 7] + instr_b, // addi
                2 => register[instr_c & 7] = register[instr_a & 7] + register[instr_b & 7], // addr
                66 => {
                    soln[*part] = register[instr_b & 7];
                    break;
                } // eqrr
                -51 => register[instr_c & 7] = register[instr_a & 7] * instr_b, // muli
                -42 => register[instr_c & 7] = register[instr_a & 7] * register[instr_b & 7], // mulr
                77 => register[instr_c & 7] = instr_a, // seti
                86 => register[instr_c & 7] = register[instr_a & 7], // setr
                _ => panic!("Impossible instruction"),
            }
            register[ip_reg as usize] += 1;
        }

        soln[*part] = divisor_sum(soln[*part]);
    }
    (soln[0], soln[1])
}

fn try_factor(n: &mut usize, f: &usize, d_sum: &mut usize) {
    if *n % f != 0 {
        return;
    }
    let mut mult = 1;
    let mut fk = 1;
    while *n % f == 0 {
        *n /= f;
        fk *= f;
        mult += fk;
    }
    *d_sum *= mult;
}

fn divisor_sum(n: usize) -> usize {
    let mut n = n;
    let mut d_sum = 1;

    for f in [2, 3, 5].iter() {
        try_factor(&mut n, f, &mut d_sum);
    }

    let mut step = 0x6264_2424;
    let mut f = 7;
    while f * f <= n {
        try_factor(&mut n, &f, &mut d_sum);
        f += step & 15;
        step = (step << 28) | (step >> 4);
    }

    if n > 1 {
        d_sum *= n + 1;
    }

    d_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
#ip 3
addi 3 16 3
seti 1 2 5
seti 1 3 2
mulr 5 2 1
eqrr 1 4 1
addr 1 3 3
addi 3 1 3
addr 5 0 0
addi 2 1 2
gtrr 2 4 1
addr 3 1 3
seti 2 5 3
addi 5 1 5
gtrr 5 4 1
addr 1 3 3
seti 1 2 3
mulr 3 3 3
addi 4 2 4
mulr 4 4 4
mulr 3 4 4
muli 4 11 4
addi 1 6 1
mulr 1 3 1
addi 1 21 1
addr 4 1 4
addr 3 0 3
seti 0 3 3
setr 3 4 1
mulr 1 3 1
addr 3 1 1
mulr 3 1 1
muli 1 14 1
mulr 1 3 1
addr 4 1 4
seti 0 3 0
seti 0 7 3
"#;
        assert_eq!(day19(&input), (1056, 10915260));
    }
}
