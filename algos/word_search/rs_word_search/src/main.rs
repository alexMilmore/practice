use std::iter::Peekable;

struct Solution;

fn dfs(board: &mut Vec<Vec<char>>, pos: (usize, usize), mut letters: Peekable<std::str::Chars>) -> bool {
    let (i, j) = pos;
    // assure pos isn't visited twice
    if board[i][j] == '\n' { return false; }

    // check match
    match letters.next() {
        Some(letter) => {
            if letter != board[i][j] { return false }
            if letters.peek().is_none() { return true }
        },
        None => return true,
    }

    // check directions
    let temp = board[i][j];
    board[i][j] = '\n';
    let mut check = false;
    if i > 0                { check |= dfs(board, (i-1, j), letters.clone()); }
    if j > 0                { check |= dfs(board, (i, j-1), letters.clone()); }
    if i < board.len()-1    { check |= dfs(board, (i+1, j), letters.clone()); }
    if j < board[0].len()-1 { check |= dfs(board, (i, j+1), letters.clone()); }
    board[i][j] = temp;

    check
}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        if board.len() == 0 { return false }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if dfs(&mut board, (i, j), word.chars().peekable()) { return true; }
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world! {}", Solution::exist(vec![], "".to_string()));
}

#[cfg(test)]
pub mod test {
    use super::*;

    // Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
    // Output: true
    #[test]
    fn test_1() {
        let input = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        let ans = Solution::exist(input, "ABCCED".to_string());
        assert_eq!(ans, true);
    }

    // Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
    // Output: true
    #[test]
    fn test_2() {
        let input = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        let ans = Solution::exist(input, "SEE".to_string());
        assert_eq!(ans, true);
    }

    // Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
    // Output: false
    #[test]
    fn test_3() {
        let input = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        let ans = Solution::exist(input, "ABCB".to_string());
        assert_eq!(ans, false);
    }

    // Input: board [['a']], word = "a"
    // Output: true
    #[test]
    fn test_4() {
        let input = vec![vec!['a']];
        let ans = Solution::exist(input, "a".to_string());
        assert_eq!(ans, true);
    }

    // Input: board = [["a","b"],["c","d"]], word = "cdba"
    // Output: true
    #[test]
    fn test_5() {
        let input = vec![vec!['a','b'],vec!['c','d']];
        let ans = Solution::exist(input, "cdba".to_string());
        assert_eq!(ans, true);
    }

    // Input: board =
    // [["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","A"],["A","A","A","A","A","A"]],
    // word = "AAAAAAAAAAAAAAB"
    // Output: false
    #[test]
    fn test_6() {
        let input = vec![vec!['A';6];6];
        let ans = Solution::exist(input, "AAAAAAAAAAAAAAB".to_string());
        assert_eq!(ans, false);
    }
}
