//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #03          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <fmt/format.h>

template <>
void
solve<day03>(bool part2, std::istream& is, std::ostream& os)
{
  int valid_count{0};
  for (int a, b, c; is >> a >> b >> c;) {
      if (a + b > c && b + c > a && c + a > b)
        valid_count++;
    }
  os << valid_count << std::endl;
}
