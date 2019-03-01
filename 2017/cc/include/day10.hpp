//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #10          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <iterator>
#include <sstream>
#include <locale>
#include <algorithm>
#include <numeric>
#include <iomanip>
#include <utility>

template<>
template<bool part2>
void day<10>::solve(std::istream& is, std::ostream& os)
{
  std::vector<int> string(256);
  unsigned char skip_size{0}, delta{0}, pos{0};
  std::iota(string.begin(), string.end(), 0);
  std::vector<int> lengths;
  if (part2) {
    lengths.insert(lengths.end(), std::istream_iterator<char>{is}, {});
    lengths.insert(lengths.end(), {17, 31, 73, 47, 23});
  }
  else {
    for (int i; is >> i; )  { // read in lengths
      lengths.push_back(i);
      if (is.peek() == ',')
        is.ignore();
    }
  }

  int rounds{(part2 ? 64 : 1)};
  for (int i{0}; i < rounds; ++i){
    for (auto len : lengths) {
      std::reverse(string.begin(), string.begin()+len);
      delta = len + skip_size++;
      pos += delta;
      std::rotate(string.begin(), string.begin()+delta, string.end());
    }
  }
  std::rotate(string.begin(), string.end() - pos, string.end());

  if (part2) {
    auto const [flags, fill] = std::pair(os.flags(std::ios::hex), os.fill('0'));
    for (auto b = string.begin(); b != string.end(); std::advance(b, 16))
      os << std::setw(2) << std::accumulate(b, std::next(b, 16), 0, std::bit_xor<void>());
    os.flags(flags), os.fill(fill);
  }
  else
    os << string[0] * string[1];
  os << std::endl;
}
