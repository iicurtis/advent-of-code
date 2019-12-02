# advent2018-rust

[Advent of Code 2018](http://adventofcode.com/2018/) rust solutions.

# Solution summaries


## Day 02

A few different ideas on this one.

### Method 1 - bit difference sum

This method takes `0.5 (n^2 - n) * M`, where M is char count and n is line
count. Note that when using SIMD, the M can be ignored.

```
for line in lines:
    for other in lines.skip(line):
        if line.bitwise_eq(other).sum() == 31:  # everything equal but 1
            done
```

### Method 2 - set

Hash left and right sides, ignoring the middle character.

```
parts = HashSet::with_capacity(line_len);
for split in line_len..:
    for line in lines:
        left, right = (line[..split], line[split + 1..])
        if !parts.insert((left, right)):
            done
```

### Method 3 - Voltara

This effectively combines Method 1 + 2

* 2 hashes that split every 2 letters:

```
h1 = hash("ab00cd00ef00...")
h2 = hash("00gh00ij00kl...")
```

* Sort hashes
* Collect matching hashes
* Remove duplicates
* Use method 1 on resulting vector

Notes: Method 1 with SIMD is much faster on its own.


### SIMD

Each 26-character word fits in a 32-byte AVX2 (256 bit) register, so comparisons
can be very fast.

## Day 21

```
#ip  3
00 seti 123      0        5  // reg[5] = 123
01 bani 5        456      5  // reg[5] = reg[5] & 456
02 eqri 5        72       5  // if reg[5] == 72 { goto 04 } else { goto 05 }
03 addr 5        3        3  // ^^^
04 seti 0        0        3  // goto 01
05 seti 0        5        5  // reg[5] = 5
06 bori 5        65536    2  // reg[2] = reg[5] | 65536
07 seti 10362650 3        5  // reg[5] = 10362650
08 bani 2        255      4  // reg[4] = reg[2] & 255
09 addr 5        4        5  // reg[5] = reg[5] + reg[4]
10 bani 5        16777215 5  // reg[5] = reg[5] & 16777215
11 muli 5        65899    5  // reg[5] = reg[5] * 65899
12 bani 5        16777215 5  // reg[5] = reg[5] & 16777215
13 gtir 256      2        4  // if reg[2] < 256 { goto 16 } else { goto 15 }
14 addr 4        3        3  // ^^^
15 addi 3        1        3  // goto 17
16 seti 27       4        3  // goto 28
17 seti 0        3        4  // reg[4] = 0
18 addi 4        1        1  // reg[1] = reg[4] + 1
19 muli 1        256      1  // reg[1] = reg[1] * 256
20 gtrr 1        2        1  // if reg[1] > reg[2] { goto 23 } else { goto 22 }
21 addr 1        3        3  // ^^^
22 addi 3        1        3  // goto 24
23 seti 25       2        3  // goto 26
24 addi 4        1        4  // reg[4] = reg[4] + 1
25 seti 17       7        3  // goto 18
26 setr 4        0        2  // reg[2] = reg[4]
27 seti 7        8        3  // goto 8
28 eqrr 5        0        4  // if reg[5] == reg[0] { halt } else { goto 6 }
29 addr 4        3        3  // ^^^
30 seti 5        1        3  // ^^^
```

Which kind of seems to go to:

```
reg[5] = 5
do {
    reg[2] = reg[5] | 0x10000
    reg[5] = INPUT

    loop {
        reg[5] = (reg[5] + (reg[2] & 0xff)) & 0xffffff;
        reg[5] = (reg[5] * 0x1016B) & 0xffffff;
        if (reg[2] < 256) { break; }
        reg[2] >>= 8;
    }

} while reg[5] != reg[0]

```

