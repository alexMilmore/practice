#include <gtest/gtest.h>
#include <solution.hpp>

/* TEST(PartyPeople, Test1) { */
/*   Solution sol{}; */
/*   std::vector<std::string> bank = {"AAAACCCC","AAACCCCC","AACCCCCC"}; */
/*   int ans = sol.minMutation("AAAAACCC", "AACCCCCC", bank); */
/*   ASSERT_EQ(ans, 3); */
/* } */

/* /1* Input: start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"] *1/ */
/* /1* Output: 2 *1/ */
/* TEST(PartyPeople, Test2) { */
/*   Solution sol{}; */
/*   std::vector<std::string> bank = {"AACCGGTA","AACCGCTA","AAACGGTA"}; */
/*   int ans = sol.minMutation("AACCGGTT", "AAACGGTA", bank); */
/*   ASSERT_EQ(ans, 2); */
/* } */

/* /1* Input: start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"] *1/ */
/* /1* Output: 1 *1/ */
/* TEST(PartyPeople, Test3) { */
/*   Solution sol{}; */
/*   std::vector<std::string> bank = {"AACCGGTA"}; */
/*   int ans = sol.minMutation("AACCGGTT", "AACCGGTA", bank); */
/*   ASSERT_EQ(ans, 1); */
/* } */

/* "AAAAAAAA" */
/* "CCCCCCCC" */
/* ["AAAAAAAA","AAAAAAAC","AAAAAACC","AAAAACCC","AAAACCCC","AACACCCC","ACCACCCC","ACCCCCCC","CCCCCCCA"] */
TEST(PartyPeople, Test4) {
  Solution sol{};
  std::vector<std::string> bank = {"AAAAAAAA","AAAAAAAC","AAAAAACC","AAAAACCC","AAAACCCC","AACACCCC","ACCACCCC","ACCCCCCC","CCCCCCCA"};
  int ans = sol.minMutation("AAAAAAAA", "CCCCCCCC", bank);
  ASSERT_EQ(ans, -1);
}
