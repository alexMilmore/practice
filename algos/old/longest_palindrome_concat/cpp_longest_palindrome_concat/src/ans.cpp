#include "ans.hpp"
#include <iostream>

int Solution::longestPalindrome(std::vector<std::string>& words) {
  int letter_matrix[26][26] = {};
  int counter{0};
  /* for (int i{0}; i < 26; i++) { */
  /*   for (int j{0}; j < 26; j++) { */
  /*     letter_matrix[i][j] = 0; */
  /*   } */
  /* } */

  for (auto word : words) {
    int ascii_first = int(word[0]) - int('a');
    int ascii_second = int(word[1]) - int('a');

    // palindrome pair here
    if (letter_matrix[ascii_second][ascii_first] > 0) {
      letter_matrix[ascii_second][ascii_first] -= 1;
      counter += 4;
    }
    else {
      letter_matrix[ascii_first][ascii_second] += 1;
    }
  }
  // check if double letter (eg: "gg")
  for (int i{0}; i < 26; i++) {
    if (letter_matrix[i][i] != 0) { counter += 2; break; }
  }
  return counter;
}
