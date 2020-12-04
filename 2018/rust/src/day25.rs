// Advent of Code
// Copyright (C) 2018  Isaac Curtis

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<String, Error> {
    let map = parse(&input);
    let soln1 = part1(&map);
    Ok(format!("Part 1: {}\n Merry Christmas!", soln1))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point(i32, i32, i32, i32);

pub struct DisjointSet {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

pub fn parse(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut coords = line.split(',');
            Point(
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &[Point]) -> usize {
    let mut constellations = DisjointSet::new(input.len());
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i].manhatten(input[j]) < 4 {
                constellations.union(i, j)
            }
        }
    }
    constellations.count
}

impl Point {
    fn manhatten(&self, other: Point) -> i32 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }
}

impl DisjointSet {
    fn new(count: usize) -> DisjointSet {
        let id = (0..count).collect();
        let size = vec![1; count];
        DisjointSet { id, count, size }
    }

    fn find(&mut self, idx: usize) -> usize {
        let mut root = idx;
        while self.id[root] != root {
            root = self.id[root];
        }
        root
    }

    fn union(&mut self, idx: usize, idy: usize) {
        let x = self.find(idx);
        let y = self.find(idy);
        if x != y {
            if self.size[idx] < self.size[idy] {
                self.id[idx] = idy;
                self.size[idy] += self.size[idx];
            } else {
                self.id[idy] = idx;
                self.size[idx] += self.size[idy];
            }
            self.count -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
"#;
        assert_eq!(part1(&parse(input)), 2);
    }

    #[test]
    fn test_part1_1() {
        let input = r#"
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
"#;
        assert_eq!(part1(&parse(input)), 4);
    }

    #[test]
    fn test_part1_2() {
        let input = r#"
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
"#;
        assert_eq!(part1(&parse(input)), 3);
    }

    #[test]
    fn test_part1_3() {
        let input = r#"
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
"#;
        assert_eq!(part1(&parse(input)), 8);
    }
}
