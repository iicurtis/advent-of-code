//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #06          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <vector>
#include <map>
#include <iterator>
#include <algorithm>

template<>
template<bool part2>
void day<6>::solve(std::istream& is, std::ostream& os)
{
  std::vector<int> tem{std::istream_iterator<int>{is}, {}};
  std::map< std::vector<int>, int > confs;
  for (int count{0}; confs.emplace(tem, count).second; ++count) {
    auto max = std::max_element(tem.begin(), tem.end());
    for (int iters{std::exchange(*max, 0)}; iters--; ++*max)
      if (++max == tem.end())
        max = tem.begin();
  }
  os << confs.size() - (part2 ? confs[tem] : 0) << std::endl;
}
