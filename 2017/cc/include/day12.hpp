//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #12          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <iterator>
#include <sstream>
#include <set>
#include <regex>

static std::regex const NUM {R"(\d+)", std::regex::optimize};

template<>
template<bool part2>
void day<12>::solve(std::istream& is, std::ostream& os)
{
  std::map<int, std::vector<int>> nodes;
  for (std::string line; std::getline(is, line); ) {
    auto nums = std::sregex_iterator(std::begin(line), std::end(line), NUM);
    int n = std::stoi(nums->str());
    std::vector<int> children;
    while (++nums != std::sregex_iterator())
      children.push_back(std::stoi(nums->str()));
    nodes.emplace(n, children);
  }
  int ngroups{0};
  std::set<int> seen;
  std::deque<int> parse_list;
  for (size_t i{0}; i<nodes.size(); ++i) {
    if (seen.find(i) != seen.end())
      continue;
    parse_list.push_back(i);
    ngroups++;
    while(!parse_list.empty()) {
      auto n = parse_list.back();
      parse_list.pop_back();
      if (!seen.insert(n).second)
        continue;
      for (auto child : nodes[n])
        parse_list.push_back(child);
    }
    if (ngroups == 1 && !part2)
      os << seen.size() << "\n";
  }
  if (part2)
    os << ngroups << std::endl;
}
