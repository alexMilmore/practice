struct Solution;

use std::collections::HashMap;

#[derive(Debug)]
struct PairCounter {
    data: HashMap<String, usize>
}

impl PairCounter {
    fn new() -> Self { Self {data: HashMap::<String, usize>::new()} }

    fn contains(&self, word: &String) -> bool {
        self.data.contains_key(word)
    }

    fn add(&mut self, word: &String) {
        if self.data.contains_key(word) { *self.data.get_mut(word).unwrap() += 1; }
        else { self.data.insert((*word).clone(), 1); }
    }

    fn remove(&mut self, word: &String) {
        if self.data[word] > 1 { *self.data.get_mut(word).unwrap() -= 1; }
        else { self.data.remove(word); }
    }
}

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut pair_candidates = PairCounter::new();
        let mut counter = 0;
        // match palindrome pair_candidates
        for item in words {
            let reverse = item.chars().rev().collect::<String>();
            if pair_candidates.contains(&reverse) {
                counter += 4;
                pair_candidates.remove(&reverse);
            }
            else { pair_candidates.add(&item); }
        }

        // check for double letters (eg: "gg")
        for item in pair_candidates.data.keys() {
            if item.chars().nth(0) == item.chars().nth(1) {
                counter += 2;
                break
            }
        }
        counter
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Input: words = ["lc","cl","gg"]
    // Output: 6
    #[test]
    fn test_1() {
        let ans = Solution::longest_palindrome(
            vec!["lc".to_string(),"cl".to_string(),"gg".to_string()]);
        assert_eq!(ans, 6);
    }

    // Input: words = ["ab","ty","yt","lc","cl","ab"]
    // Output: 8
    #[test]
    fn test_2() {
        let ans = Solution::longest_palindrome(
            vec!["ab".to_string(),"ty".to_string(),"yt".to_string(),
            "lc".to_string(),"cl".to_string(),"ab".to_string()]);
        assert_eq!(ans, 8);
    }

    // Input: words = ["cc","ll","xx"]
    // Output: 2
    #[test]
    fn test_3() {
        let ans = Solution::longest_palindrome(
            vec!["cc".to_string(),"ll".to_string(),"xx".to_string()]);
        assert_eq!(ans, 2);
    }

    // Input: words = ["qo","fo","fq","qf","fo","ff","qq","qf","of","of","oo","of","of","qf","qf","of"]
    // Output: 14
    #[test]
    fn test_4() {
        let input = vec!["qo","fo","fq","qf","fo","ff","qq","qf","of","of","oo","of","of","qf","qf","of"]
            .iter().map(|a| a.to_string()).collect();
        let ans = Solution::longest_palindrome(input);
        assert_eq!(ans, 14);
    }
}
