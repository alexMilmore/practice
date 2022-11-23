use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let letters: Vec<char> = s.chars().collect();
        if letters.len() == 0 {return 0;}
        else if letters.len() == 1 {return 1;}

        let mut window_start = 0;
        let mut window_end = 1; // window does not include this index
        let mut window_len: usize;
        let mut next_val: char;
        let mut letters_map = HashMap::<char, usize>::new();

        let mut max_letters = 0;

        for (index, letter) in letters.iter().enumerate() {
            match letters_map.get(letter) {
                Some(val) => {
                    if *val > window_start { // duplicate letter in window
                        window_len = index - window_start; 
                        if window_len > max_letters {max_letters = window_len;}
                        window_start = *val;
                    }
                },
                None => (),
            }

            letters_map.insert(*letter, index);
            println!("{:?} {:?}", letters, &letters[window_start..index]);
        }
        
        max_letters as i32
    }

    pub fn length_of_longest_substring_slow(s: String) -> i32 {
        let letters: Vec<char> = s.chars().collect();
        if letters.len() == 0 {return 0;}
        else if letters.len() == 1 {return 1;}

        let mut window_start = 0;
        let mut window_end = 1;
        let mut window_len: usize;
        let mut next_val: char;

        let mut max_letters = 0;

        loop {
            // try expand window
            if window_end < letters.len() {
                next_val = letters[window_end];
                if letters[window_start..window_end].contains(&next_val) {
                    window_start += 1;
                }
                else {
                    window_end += 1;
                }
            }

            // check window
            window_len = window_end - window_start;
            if max_letters < window_len {max_letters = window_len;}

            // exit
            if window_end >= letters.len() {
                break;
            }
        }
        
        max_letters as i32
    }
}

fn main() {
    Solution::length_of_longest_substring("bill-e".to_string());
    println!("Hello, world!");
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test1() {
        let ans = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(ans, 3);
    }

    #[test]
    fn test2() {
        let ans = Solution::length_of_longest_substring("bbbbb".to_string());
        assert_eq!(ans, 1);
    }

    #[test]
    fn test3() {
        let ans = Solution::length_of_longest_substring("pwwkew".to_string());
        assert_eq!(ans, 3);
    }

    #[test]
    fn test4() {
        let ans = Solution::length_of_longest_substring("".to_string());
        assert_eq!(ans, 0);
    }

    #[test]
    fn test5() {
        let ans = Solution::length_of_longest_substring("a".to_string());
        assert_eq!(ans, 1);
    }

    #[test]
    fn test6() {
        let ans = Solution::length_of_longest_substring("au".to_string());
        assert_eq!(ans, 2);
    }

    #[test]
    fn test7() {
        let ans = Solution::length_of_longest_substring("abcdefg".to_string());
        assert_eq!(ans, 7);
    }

    #[test]
    fn test8() {
        let ans = Solution::length_of_longest_substring("aaaabcdefg".to_string());
        assert_eq!(ans, 7);
    }
}
