//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #02          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <sstream>
#include <algorithm>
#include <iterator>

template<>
template<bool part2>
void day<2>::solve(std::istream& is, std::ostream& os)
{
  int checksum{0};
  for(std::string line; std::getline(is, line);) {
    int max, min;
    std::stringstream iss(line);
    std::vector<int> const n{std::istream_iterator<int>{iss}, {}};
    if (!part2) {
      auto[min, max] = std::minmax_element(std::begin(n), std::end(n));
      checksum += *max - *min;
    }
    else {
      for (auto const i : n) {
        for (auto const j : n) {
          if (i != j && i % j == 0) checksum += i / j;
        }
      }
    }
  }
  os << checksum << std::endl;
}
