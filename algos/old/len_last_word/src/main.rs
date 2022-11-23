struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut counter: i32 = 0;
        let mut found_1st_word = false;
        for letter in s.chars().rev() {
            match letter {
                ' ' => if found_1st_word {break},
                _   => {
                    counter += 1;
                    found_1st_word = true;
                },
            }
        }
        counter
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let testcase = "Hello World".to_string();
        let expected = 5;
        
        assert_eq!(
            Solution::length_of_last_word(testcase),
            expected);
    }

    #[test]
    fn test_2() {
        let testcase = "   fly me     to    the moon   ".to_string();
        let expected = 4;
        
        assert_eq!(
            Solution::length_of_last_word(testcase),
            expected);
    }

    #[test]
    fn test_3() {
        let testcase = "luffy is still joyboy".to_string();
        let expected = 6;
        
        assert_eq!(
            Solution::length_of_last_word(testcase),
            expected);
    }

    #[test]
    fn test_4() {
        let testcase = "       ".to_string();
        let expected = 0;
        
        assert_eq!(
            Solution::length_of_last_word(testcase),
            expected);
    }
}
