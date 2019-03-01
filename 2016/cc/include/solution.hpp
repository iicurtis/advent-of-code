#ifndef _SOLUTION_HPP_
#define _SOLUTION_HPP_

#include <fstream>
#include <iostream>
#include <string>

enum Day {
  day01,
  day02,
  day03,
  day04,
  day05,
  day06,
  day07,
  day08,
  day09,
  day10,
  day11,
  day12,
  day13,
  day14,
  day15,
  day16,
  day17,
  day18,
  day19,
  day20,
  day21,
  day22,
  day23,
  day24,
  day25,
  TOTAL_DAYS
};

std::string
asString(Day d);

template <Day DAY>
void
solve(bool part2, std::istream& is, std::ostream& os)
{
  os << asString(DAY) << " part " << (part2 ? '2' : '1') << " is not implemented";
  if (is.bad())
    os << " and input file does not exist" << std::endl;
  else
    os << std::endl;
}

#endif
