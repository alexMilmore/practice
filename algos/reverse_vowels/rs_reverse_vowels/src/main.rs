struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = HashSet::from(['a','e','i','o','u','A','E','I','O','U']);
        let mut chars = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = chars.len()-1;
        while i < j {
            while i < j && !vowels.contains(&chars[i]) { i += 1; }
            while i < j && !vowels.contains(&chars[j]) { j -= 1; }
            let temp = chars[i];
            chars[i] = chars[j];
            chars[j] = temp;
            i += 1;
            if j > 0 {j -= 1;}
        }
        chars.iter().collect::<String>()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Input: s = "hello"
    // Output: "holle"
    #[test]
    fn test_1() {
        let ans = Solution::reverse_vowels("hello".to_string());
        assert_eq!(&ans, "holle");
    }

    // Input: s = "leetcode"
    // Output: "leotcede"
    #[test]
    fn test_2() {
        let ans = Solution::reverse_vowels("leetcode".to_string());
        assert_eq!(&ans, "leotcede");
    }

    // Input: s = "leetcode"
    // Output: "leotcede"
    #[test]
    fn test_3() {
        let ans = Solution::reverse_vowels("a.".to_string());
        assert_eq!(&ans, "a.");
    }
}
