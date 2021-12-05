// Advent of Code Solutions
// Copyright (C) 2021  Isaac Curtis
type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let input = parse(input);
    let soln1 = part1(&input);
    let soln2 = part2(&input);
    Ok(format!("Part 1: {}\nPart 2: {}", soln1, soln2))
}

pub fn parse(input: &str) -> Map {
    let mut xmax = 0;
    let mut ymax = 0;
    let points = input
        .trim()
        .lines()
        .map(|l| {
            let mut coords = l.split(" -> ");
            let start = coords.next().unwrap().split_once(',').unwrap();
            let start = (start.0.parse().unwrap(), start.1.parse().unwrap());
            let end = coords.next().unwrap().split_once(',').unwrap();
            let end = (end.0.parse().unwrap(), end.1.parse().unwrap());
            if start.0 > xmax {
                xmax = start.0;
            }
            if start.1 > ymax {
                ymax = start.1;
            }
            if end.0 > xmax {
                xmax = end.0;
            }
            if end.1 > ymax {
                ymax = end.1;
            }

            Points { start, end }
        })
        .collect();
    ymax += 1;
    xmax += 1;
    Map { points, xmax, ymax }
}

pub fn part1(input: &Map) -> usize {
    let mut map = vec![0; input.xmax * input.ymax];
    for p in &input.points {
        if p.start.0 == p.end.0 {
            for i in std::cmp::min(p.start.1, p.end.1)..=std::cmp::max(p.start.1, p.end.1) {
                let idx = p.start.0 + input.xmax * i;
                map[idx] += 1;
            }
        } else if p.start.1 == p.end.1 {
            for i in std::cmp::min(p.start.0, p.end.0)..=std::cmp::max(p.start.0, p.end.0) {
                let idx = i + input.xmax * p.start.1;
                map[idx] += 1;
            }
        } else {
            // ignore non vertical or horizontal lines
            continue;
        }
    }
    map.iter().filter(|&&x| x > 1).count()
}

pub fn part2(input: &Map) -> usize {
    let mut map = vec![0; input.xmax * input.ymax];
    for p in &input.points {
        let dx = (p.end.0 as isize - p.start.0 as isize).signum();
        let dy = (p.end.1 as isize - p.start.1 as isize).signum();
        let mut x = p.start.0 as isize;
        let mut y = p.start.1 as isize;
        map[x as usize + y as usize * input.xmax] += 1;
        while x != p.end.0 as isize || y != p.end.1 as isize {
            x += dx;
            y += dy;
            let idx = x as usize + input.xmax * y as usize;
            map[idx] += 1;
        }
    }
    map.iter().filter(|&&x| x > 1).count()
}

pub struct Map {
    points: Vec<Points>,
    xmax: usize,
    ymax: usize,
}

pub struct Points {
    start: (usize, usize),
    end: (usize, usize),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;
        assert_eq!(part1(&parse(input)), 5);
    }

    #[test]
    fn test_part2_0() {
        let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;
        assert_eq!(part2(&parse(input)), 12);
    }
}
