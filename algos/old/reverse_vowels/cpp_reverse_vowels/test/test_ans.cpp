#include <gtest/gtest.h>
#include "ans.hpp"

/* Input: s = "hello" */
/* Output: "holle" */
TEST(A, TEST1) {
  Solution sol;
  std::string input = "hello";
  std::string ans = sol.reverseVowels(input);
  EXPECT_EQ(ans, "holle");
}

/* Input: s = "leetcode" */
/* Output: "leotcede" */
TEST(A, TEST2) {
  Solution sol;
  std::string ans = sol.reverseVowels("leetcode");
  EXPECT_EQ(ans, "leotcede");
}
