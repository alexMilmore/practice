#include "ans.hpp"

bool Solution::isUgly(int n) {
  if (n == 0) { return false; }
  int divisors[3] = {2,3,5};
  for (auto divisor : divisors) {
    while (n % divisor == 0) { n /= divisor; }
  }
  return n == 1;
}

