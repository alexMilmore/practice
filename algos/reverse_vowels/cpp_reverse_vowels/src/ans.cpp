#include <unordered_set>
#include <iostream>
#include "ans.hpp"

std::string Solution::reverseVowels(std::string s) {
  int i{0};
  int j{s.size()-1};
  std::unordered_set<char> vowels(
      {'a','e','i','o','u','A','E','I','O','U'});

  while (i < j) {
    while (i < j && !vowels.count(s[i])) {i++;}
    while (i < j && !vowels.count(s[j])) {j--;}
    std::cout << i << ", " << j << ", " <<
      vowels.count(s[i]) << ", " << vowels.count(s[j]) << std::endl;
    std::swap(s[i], s[j]);
    i++;
    j--;
  }
  return s;
}
