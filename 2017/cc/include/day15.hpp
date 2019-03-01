//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #15          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <random>

auto matches(size_t n, int a, int b, int ma, int mb) {
  std::minstd_rand0 gen_a{a};
  std::minstd_rand gen_b{b};
  auto count{0};
  for (unsigned int i{0}; i<n; ++i) {
    std::minstd_rand0::result_type xa;
    std::minstd_rand::result_type xb;
    do xa = gen_a(); while (xa & ma);
    do xb = gen_b(); while (xb & mb);
    if ((xa & 0xFFFF) == (xb & 0xFFFF))
      ++count;
  }
  return count;
}

template<>
template<bool part2>
void day<15>::solve(std::istream& is, std::ostream& os)
{
  unsigned int genA{0}, genB{0};
  is.ignore(24) >> genA;
  is.ignore(24) >> genB;
  if (part2)
    os << matches(5'000'000, genA, genB, 0b11, 0b111) << '\n';
  else
    os << matches(40'000'000, genA, genB, 0, 0) << '\n';
}
