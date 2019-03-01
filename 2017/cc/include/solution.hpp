#ifndef _SOLUTION_HPP_
#define _SOLUTION_HPP_

#include <iostream>
#include <string>
#include <vector>


template <int day_of_advent>
struct day {
  static std::string text() {
    return std::string{(day_of_advent < 10) ? "day0" : "day"} + std::to_string(day_of_advent);
  }

  template <bool part2>
  static void solve(std::istream&, std::ostream&);
};

using day_list = std::integer_sequence<int, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25>;


struct csv_reader: std::ctype<char> {
  csv_reader(): std::ctype<char>(get_table()) {}
  static std::ctype_base::mask const* get_table() {
    static std::vector<std::ctype_base::mask> rc(table_size, std::ctype_base::mask());

    rc[','] = std::ctype_base::space;
    rc['\n'] = std::ctype_base::space;
    rc[' '] = std::ctype_base::space;
    return &rc[0];
  }
};

#endif
