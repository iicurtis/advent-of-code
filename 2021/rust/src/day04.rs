// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

fn score(draws: &[usize], board: &[Vec<usize>]) -> usize {
    board.iter().flatten().filter(|x| !draws.contains(x)).sum()
}

fn board_win(draws: &[usize], board: &[Vec<usize>]) -> Option<usize> {
    for i in 0..5 {
        if (0..5).all(|j| draws.contains(&board[i][j])) {
            return Some(score(draws, board) * draws.last().unwrap());
        }
        if (0..5).all(|j| draws.contains(&board[j][i])) {
            return Some(score(draws, board) * draws.last().unwrap());
        }
    }
    None
}

pub fn parse(input: &str) -> Bingo {
    let mut sections = input.trim().split("\n\n");
    let draws = sections
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let boards: Vec<Vec<Vec<usize>>> = sections
        .map(|b| {
            b.lines()
                .map(|l| {
                    l.split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    Bingo { draws, boards }
}

pub fn part1(input: &Bingo) -> usize {
    for i in 5..input.draws.len() {
        let winner = input
            .boards
            .iter()
            .find_map(|b| board_win(&input.draws[0..i], b));
        if let Some(score) = winner {
            return score;
        }
    }
    unreachable!()
}

pub fn part2(input: &Bingo) -> usize {
    let mut final_score = 0;
    let mut boards = input.boards.clone();
    for i in 5..input.draws.len() {
        boards.retain(|b| {
            if let Some(score) = board_win(&input.draws[0..i], b) {
                final_score = score;
                false
            } else {
                true
            }
        });
    }
    final_score
}

pub struct Bingo {
    draws: Vec<usize>,
    boards: Vec<Vec<Vec<usize>>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;
        assert_eq!(part1(&parse(input)), 4512);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;
        assert_eq!(part2(&parse(input)), 1924);
    }
}
