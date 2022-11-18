#include <gtest/gtest.h>
#include "ans.hpp"

// Input: n = 6
// Output: true
TEST(A, TEST_1) {
  Solution sol;
  bool ans = sol.isUgly(6);
  EXPECT_EQ(ans, true);
}

// Input: n = 1
// Output: true
TEST(A, TEST_2) {
  Solution sol;
  bool ans = sol.isUgly(1);
  EXPECT_EQ(ans, true);
}

// Input: n = 14
// Output: false
TEST(A, TEST_3) {
  Solution sol;
  bool ans = sol.isUgly(14);
  EXPECT_EQ(ans, false);
}

// Input: n = 0
// Output: false
TEST(A, TEST_4) {
  Solution sol;
  bool ans = sol.isUgly(0);
  EXPECT_EQ(ans, false);
}

// Input: n = 8
// Output: true
TEST(A, TEST_5) {
  Solution sol;
  bool ans = sol.isUgly(8);
  EXPECT_EQ(ans, true);
}
