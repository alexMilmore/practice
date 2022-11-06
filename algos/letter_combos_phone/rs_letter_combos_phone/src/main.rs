struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits == "" {return vec!()}

        let mut cache = HashMap::<usize, String>::from([
            (1, "".into()),
            (2, "a,b,c".into()),
            (3, "d,e,f".into()),
            (4, "g,h,i".into()),
            (5, "j,k,l".into()),
            (6, "m,n,o".into()),
            (7, "p,q,r,s".into()),
            (8, "t,u,v".into()),
            (9, "w,x,y,z".into()),
        ]);

        get_digit_permutations(digits.parse::<usize>().unwrap(), &mut cache)
            .split(',').map(|x| x.to_string()).collect::<Vec<String>>()
    }
}

fn combine(a: &str, b: &str) -> String {
    let mut new_string = String::new();
    for word_a in a.split(',') {
        for word_b in b.split(',') {
            new_string += word_a;
            new_string += word_b;
            new_string += ",";
        }
    }
    // remove trailing comma
    match new_string.pop() {
        Some(',') => (),
        None => (),
        Some(letter) => new_string.push(letter),
    }
    new_string
}

fn get_digit_permutations(num: usize, cache: &mut HashMap<usize, String>) -> String {
    match cache.get(&num) {
        Some(val) => {return val.clone();},
        None => {
            let out = combine(
                &get_digit_permutations(num/10, cache),
                &get_digit_permutations(num%10, cache)
            );
            cache.insert(num, out.clone());
            return out;
        },

    }
}

fn main() {
    println!("Hello, world! {:?}", Solution::letter_combinations("".into()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Input: digits = "23"
    // Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
    #[test]
    fn test_1() {
        let ans = Solution::letter_combinations("23".into());
        let expected = vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(ans, expected);
    }

    // Input: digits = ""
    // Output: []
    #[test]
    fn test_2() {
        let ans = Solution::letter_combinations("".into());
        let expected: Vec<String> = vec![];
        assert_eq!(ans, expected);
    }

    // Input: digits = "2"
    // Output: ["a","b","c"]
    #[test]
    fn test_3() {
        let ans = Solution::letter_combinations("2".into());
        let expected: Vec<String> = vec!["a","b","c"]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(ans, expected);
    }
}
