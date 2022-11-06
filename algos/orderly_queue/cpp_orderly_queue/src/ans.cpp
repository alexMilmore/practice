#include <algorithm>
#include <string>
#include "ans.hpp"

std::string Solution::orderlyQueue(std::string s, int k) {
  if (k == 1) {
    size_t size = s.size();
    std::string mem;
    std::string min = s;
    std::string repeatstring = s+s;
    for (size_t i{0}; i < size; i++) {
      mem = repeatstring.substr(i,size);
      if (mem < s) { s = mem; }
    }
    return s;
  }
  std::sort(s.begin(), s.end());
  return s;
}
