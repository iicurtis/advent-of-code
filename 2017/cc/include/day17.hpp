//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #17          //
//////////////////////

// A general description of the goal

#include "solution.hpp"

template<>
template<bool part2>
void day<17>::solve(std::istream& is, std::ostream& os)
{
  int step;
  is >> step;
  if (part2) {
    int final{0}, pos{0};
    for (int i{1}; i<=50000000; ++i) {
      pos = ((pos + step) % i) + 1;
      if (pos == 1)
        final = i;
    }
    os << final << '\n';
  } else {
    int pos{0};
    std::vector<int> buffer{0};
    buffer.reserve(2017);
    for (int i{1}; i<=2017; ++i) {
      pos = ((pos + step) % buffer.size()) + 1;
      buffer.insert(buffer.begin() + pos, i);
    }
    os << buffer[(pos+1) % buffer.size()] << "\n";
  }
}
