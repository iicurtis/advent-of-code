# Day 02

## Other solutions

Intriguingly, you can get significant speed ups by comparing only half of the
strings:

* [c] [sjmulder](https://github.com/sjmulder/aoc/blob/master/2019/day02/day02b.c)
* [c++] [askalksi no...](https://www.reddit.com/r/adventofcode/comments/a2hhw1/manually_vectorizing_d2p2/)
* [rust] [CryZe](https://www.reddit.com/r/adventofcode/comments/a2hhw1/manually_vectorizing_d2p2/)

# Day 03

## Other solutions

My solution is naive, and quite slow. In fact its basically the worst way to
implement AABB ever known to mankind. These are some much faster implementations
using the scanline algorithm.

* [rust] [BlockCat](https://github.com/BlockCat/adventofcode2019/blob/master/src/day3.rs)
* [c++] [Yepoleb](https://gist.github.com/Yepoleb/641fa53634b3727bbd784e260a6de993)

# Day 06 - Area of Bounding Box

This appears to be a voronoi problem. But it is unusual to find the area of a
voronoi.

Check out [de-delaunay](https://github.com/d3/d3-delaunay) for better instructions.
