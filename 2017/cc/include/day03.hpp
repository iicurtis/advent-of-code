//////////////////////
// Isaac Curtis     //
// Advent of Code   //
// Day #03          //
//////////////////////

// A general description of the goal

#include "solution.hpp"
#include <cmath>
#include <unordered_map>
#include <array>

/* FROM http://oeis.org/A141481
 * m=5;
 * h=2*m-1;
 * A=matrix(h, h);
 * print1(A[m, m]=1, ", ");
 * T=[[1, 0], [1, -1], [0, -1], [ -1, -1], [ -1, 0], [ -1, 1], [0, 1], [1, 1]];
 * for(n=1, (h-2)^2-1, g=sqrtint(n);
 * r=(g+g%2)\2;
 * q=4*r^2;
 * d=n-q;
 * if(n<=q-2*r, j=d+3*r; k=r, if(n<=q, j=r; k=-d-r, if(n<=q+2*r, j=r-d; k=-r, j=-r; k=d-3*r)));
 * j=j+m;
 * k=k+m;
 * s=0;
 * for(c=1, 8, v=[j, k]; v+=T[c]; s=s+A[v[1], v[2]]);
 * A[j, k]=s; print1(s, ", "))
 */

struct Point {
  int x{0}, y{0};
  operator std::uint64_t() const {
    auto as_u32 = [](int a) { return *reinterpret_cast<std::uint32_t*>(&a); };
    return static_cast<std::uint64_t>(as_u32(x)) << 32 | as_u32(y);
  }
  Point& operator+=(Point const& p) {
    x += p.x;
    y += p.y;
    return *this;
  }
  Point operator+(Point const& p) const {
    return Point{*this} += p;
  }
};

template<>
template<bool part2>
void day<3>::solve(std::istream& is, std::ostream& os)
{
  auto findManhatten = [](int x)
  {
      int n = std::floor((std::sqrt(x) + 1) / 2);
      return n + std::abs( n - (x - (1 + 4*n*(n-1))) % (2*n));
  };

  int x;
  is >> x;
  if (!part2) {
    os << findManhatten(x) << std::endl;
  }
  else {
      static std::array<Point, 8> const neighbors{{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}}};
      int limit{1}, step{0};
      bool sawTwo{true};
      Point loc{0, 0}, dir{1, 0};
      std::unordered_map<std::uint64_t, int> vals;
      vals[loc] = 1;
      while (vals[loc] <= x) {
          loc += dir;
          if (++step == limit) {
              step = 0;
              if ((sawTwo = !sawTwo)) {
                  limit++;
                }
              dir = Point{-dir.y, dir.x};
            }
          int sum{0};
          for (auto const& n : neighbors) {
              sum += vals[loc + n];
            }
          vals[loc] = sum;
        }
      os << vals[loc] << '\n';
  }
}
