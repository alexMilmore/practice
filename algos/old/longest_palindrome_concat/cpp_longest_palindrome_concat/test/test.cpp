#include <gtest/gtest.h>
#include "ans.hpp"

/* Input: words = ["lc","cl","gg"] */
/* Output: 6 */
TEST(A, TEST1) {
  Solution ans_obj;
  std::vector<std::string> input = {"lc","cl","gg"};
  int ans = ans_obj.longestPalindrome(input);
  ASSERT_EQ(ans, 6);
}

/* Input: words = ["ab","ty","yt","lc","cl","ab"] */
/* Output: 8 */
TEST(A, TEST2) {
  Solution ans_obj;
  std::vector<std::string> input =
    {"ab","ty","yt","lc","cl","ab"};
  int ans = ans_obj.longestPalindrome(input);
  ASSERT_EQ(ans, 8);
}

/* Input: words = ["cc","ll","xx"] */
/* Output: 2 */
TEST(A, TEST3) {
  Solution ans_obj;
  std::vector<std::string> input = {"cc","ll","xx"};
  int ans = ans_obj.longestPalindrome(input);
  ASSERT_EQ(ans, 2);
}
