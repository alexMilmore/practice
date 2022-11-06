struct Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        match k {
            1 => {
                let mut strings = (0..s.len()).map(|i| {
                    let mut new_string = String::new();
                    new_string += &s[i..];
                    new_string += &s[..i];
                    new_string
                }).collect::<Vec<String>>();
                strings.sort();
                strings.remove(0)
            },
            _ => {
                let mut out = s.chars().collect::<Vec<char>>();
                out.sort();
                out.iter().collect::<String>()
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Input: s = "cba", k = 1
    // Output: "acb"
    #[test]
    fn test_1() {
        let ans = Solution::orderly_queue("cba".into(), 1);
        assert_eq!(&ans, "acb");
    }

    // Input: s = "baaca", k = 3
    // Output: "aaabc"
    #[test]
    fn test_2() {
        let ans = Solution::orderly_queue("baaca".into(), 3);
        assert_eq!(&ans, "aaabc");
    }
}
