//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #08          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <sstream>
#include <algorithm>
#include <map>

template<>
template<bool part2>
void day<8>::solve(std::istream& is, std::ostream& os)
{
  int actmax{0};
  std::map<std::string, int> vars;
  for (std::string line; std::getline(is, line); ) {
    std::string n1, op1, n2, ineq;
    int w1{0}, w2{0};
    std::string iff;
    std::istringstream iss{line};
    iss >> n1 >> op1 >> w1 >> iff >> n2 >> ineq >> w2;
    if (vars.find(n1) == vars.end()) vars[n1] = 0;
    if (vars.find(n2) == vars.end()) vars[n2] = 0;
    if (ineq == ">" && vars[n2] > w2) {
      if (op1 == "inc") {vars[n1] += w1; if(vars[n1]>actmax) actmax=vars[n1]; }
      else if (op1 == "dec") {vars[n1] -= w1; if(vars[n1]>actmax) actmax=vars[n1]; }
    }
    if (ineq == "<" && vars[n2] < w2) {
      if (op1 == "inc") {vars[n1] += w1; if(vars[n1]>actmax) actmax=vars[n1]; }
      else if (op1 == "dec") {vars[n1] -= w1; if(vars[n1]>actmax) actmax=vars[n1]; }
    }
    if (ineq == ">=" && vars[n2] >= w2) {
      if (op1 == "inc") {vars[n1] += w1; if(vars[n1]>actmax) actmax=vars[n1]; }
      else if (op1 == "dec") {vars[n1] -= w1; if(vars[n1]>actmax) actmax=vars[n1]; }
    }
    if (ineq == "<=" && vars[n2] <= w2) {
      if (op1 == "inc") {vars[n1] += w1; if(vars[n1]>actmax) actmax=vars[n1]; }
      else if (op1 == "dec") {vars[n1] -= w1; if(vars[n1]>actmax) actmax=vars[n1]; }
    }
    if (ineq == "==" && vars[n2] == w2) {
      if (op1 == "inc") {vars[n1] += w1; if(vars[n1]>actmax) actmax=vars[n1]; }
      else if (op1 == "dec") {vars[n1] -= w1; if(vars[n1]>actmax) actmax=vars[n1]; }
    }
    if (ineq == "!=" && vars[n2] != w2) {
      if (op1 == "inc") {vars[n1] += w1; if(vars[n1]>actmax) actmax=vars[n1]; }
      else if (op1 == "dec") {vars[n1] -= w1; if(vars[n1]>actmax) actmax=vars[n1]; }
    }
  }
  auto max = std::max_element(vars.begin(), vars.end(), [] (const decltype(vars)::value_type &a1, decltype(vars)::value_type const &a2) {
      return a1.second < a2.second;
      }
      );
  if (part2) os << actmax << std::endl;
  else os << max->second << std::endl;
}
