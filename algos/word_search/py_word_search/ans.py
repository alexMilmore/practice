import unittest
from typing import List

def dfs(board: List[List[str]], i: int, j: int, word: str, word_index: int):
    if board[i][j] != word[word_index]:
        return False
    word_index += 1
    if word_index > len(word)-1:
        return True

    temp = board[i][j]
    board[i][j] = 0
    check = False
    if i > 0:
        check |= dfs(board, i-1, j, word, word_index)
    if j > 0:
        check |= dfs(board, i, j-1, word, word_index)
    if i < len(board)-1:
        check |= dfs(board, i+1, j, word, word_index)
    if j < len(board[0])-1:
        check |= dfs(board, i, j+1, word, word_index)
    board[i][j] = temp
    return check

class Solution(object):
    def exist(self, board, word):
        """
        :type board: List[List[str]]
        :type word: str
        :rtype: bool
        """
        # check possible
        letters = {}
        for i in range(0, len(board)):
            for j in range(0, len(board[0])):
                if board[i][j] in letters:
                    letters[board[i][j]] += 1
                else:
                    letters[board[i][j]] = 1
        for letter in word:
            if letter not in letters or letters[letter] == 0:
                return False
            else:
                letters[letter] -= 1

        # search
        for i in range(0, len(board)):
            for j in range(0, len(board[0])):
                if (dfs(board, i, j, word, 0)):
                    return True
        return False

class TestAns(unittest.TestCase):
    # Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
    # Output: true
    def test_1(self):
        sol = Solution()
        ans = sol.exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCCED")
        self.assertEqual(ans, True)

    # Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
    # Output: true
    def test_2(self):
        sol = Solution()
        ans = sol.exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "SEE")
        self.assertEqual(ans, True)

    # Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
    # Output: false
    def test_3(self):
        sol = Solution()
        ans = sol.exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCB")
        self.assertEqual(ans, False)

    # Input: board = [["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","B"],["A","A","A","A","B","A"]]
    #        word = "AAAAAAAAAAAAABB"
    # Output: false
    def test_4(self):
        sol = Solution()
        ans = sol.exist([["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","B"],["A","A","A","A","B","A"]], "AAAAAAAAAAAAABB")
        self.assertEqual(ans, False)
