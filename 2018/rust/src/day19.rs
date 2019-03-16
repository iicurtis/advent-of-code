// Advent of Code
// Copyright (C) 2018  Isaac Curtis

type Error = Box<std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let (soln1, soln2) = day19(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn day19(input: &str) -> (usize, usize) {
    let mut inputiter = input.as_bytes().iter();

    // parse first line of file for IP register
    let mut ip_reg = -1;
    while let Some(c) = inputiter.next() {
        if (c - b'0') < 6 {
            ip_reg = (c - b'0') as isize;
            break;
        }
    }

    if ip_reg == -1 {
        panic!("Error: file invalid");
    }

    // parse instructions
    let mut instructions = Vec::<[i8; 4]>::with_capacity(36);
    let mut current_instructions = [0; 4];
    let mut have = false;
    let mut n_parse = 0;
    let mut i = 0;
    while let Some(nextbyte) = inputiter.next() {
        let c = nextbyte - b'0';
        if c < 100 {
            have = true;
            n_parse = 10 * n_parse + c;
        } else if have {
            current_instructions[i] = n_parse as i8;
            n_parse = 0;
            have = false;
            i += 1;
            if i == 4 {
                i = 0;
                instructions.push(current_instructions);
            }
        }
    }

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
                -7 => register[instr_c & 7] = register[instr_a & 7] + instr_b,                  // addi
                2 => register[instr_c & 7] = register[instr_a & 7] + register[instr_b & 7] ,    // addr
                66 => { soln[*part] = register[instr_b & 7]; break; },                                // eqrr
                -51 => register[instr_c & 7] = register[instr_a & 7] * instr_b,                 // muli
                -42 => register[instr_c & 7] = register[instr_a & 7] * register[instr_b & 7],   // mulr
                77 =>  register[instr_c & 7] = instr_a,                                          // seti
                86 =>  register[instr_c & 7] = register[instr_a & 7],                            // setr
                _ => panic!("Impossible instruction")
            }
            register[ip_reg as usize] += 1;
        }

    soln[*part] = divisor_sum(soln[*part]);
    }
    return (soln[0], soln[1]);
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

    let mut step = 0x62642424;
    let mut f = 7;
    while f * f <= n {
        try_factor(&mut n, &f, &mut d_sum);
        f += step & 15;
        step = (step << 28) | (step >> 4);
    }

    if n > 1 {
        d_sum *= n + 1;
    }

    return d_sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
"#;
        assert_eq!(day19(&input), (6, 0));
    }

}
