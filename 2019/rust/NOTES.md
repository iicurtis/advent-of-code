# advent2019-rust

[Advent of Code 2019](http://adventofcode.com/2019/) rust solutions.

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
