import unittest

class Solution(object):
    def reverseVowels(self, s):
        """
        :type s: str
        :rtype: str
        """
        chars = list(s)
        vowels = ['a','e','i','o','u', 'A','E','I','O','U']
        # s_vowels = list(filter(lambda x: x[0] in vowels, enumerate(chars)))
        s_vowels = list(filter(lambda x: x[1] in vowels, enumerate(chars)))
        num_vowels = len(s_vowels)
        for i in range(0, num_vowels):
            reverse_i = num_vowels-1-i
            chars[s_vowels[i][0]] = s_vowels[num_vowels-1-i][1]
        return "".join(chars)

class TestSolution(unittest.TestCase):
    # Input: s = "hello"
    # Output: "holle"
    def test_1(self):
        sol = Solution()
        ans = sol.reverseVowels("hello")
        self.assertEqual(ans, "holle")

    # Input: s = "leetcode"
    # Output: "leotcede"
    def test_2(self):
        sol = Solution()
        ans = sol.reverseVowels("leetcode")
        self.assertEqual(ans, "leotcede")
