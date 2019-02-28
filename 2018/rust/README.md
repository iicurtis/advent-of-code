# adventofcode
My personal implementations for
[Advent of Code (AoC)](https://adventofcode.com/)

## Instructions

The code is designed to take input from `stdin`. A wrapper program *run_day.sh*
exists to run code from the *inputs/* directory.

### Install cargo-aoc

This requires `cargo-aoc` to work. 

```
cargo install cargo-aoc
```

### Running a single day

```
cargo aoc -d 1
```

### Running all days

```
cargo run --release
```

### Running custom input

I actually don't know how to do this with `cargo-aoc`. It does lose a bit of its
flexibility. But its nice to get benchmarks and have a coding paradigm very
similar to others.


## Other Rust AoC

Be sure to check out the Rust Discord! We are on there every day.

* [BurntSushi/advent-of-code](https://github.com/BurntSushi/advent-of-code)
* [Globidev/advent-of-code](https://github.com/Globidev/advent-of-code)
* [birkenfield/advent18](https://github.com/birkenfeld/advent18)
* [kcaffrey/aoc-rs](https://github.com/kcaffrey/aoc-rs)
* [AuroransSolic/aoc_2018](https://github.com/AuroransSolis/aoc_2018)
* [gobanos/advent-of-code-2018](https://github.com/gobanos/advent-of-code-2018)
* [anowell/advent-of-code](https://github.com/anowell/advent-of-code)
* [zesterer/advent-of-code-2018](https://github.com/zesterer/advent-of-code-2018)
* [troiganto/aoc-2018](https://github.com/troiganto/aoc-2018)

## TODO

* [ ] Add tests
* [ ] Follow BurntSushi's example and don't panic
