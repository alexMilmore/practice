#ifndef ANS_HPP
#define ANS_HPP

#include <vector>
#include <string>

class Solution {
public:
    bool exist(std::vector<std::vector<char>>& board, std::string word);
};

bool dfs(std::vector<std::vector<char>>& board, size_t i, size_t j,
std::string word, size_t word_index);

#endif // ANS_HPP
