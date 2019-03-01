//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #01          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <fmt/format.h>
#include <set>

template <>
void
solve<day01>(bool part2, std::istream& is, std::ostream& os)
{
  int pos[2]{0, 0};
  char d;
  std::set<std::pair<int, int>> visited;
  for ( int index{1}, dist; is >> d >> dist; is.ignore(1, ',') ) {
      index += ~d & 3;
      for (int i{0}; i < dist; ++i) {
          pos[index & 1] += 1 - (index & 2);
          if (!visited.emplace(pos[0], pos[1]).second && part2) {
              os << (std::abs(pos[0])) + (std::abs(pos[1])) << std::endl;
              return;
            }
        }
    }
  os << std::abs(pos[0]) + (std::abs(pos[1])) << std::endl;
}
