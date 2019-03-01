//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #16          //
//////////////////////

// A general description of the goal

#include <list>
#include <algorithm>
#include <numeric>
#include <functional>
#include <map>

#include "solution.hpp"

template<>
template<bool part2>
void day<16>::solve(std::istream& is, std::ostream& os)
{
  std::vector<std::function<void()>> cmds;
  std::string a, b;
  char n, m;
  std::string l(16, ' ');
  std::iota(l.begin(), l.end(), 'a');
  for (char act; is >> act;) {
    switch (act) {
    case 's': std::getline(is, a, ',');
      cmds.emplace_back([&l, a] {
          std::rotate(std::begin(l), std::prev(std::end(l), std::stoi(a)), std::end(l));
        });
      break;
    case 'x': std::getline(is, a, '/'); std::getline(is, b, ',');
      cmds.emplace_back([&l, a, b] {
          std::iter_swap(std::begin(l) + std::stoi(a), std::begin(l) + std::stoi(b));
        });
      break;
    case 'p': is >> n; is.ignore(1); is >> m;
      cmds.emplace_back([&l, n, m] {
          std::iter_swap(std::find(l.begin(), l.end(), n), std::find(l.begin(), l.end(), m));
        });
      break;
    }
  }

  auto dance = [&] {
    for (auto & cmd : cmds)
      cmd();
  };

  if (part2) {
    std::map<std::string, int> lookup;
    for (int rep{0}; lookup.emplace(l, rep++).second; )
      dance();
    int rem(1000000000 % lookup.size());
    for (auto const & [s, i] : lookup) {
      if (i == rem) {
        os << s << '\n';
        return;
      }
    }
  } else
    dance();
  os << l;
  os << "\n";
}
