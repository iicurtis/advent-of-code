//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #14          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <numeric>
#include <algorithm>
#include <iterator>
#include <bitset>
#include <climits>
#include <utility>
#include <iomanip>

auto knothash (std::vector<int> input) {
  std::vector<int> string(256);
  std::vector<size_t> dense_hash;
  std::iota(string.begin(), string.end(), 0);
  unsigned char skip_size{0}, delta{0}, pos{0};
  input.insert(input.end(), {17, 31, 73, 47, 23});
  for (size_t i{0}; i < 64; ++i)
    for (unsigned char len : input) {
      std::reverse(string.begin(), string.begin()+len);
      delta = len + skip_size++;
      pos += delta;
      std::rotate(string.begin(), string.begin()+delta, string.end());
    }
  std::rotate(string.begin(), string.end() - pos, string.end());
  for (auto b = string.begin(); b != string.end(); std::advance(b, 16))
    dense_hash.emplace_back(std::accumulate(b, std::next(b, 16), 0, std::bit_xor<void>()));
  return dense_hash;
}

void recursive_delete(size_t i, size_t j, std::vector<std::vector<bool>> &m) {
  m[i][j] = 0;
  if (i > 0)
    if(m[i-1][j])
      recursive_delete(i-1, j, m);
  if (i < 127)
    if (m[i+1][j])
      recursive_delete(i+1, j, m);
  if (j > 0)
    if (m[i][j-1])
      recursive_delete(i, j-1, m);
  if (j < 127)
    if (m[i][j+1])
      recursive_delete(i, j+1, m);
}

template<>
template<bool part2>
void day<14>::solve(std::istream& is, std::ostream& os)
{
  std::vector<int> lengths{std::istream_iterator<char>{is}, {}};
  lengths.insert(lengths.end(), {'-'});
  std::vector<int> input = lengths;
  int sum{0};
  std::vector<std::vector<bool> > matrix;
  for (size_t i{0}; i<128; ++i) {
    input = lengths;
    std::string num = std::to_string(i);
    std::copy(num.begin(), num.end(), std::back_inserter(input));
    std::vector<size_t> hash = knothash(input);
    std::vector<bool> m;
    for (auto j : hash) {
      std::bitset<8> b(j);
      sum += b.count();
      for (size_t k{b.size()}; k; --k)
        m.emplace_back(b[k-1]);
    }
    matrix.emplace_back(m);
  }
  if (part2) {
    int ngroups{0};
    for (size_t i{0}; i<matrix.size(); ++i) {
      for (size_t j{0}; j<matrix[i].size(); ++j) {
        if (matrix[i][j]) {
          ngroups++;
          recursive_delete(i, j, matrix);
        }
      }
    }
    os << ngroups << '\n';
  }
  else
    os << sum << '\n';
}
