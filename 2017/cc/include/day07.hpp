//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #07          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <vector>
#include <sstream>
#include <regex>
#include <unordered_map>

struct Node {
  std::string name;
  int weight, total;
  Node *parent;
  std::vector<std::string> children;
  std::vector<Node*> child_nodes;
};

int calc_totals(Node* n) {
  int total = n->weight;
  for (Node* ch : n->child_nodes) {
    total += calc_totals(ch);
  }
  n->total = total;
  return total;
}

// This seems overly complex
template<typename It, typename Pred>
It diff(It begin, It end, Pred p) {
  if(begin == end)
    return end;
  int seen = 1;
  auto child1 = *begin;
  auto pos = end;
  for ( auto it = std::next(begin); it != end; ++it)
    if(p(child1, *it))
      ++seen;
    else
      pos = it;
  return (seen == 1) ? begin : pos;
}

Node* find_offweight(Node *n) {
  auto eq = [](Node* lhs, Node* rhs){
    return lhs->total == rhs->total;
  };
  auto odd = diff(std::begin(n->child_nodes), std::end(n->child_nodes), eq);
  if (odd != std::end(n->child_nodes))
    return find_offweight(*odd);
  return n;
}


template<>
template<bool part2>
void day<7>::solve(std::istream& is, std::ostream& os)
{
  std::string name;
  int weight;
  std::vector<std::string> children;
  static std::regex re{"[^a-z0-9]+", std::regex::optimize | std::regex::extended};
  std::unordered_map<std::string, Node> nodes;
  for(std::string line; std::getline(is, line); ) {
    line = std::regex_replace(line, re, " ");
    std::stringstream iss{line};
    iss >> name >> weight;
    std::vector<std::string> children{std::istream_iterator<std::string>{iss}, {}};
    nodes[name] = Node{name, weight, 0, nullptr, children};
  }
  for (auto it = std::begin(nodes); it != std::end(nodes); ++it) {
    auto & [name, node] = *it;
    for (auto& child_name : node.children) {
      Node* child = &nodes[child_name];
      child->parent  = &node;
      node.child_nodes.push_back(child);
    }
  }
  Node* tree = &std::begin(nodes)->second;
  while(tree->parent)
    tree = tree->parent;
  if (part2) {
    calc_totals(tree);
    Node* offweight = find_offweight(tree);
    int bad_weight = offweight->total;
    int diff{0};
    for (auto sib : offweight->parent->child_nodes) {
      if (sib->total != bad_weight)
        diff = sib->total - bad_weight;
    }
    os << offweight->weight + diff << std::endl;
  }
  else {
    os << tree->name << std::endl;
  }
}
