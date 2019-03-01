//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #09          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <sstream>
#include <limits>

enum State {
  Default, Garbage, Ignore
};

template<>
template<bool part2>
void day<9>::solve(std::istream& is, std::ostream& os)
{
  State s{Default};
  int depth{0}, score{0}, count{0};
  for (char b; is >> b; ) {
    switch (s) {
    case Default: switch (b) {
      case '<': s = Garbage; continue;
      case '{': ++depth;     continue;
      case '}': score += depth--; continue;
      } continue;
    case Garbage: switch (b) {
      case '!': s = Ignore; continue;
      case '>': s = Default; continue;
      default: ++count;     continue;
      } continue;
    case Ignore:
      s = Garbage;
    }
  }
  os << (part2 ? count : score) << std::endl;
}
