struct Solution;

#[derive(Debug, PartialEq, Eq)]
struct Token {
    letter: char,
    repeat: usize,
}

impl Token {
    fn new(letter: char, repeat: usize) -> Self {
        Self {letter, repeat}
    }

    fn len(&self) -> usize {
        self.str_len(self.repeat)
    }

    fn str_len(&self, repeat: usize) -> usize {
        let mut count = 1; // include letter
        let mut val = self.repeat;
        while val > 0 as usize {
            count += 1;
            val /= 10;
        }
        count
    }

    fn reduction(&self, num_delete: usize) -> Result<usize, ()> {
        if num_delete >= self.repeat {
            return Err(())
        }
        Ok(self.len() - self.str_len(self.repeat - num_delete))
    }
}

struct Reduction {
    index: usize,
    num: usize,
    reduction: usize
}

impl Solution {
    fn get_tokens(s: String) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();
        let mut last_letter = None;
        let mut count = 0;
        for letter in s.chars() {
            match last_letter {
                None => count += 1,
                Some(last) => {
                    if letter != last {
                        tokens.push(Token::new(last, count));
                        count = 1;
                    }
                    else {count += 1;}
                }
            }
            last_letter = Some(letter);
        }
        if last_letter.is_some() {
            tokens.push(Token::new(last_letter.unwrap(), count));
        }
        tokens
    }

    fn get_min_tokens(tokens: Vec<Token>) {
        let min_token = |index: usize, prev: Token, end: usize| -> usize {
            let t = &tokens[index];
            return 0;
        };
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let tokens = Self::get_tokens(s);
        
        0
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn token_test_1() {
        let ans = Solution::get_tokens("aaabcccd".to_string());
        assert_eq!(
            ans,
            vec!(Token::new('a',3), Token::new('b',1), Token::new('c',3), Token::new('d',1))
        );
    }

    #[test]
    fn len_test_1() {
        let token = Token::new('a', 4);
        assert_eq!(token.len(), 2);
    }

    #[test]
    fn len_test_2() {
        let token = Token::new('a', 432);
        assert_eq!(token.len(), 4);
    }

    // #[test]
    // fn test_1() {
    //     let ans = Solution::get_length_of_optimal_compression(
    //         "aaabcccd".to_string(), 2);
    //     assert_eq!(ans, 4);
    // }

    // #[test]
    // fn test_2() {
    //     let ans = Solution::get_length_of_optimal_compression(
    //         "aabbaa".to_string(), 2);
    //     assert_eq!(ans, 2);
    // }

    // #[test]
    // fn test_3() {
    //     let ans = Solution::get_length_of_optimal_compression(
    //         "aaaaaaaaaaa".to_string(), 0);
    //     assert_eq!(ans, 3);
    // }
}
