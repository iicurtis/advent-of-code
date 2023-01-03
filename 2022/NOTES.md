# advent2022-rust

[Advent of Code 2022](http://adventofcode.com/2022/) rust solutions.

# Solution summaries


## Day 1: Calorie Counting

Part 1: Sum each group, report max.

Part 2: Sum each group, sum of 3 max.

Example: 
```
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```

### parsing

```rust
pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum()
        })
        .collect()
}
```

## Day 02: Rock Paper Scissors

Part 1: Calculate score based on round of Rock, Paper, Scissors

Part 2: Calculate play to get score on each round of Rock, Paper, Scissors

```
A Y
B X
C Z
```

### parsing

```rust
pub fn parse(input: &str) -> Vec<(u8, u8)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let abc = line[0] - b'A';
            let xyz = line[2] - b'X';
            (abc, xyz)
        })
        .collect()
}
```

## Day 3: Rucksack Reorganization

Part 1: Split at half of string, find the matching character in both halves.
Sum the values assigned to each match.

Part 2: Same as 1, but find match for groups of 3.

```
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
```

### Method

Use u128 to store characters that match. Then AND compare the two sides to see
what overlaps.

### parsing
Key is to split at half of strings and examine each character.

rust
```rust
input
    .trim()
    .lines()
    .map(|line| {
        let bytes = line.as_bytes();
        let (first, second) = bytes.split_at(line.len() / 2);
        (Seen::try_from(first).unwrap() & Seen::try_from(second).unwrap()).priority()
    })
    .sum()

impl TryFrom<&[u8]> for Seen {
    type Error = color_eyre::Report;

    fn try_from(values: &[u8]) -> Result<Self, Self::Error> {
        let mut bits = 0;
        for &c in values {
            bits |= 1 << c;
        }
        Ok(Seen(bits))
    }
}
```
