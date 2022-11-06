#include <gtest/gtest.h>
#include "ans.hpp"

// Input: s = "cba", k = 1
// Output: "acb"
TEST(A, TEST1) {
  Solution sol;
  ASSERT_EQ(sol.orderlyQueue("cba", 1), "acb");
}

// Input: s = "baaca", k = 3
// Output: "aaabc"
TEST(A, TEST2) {
  Solution sol;
  ASSERT_EQ(sol.orderlyQueue("baaca", 3), "aaabc");
}
