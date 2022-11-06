import unittest

class Solution:
    def orderlyQueue(self, s: str, k: int) -> str:
        if k == 1:
            orders = []
            for i in range(0, len(s)):
                orders.append(s[i:]+s[:i])
            print(orders)
            return sorted(orders)[0]
        return ''.join(sorted(s))

class TestAns(unittest.TestCase):
    # Input: s = "cba", k = 1
    # Output: "acb"
    def test_1(self):
        sol = Solution()
        ans = sol.orderlyQueue("cba", 1);
        self.assertEqual(ans, "acb")

    # Input: s = "baaca", k = 3
    # Output: "aaabc"
    def test_2(self):
        sol = Solution()
        ans = sol.orderlyQueue("baaca", 3);
        self.assertEqual(ans, "aaabc")
