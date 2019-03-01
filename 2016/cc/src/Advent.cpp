//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// December 2016    //
//////////////////////

#include "preprocessor.hpp"
#include "solution.hpp"
#include <array>
#include <regex>
#include <iomanip>

template <Day>
double
timeSolve(bool, bool);

double
run(Day, bool, std::ostream&);

int main(int argc, char* argv[])
{
  std::ofstream DEVNULL{"/dev/null"};
  std::ostream  os{std::cout.rdbuf()};
  for (int d{day01}; d != TOTAL_DAYS; ++d) {
    Day day{static_cast<Day>(d)};
    os << asString(day) << "\n";
    os << "Part 1: ";
    run(day, false, os);
    os << "Part 2: ";
    run(day, true, os);
  }
  return EXIT_SUCCESS;
}

template <Day DAY>
double
timeSolve(bool part2, std::ostream& os)
{
  std::ifstream is{"../inputs/" + asString(DAY) + ".txt"};
  solve<DAY>(part2, is, os);
  return 0.0;
}

#define RUN_DAY(X) \
  case day##X:     \
    return timeSolve<day##X>(part2, os);
double
run(Day day, bool part2, std::ostream& os)
{
  switch(day) {
    EVAL(MAP(RUN_DAY, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25))
    default:
      return 0.0;
  }
}

