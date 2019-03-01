//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// December 2017    //
//////////////////////

#include <array>
#include <regex>
#include <iomanip>
#include <functional>
#include <fstream>

#include "timer.hpp"
#include "solution.hpp"

#include "day01.hpp"
#include "day02.hpp"
#include "day03.hpp"
#include "day04.hpp"
#include "day05.hpp"
#include "day06.hpp"
#include "day07.hpp"
#include "day08.hpp"
#include "day09.hpp"
#include "day10.hpp"
#include "day11.hpp"
#include "day12.hpp"
#include "day13.hpp"
#include "day14.hpp"
#include "day15.hpp"
#include "day16.hpp"
#include "day17.hpp"
#include "day18.hpp"
#include "day19.hpp"
#include "day20.hpp"
#include "day21.hpp"
#include "day22.hpp"
#include "day23.hpp"
#include "day24.hpp"
#include "day25.hpp"


template <typename, bool>
double run_instance(bool, std::ostream&);

template <typename>
double run_single(std::ostream&);

template <int ... days>
double run_all(std::ostream& os, std::integer_sequence<int, days...>) {
  return (... + run_single<day<days>>(os));
}

int main(int argc, char* argv[])
{
  std::ostream  os{std::cout.rdbuf()};
  double total_time = run_all(os, day_list{});
  printf("  time: %.51fms\n", total_time);
  return EXIT_SUCCESS;
}

template <typename daytorun>
double run_single(std::ostream& os) {
  double total_time {0.0};
  os << daytorun::text() << "\n";
  os << "Part 1: ";
  total_time += run_instance<daytorun, false>(2, os);
  os << "Part 2: ";
  total_time += run_instance<daytorun, true>(2, os);
  return total_time;
}

template <typename daytorun, bool part2>
double run_instance(bool time, std::ostream& os)
{
  std::ifstream is{"../inputs/" + daytorun::text() + ".txt"};
  if (time) {
    Timer<> t;
    daytorun::template solve<part2>(is, os);
    double res_time = t.current<std::milli>();
    os.precision(5);
    os << " time: ";
    os.setf(std::ios::fixed, std::ios::floatfield);
    os << res_time << "ms\n";
    return res_time;
  } else {
    daytorun::template solve<part2>(is, os);
    return 0.0;
  }
}
