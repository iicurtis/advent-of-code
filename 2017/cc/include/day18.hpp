//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #18          //
//////////////////////

// A general description of the goal

#include <map>
#include <locale>
#include <queue>

#include "solution.hpp"

bool execute(int &i, std::vector<std::string> &commands, std::vector<long int> &reg, std::queue<long int> &rcv, std::queue<long int> &snd, int &sent) {
  if ((i >= commands.size()) || (i < 0))
    return true;

  std::string op, in_a, in_b;
  long int a, b;
  std::istringstream command(commands[i]);
  command >> op >> in_a;
  if (op == "snd" || op == "rcv")
    in_b = in_a;
  else
    command >> in_b;

  a = in_a[0] - 'a';
  b = (in_b[0] >= 'a' && in_b[0] <= 'z') ? reg[in_b[0] - 'a'] : std::stoi(in_b);

  // std::cout << i << ": ";

  if (op == "snd") {
    // std::cout << "snd " << b << '\n';
    snd.push(b);
    sent++;
  } else if (op == "set") {
    // std::cout << "set " << a << " " << b << " old:" << reg[a] << '\n';
    reg[a] = b;
  }
  else if (op == "add") {
    // std::cout << "add " << a << " " << b << " old:" << reg[a] << '\n';
    reg[a] += b;
  }
  else if (op == "mul") {
    // std::cout << "add " << a << " " << b << " old:" << reg[a] << '\n';
    reg[a] *= b;
  }
  else if (op == "mod") {
    // std::cout << "add " << a << " " << b << " old:" << reg[a] << '\n';
    reg[a] %= b;
  }
  else if (op == "rcv") {
    if (rcv.empty()) {
      // std::cout << "rcv empty" << '\n';
      return true;
    }
    else {
      // std::cout << "rcv " << a << " " << rcv.front() << " old:" << reg[a] << '\n';
      reg[a] = rcv.front();
      rcv.pop();
    }
  } else if (op == "jgz") {
    a = (in_a[0] >= 'a' && in_a[0] <= 'z') ? reg[in_a[0] - 'a'] : std::stoi(in_a);
    // std::cout << "jgz " << a << " " << b << '\n';
    if (a > 0)
      i += b - 1;
  }
  i++;
  return false;
}

template<>
template<bool part2>
void day<18>::solve(std::istream& is, std::ostream& os)
{
  std::vector<long int> reg_a(26,0);
  std::vector<long int> reg_b(26,0);
  std::queue<long int> queue_a;
  std::queue<long int> queue_b;
  int sent_a{0}, sent_b{0};
  int instr_a{0}, instr_b{0};
  std::vector<std::string> commands;
  for (std::string line; std::getline(is, line);)
    commands.emplace_back(line);

  reg_b['p' - 'a'] = 1;

  while(true) {
    bool waiting_a = execute(instr_a, commands, reg_a, queue_a, queue_b, sent_a);
    bool waiting_b = execute(instr_b, commands, reg_b, queue_b, queue_a, sent_b);
    if (waiting_a && waiting_b) break;
  }

  os << sent_b << '\n';
}
