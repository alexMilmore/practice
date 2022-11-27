#include <gtest/gtest.h>

#include "ans.hpp"

// Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
// Output: 120
TEST(A, TEST_1) {
  Solution sol;
  std::vector<int> starts{1,2,3,3};
  std::vector<int> ends{3,4,5,6};
  std::vector<int> profits{50,10,40,70};
  
  int ans = sol.jobScheduling(
      starts, ends, profits
  );
  EXPECT_EQ(ans, 120);
}

TEST(A, TEST_2) {
  Solution sol;
  std::vector<int> starts{43,13,36,31,40,5,47,13,28,16,2,11};
  std::vector<int> ends{44,22,41,41,47,13,48,35,48,26,21,39};
  std::vector<int> profits{8,20,3,19,16,8,11,13,2,15,1,1};
  
  int ans = sol.jobScheduling(
      starts, ends, profits
  );
  EXPECT_EQ(ans, 66);
}
