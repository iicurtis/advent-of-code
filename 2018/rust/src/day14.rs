// Advent of Code
// Copyright (C) 2018  Isaac Curtis

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

type Error = Box<dyn std::error::Error>;

const MEMOIZED_STATE: [u8; 20] = [3,7,1,0,1,0,1,2,4,5,1,5,8,9,1,6,7,7,9,2];
const MEMOIZED_INIT: (usize, usize) = (8, 4);

pub fn solve(input: &str) -> Result<String, Error> {
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

fn get_digits(input: u8, recipes: &mut Vec<u8>) {
    if input >= 10 {
        recipes.push(1);
    }
    recipes.push(input % 10);
}

pub fn part1(input: &str) -> String {
    let input = input.trim().parse::<usize>().unwrap();
    let mut recipes = vec![3, 7];
    let (mut a, mut b) = (0, 1);
    while recipes.len() < input + 10 {
        get_digits(recipes[a] + recipes[b], &mut recipes);

        a = (1 + a + recipes[a] as usize) % recipes.len();
        b = (1 + b + recipes[b] as usize) % recipes.len();
    }
    recipes[input..input + 10]
        .iter()
        .map(|c| format!("{}", c))
        .collect()
}

pub fn part2(input: &str) -> usize {
    let score_len = input.trim().len();
    let goal = input.trim().bytes().map(|b| b - b'0').collect::<Vec<u8>>();
    let mut needle = Needle { data: &goal, len: score_len, pos: 0 };

    let mut recipes = vec![1; 22 <<  20];
    let mut count = MEMOIZED_STATE.len();
    recipes[0..count].copy_from_slice(&MEMOIZED_STATE);
    let (mut elf1, mut elf2) = MEMOIZED_INIT;

    loop {
        let (move1, move2) = (recipes[elf1] + 1, recipes[elf2] + 1);
        let mut new_recipe = recipes[elf1] + recipes[elf2];
        if new_recipe >= 10 {
            if needle.find(1) { break count+1 - score_len }
            count += 1;
            new_recipe -= 10;
        }
        recipes[count] = new_recipe;
        if needle.find(new_recipe) { break count+1 - score_len }
        count += 1;

        elf1 += move1 as usize;
        elf2 += move2 as usize;

        if elf1 >= count { elf1 -= count }
        if elf2 >= count { elf2 -= count }
    }
}

struct Needle<'a, T> {
    data: &'a [T],
    len: usize,
    pos: usize,
}


impl<'a, T: Eq> Needle<'a, T> {
    fn find(&mut self, t: T) -> bool {
        if self.data[self.pos] == t {
            self.pos += 1;
            self.pos == self.len
        } else {
            self.pos = 0;
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "9";
        assert_eq!(part1(input), "5158916779");
    }

    #[test]
    fn test_part1_1() {
        let input = "5";
        assert_eq!(part1(input), "0124515891");
    }

    #[test]
    fn test_part1_2() {
        let input = "18";
        assert_eq!(part1(input), "9251071085");
    }

    #[test]
    fn test_part1_3() {
        let input = "2018";
        assert_eq!(part1(input), "5941429882");
    }

    // #[test]
    // fn test_part2() {
        // let input = "51589";
        // assert_eq!(part2(input), 9);
    // }

    // #[test]
    // fn test_part2_1() {
        // let input = "01245";
        // assert_eq!(part2(input), 5);
    // }

    // #[test]
    // fn test_part2_2() {
        // let input = "92510";
        // assert_eq!(part2(input), 18);
    // }

    #[test]
    fn test_part2_3() {
        let input = "59414";
        assert_eq!(part2(input), 2018);
    }

}
