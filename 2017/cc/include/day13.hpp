//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #13          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <utility>
#include <algorithm>
#include <type_traits>
#include <iterator>

auto invalid (int delay = 0) {
  return [delay] (auto const & p) {
    auto [layer, range] = p;
    return (delay + layer) % ((range - 1) * 2) == 0;
  };
}

template <unsigned int I, typename TupleLike>
struct View {
  std::tuple_element_t<I, TupleLike> val;
  View(TupleLike v) : val(std::get<I>(v)) {}
  bool operator<(const View &o) const { return val < o.val; }
};

template<>
template<bool part2>
void day<13>::solve(std::istream& is, std::ostream& os)
{
  std::vector<std::pair<int,int>> scan;
  for (int layer, range; (is >> layer).ignore(1, ':') >> range; )
    scan.emplace_back(layer, range);

  if (part2) {
    int delay{0};
    std::sort(std::begin(scan), std::end(scan), std::less<View<1, std::pair<int, int>>>());
    while (std::any_of(scan.begin(), scan.end(), invalid(delay)))
      ++delay;
    os << delay << "\n";
  }
  else {
    int sev{0};
    for (auto [layer, range] : scan)
      if (layer % ((range - 1) * 2) == 0)
        sev += layer * range;
    os << sev << "\n";
  }
}
