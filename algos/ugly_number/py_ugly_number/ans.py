class Solution:
    def isUgly(self, n: int) -> bool:
        if n == 0:
            return False
        for i in [2,3,5]:
            while n % i == 0:
                n /= i
        return n == 1

import unittest
class TestSolution(unittest.TestCase):
    # Input: n = 6
    # Output: true
    def test_1(self):
        sol = Solution();
        ans = sol.isUgly(6)
        self.assertEqual(ans, True)

    # Input: n = 1
    # Output: true
    def test_2(self):
        sol = Solution();
        ans = sol.isUgly(1)
        self.assertEqual(ans, True)

    # Input: n = 14
    # Output: false
    def test_3(self):
        sol = Solution();
        ans = sol.isUgly(14)
        self.assertEqual(ans, False)

    # Input: n = 0
    # Output: false
    def test_4(self):
        sol = Solution();
        ans = sol.isUgly(0)
        self.assertEqual(ans, False)

    # Input: n = 8
    # Output: false
    def test_5(self):
        sol = Solution();
        ans = sol.isUgly(8)
        self.assertEqual(ans, True)
