struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() == 0 || palindrome.len() == 1 {return "".to_string()}

        let mut is_broken = false;
        let mut letters: Vec<char> = palindrome.chars().collect();
        for i in 0..(letters.len()/2) {
            if letters[i] != 'a' {
                letters[i] = 'a';
                is_broken = true;
                break;
            }
        }
        if !is_broken {
            *letters.last_mut().unwrap() = 'b';
        }
        letters.iter().collect()
    }
}

fn main() {
    println!("Hello, world!");
    Solution::break_palindrome("bill".to_string());
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test1() {
        let ans = Solution::break_palindrome("abccba".to_string());
        assert_eq!(ans, "aaccba".to_string());
    }

    #[test]
    fn test2() {
        let ans = Solution::break_palindrome("aaabbaaa".to_string());
        assert_eq!(ans, "aaaabaaa".to_string());
    }

    #[test]
    fn test3() {
        let ans = Solution::break_palindrome("aabbbaa".to_string());
        assert_eq!(ans, "aaabbaa".to_string());
    }

    #[test]
    fn test4() {
        let ans = Solution::break_palindrome("a".to_string());
        assert_eq!(ans, "".to_string());
    }

    #[test]
    fn test5() {
        let ans = Solution::break_palindrome("aaabaaa".to_string());
        assert_eq!(ans, "aaabaab".to_string());
    }
}
