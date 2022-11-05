#include "ans.hpp"
#include <iostream>

bool Solution::exist(std::vector<std::vector<char>>& board, std::string word) {
  // simple checks
  size_t x_len = board.size();
  if (x_len == 0) { return false; }
  size_t y_len = board[0].size();
  if (word.size() == 0) { return true; }
  if (x_len*y_len < word.size()) { return false; }

  // check if letters in board
  char letter_list[128]{};
  for (size_t i{0}; i < x_len; i++) {
    for (size_t j{0}; j < y_len; j++) {
      letter_list[size_t(board[i][j])]++;
    }
  }
  for (int i{0}; i < word.size(); i++) {
    size_t index = word[i];
    if (letter_list[index] > 0) { letter_list[index]--; } else { return false; }
  }

  // search
  for (size_t i{0}; i < x_len; i++) {
    for (size_t j{0}; j < y_len; j++) {
      if (dfs(board, i, j, word, 0)) { return true; }
    }
  }
  return false;
}

bool dfs(std::vector<std::vector<char>>& board, size_t i, size_t j,
std::string word, size_t word_index) {
  if (word[word_index] != board[i][j]) { return false; }
  word_index += 1;
  if (word_index > word.size()-1) { return true; }

  // check directions
  bool check = false;
  char temp = board[i][j];
  board[i][j] = '\n'; // prevent revisiting same char (faster than keeping an unordered_map)
  if (i > 0) { check |= dfs(board, i-1, j, word, word_index); }
  if (j > 0) { check |= dfs(board, i, j-1, word, word_index); }
  if (i < board.size()-1)    { check |= dfs(board, i+1, j, word, word_index); }
  if (j < board[0].size()-1) { check |= dfs(board, i, j+1, word, word_index); }
  board[i][j] = temp;
  return check;
}
