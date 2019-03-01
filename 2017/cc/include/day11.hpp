//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #11          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <cassert>
#include <iterator>
#include <locale>

struct Hex {
  int q, r, s;
  Hex(int q_, int r_, int s_): q(q_), r(r_), s(s_) {
    assert ( (q + r + s) == 0);
  }
};

template<>
template<bool part2>
void day<11>::solve(std::istream& is, std::ostream& os)
{
  int max{0};
  std::string b;
  is.imbue(std::locale{is.getloc(), new csv_reader});
  std::vector<std::string> dirs{std::istream_iterator<std::string>{is}, {}};

  auto hex_length = [](auto a) {
    return (std::abs(a.q) + std::abs(a.r) + std::abs(a.s)) / 2;
  };
  Hex c(0, 0, 0);
  for (auto b : dirs) {
    if (b == "sw") c.s++, c.q--;
    else if (b == "ne") c.s--, c.q++;
    else if (b == "s") c.r--, c.s++;
    else if (b == "n") c.r++, c.s--;
    else if (b == "nw") c.q--, c.r++;
    else if (b == "se") c.q++, c.r--;
    max = std::max(max, hex_length(c));
  }
  if (part2)
    os << max << std::endl;
  else
    os << hex_length(c) << std::endl;
}
