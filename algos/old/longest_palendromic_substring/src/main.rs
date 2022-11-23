struct Solution;

fn get_palindrome(chars: &Vec<char>, mut window: (usize, usize)) -> ((usize, usize), usize) {
    if chars[window.0] != chars[window.1] {
        return ((window.0, window.0), 0)
    }
    loop {
        if window.0 <= 0 {break;}
        if window.1 >= chars.len()-1 {break;}
        if chars[window.0-1] != chars[window.1+1] { break; }
        window.0 -= 1;
        window.1 += 1;
    }
    (window, window.1-window.0)
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut best_len = 0;
        let mut best_window = (0, 0);
        for center in 0..chars.len() {
            let (win, len) = get_palindrome(&chars, (center, center));
            if len > best_len {
                best_len = len;
                best_window = win;
            }
            if center != chars.len()-1 {
                let (win, len) = get_palindrome(&chars, (center, center+1));
                if len > best_len {
                    best_len = len;
                    best_window = win;
                }
            }
        }

        chars[best_window.0..best_window.1+1].iter().collect::<String>()
    }
}

fn main() {
    let pal = Solution::longest_palindrome("billllib".to_string());
    println!("Hello, world! {}", pal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ans = Solution::longest_palindrome("babad".into());
        assert_eq!(ans, "bab".to_string());
    }

    #[test]
    fn test_2() {
        let ans = Solution::longest_palindrome("babcbad".into());
        assert_eq!(ans, "abcba".to_string());
    }

    #[test]
    fn test_3() {
        let ans = Solution::longest_palindrome("babaabad".into());
        assert_eq!(ans, "abaaba".to_string());
    }
}
