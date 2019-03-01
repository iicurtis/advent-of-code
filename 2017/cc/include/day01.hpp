//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #01          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <iterator>

template<>
template<bool part2>
void day<1>::solve(std::istream& is, std::ostream& os)
{
  int sum{0};
  std::vector<char> const in(std::istream_iterator<char>{is}, {});
  std::size_t const N{in.size()};
  std::size_t const offset{part2 ? N / 2 : 1};
  for (std::size_t i{0lu}; i < N; ++i) {
      if (in[i] == in[(i + offset) % N]) sum += (in[i] - '0');
  }
  os << sum << std::endl;
}
