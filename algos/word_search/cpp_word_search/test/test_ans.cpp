#include <gtest/gtest.h>
#include "ans.hpp"

// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
// Output: true
TEST(A, test_1) {
  Solution obj = Solution();
  std::vector<std::vector<char>> in{{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
  EXPECT_EQ(obj.exist(in, "ABCCED"), true);
}

// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
// Output: true
TEST(A, test_2) {
  Solution obj = Solution();
  std::vector<std::vector<char>> in{{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
  EXPECT_EQ(obj.exist(in, "SEE"), true);
}

// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
// Output: false
TEST(A, test_3) {
  Solution obj = Solution();
  std::vector<std::vector<char>> in{{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
  EXPECT_EQ(obj.exist(in, "ABCB"), false);
}

TEST(A, test_4) {
  Solution obj = Solution();
  std::vector<std::vector<char>> in{{'A','A','A','A'},{'A','A','A','A'},{'A','A','A','A'}};
  EXPECT_EQ(obj.exist(in, "AAAAAAB"), false);
}
