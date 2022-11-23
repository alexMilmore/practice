use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut char_map = HashSet::<String>::new();
        let mut output: Vec::<String>::new();
        for word in words {
            for word_char in word.chars() {
                match char_map.insert(word_char.to_string()) {
                    true => output.push(word_char.to_string()),
                    false => (),
                }
            }
        }
        output
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test1() {
        let testcase = vec!("bella".to_string(),
                            "label".to_string(),
                            "roller".to_string());
        let expected = vec!("e", "l", "l");
    }
}
