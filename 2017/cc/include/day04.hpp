//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #04          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <vector>
#include <sstream>
#include <algorithm>
#include <iterator>
#include <set>

bool
is_anagram(std::string s1, std::string s2){
  std::sort(s1.begin(), s1.end());
  std::sort(s2.begin(), s2.end());
  return s1 == s2;
}

template<>
template<bool part2>
void day<4>::solve(std::istream& is, std::ostream& os)
{
  int count{0};
  for(std::string line; std::getline(is, line);) {
    std::stringstream iss(line);
    count += std::all_of(
          std::istream_iterator<std::string>{iss}, {},
          [set = std::set<std::string>()] (std::string word) mutable {
            if constexpr (part2) std::sort(word.begin(), word.end());
            return set.insert(word).second;
          });
    }
    os << count << std::endl;
}
