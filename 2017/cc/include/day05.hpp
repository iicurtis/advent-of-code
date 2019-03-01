//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #05          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <vector>
#include <iterator>

template<>
template<bool part2>
void day<5>::solve(std::istream& is, std::ostream& os)
{
  std::vector<int> instr{std::istream_iterator<int>{is}, {}};
  int count{0}, pos{0}, n(instr.size());
  while (pos >= 0 && pos < n) {
    ++count;
    if (part2 && instr[pos] >= 3) pos += instr[pos]--;
    else pos += instr[pos]++;
  }
  os << count << std::endl;
}
