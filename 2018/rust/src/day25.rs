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

struct Set {
    parent: usize,
    rank: usize,
}

pub struct DisjointSet {
    set: Vec<Set>,
    trees: usize,
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
    constellations.trees
}

impl Point {
    fn manhatten(&self, other: Point) -> i32 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }
}

impl Set {
    fn new(id: usize) -> Set {
        Set {
            rank: 0,
            parent: id,
        }
    }
}

impl DisjointSet {
    fn new(trees: usize) -> DisjointSet {
        let set = (0..trees).map(Set::new).collect();
        DisjointSet { set, trees }
    }

    fn find(&mut self, idx: usize) -> usize {
        let mut idx = idx;
        while self.set[idx].parent != idx {
            idx = self.set[idx].parent;
            self.set[idx].parent = self.set[self.set[idx].parent].parent;
        }
        idx
    }

    fn union(&mut self, idx: usize, idy: usize) {
        let x = self.find(idx);
        let y = self.find(idy);
        if x != y {
            self.trees -= 1;
            if self.set[x].rank < self.set[y].rank {
                self.set[x].parent = y;
            } else {
                self.set[y].parent = x;
                if self.set[x].rank == self.set[y].rank {
                    self.set[x].rank += 1
                };
            }
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
