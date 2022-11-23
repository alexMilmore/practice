struct Solution;

impl Solution {
    pub fn get_length_of_optimal_compression_old(s: String, k: i32) -> i32 {
        fn get_permutations(chars: &Vec<char>, i: usize, last: Option<char>, mut accum: (usize, usize), left: usize, out: &mut Vec<usize>) {

            if i == chars.len() {
                out.push(accum.1 + token_compress_len(accum.0));
                return
            }

            // remove
            if left > 0 {get_permutations(chars, i+1, last, accum, left-1, out);}
            // keep (always want to remove max chars in total)
            if left < chars.len() - i{
                if last.is_some() && last.unwrap() != chars[i] {
                    accum = (0, accum.1 + token_compress_len(accum.0));
                }
                accum.0 += 1;
                get_permutations(chars, i+1, Some(chars[i]), accum, left, out);
            }

        }

        fn token_compress_len(repeat: usize) -> usize {
            if repeat == 0 {return 0}
            else if repeat == 1 {return 1}
            else if repeat < 10 {return 2}
            else if repeat < 100 {return 3}
            else {return 4}
        }

        let mut out = Vec::<usize>::new();
        get_permutations(&s.chars().collect(), 0, None, (0, 0), k as usize, &mut out);

        *out.iter().min().unwrap() as i32
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        fn get_min(chars: &Vec<char>, i: usize, last: Option<char>, mut accum: (usize, usize), left: usize, out: &mut Vec<usize>) {
            if i == chars.len() {
                out.push(accum.1 + token_compress_len(accum.0));
                return
            }

            // remove
            if left > 0 {get_min(chars, i+1, last, accum, left-1, out);}
            // keep (always want to remove max chars in total)
            if left < chars.len() - i{
                if last.is_some() && last.unwrap() != chars[i] {
                    accum = (0, accum.1 + token_compress_len(accum.0));
                }
                accum.0 += 1;
                get_min(chars, i+1, Some(chars[i]), accum, left, out);
            }

        }

        fn token_compress_len(repeat: usize) -> usize {
            if repeat == 0 {return 0}
            else if repeat == 1 {return 1}
            else if repeat < 10 {return 2}
            else if repeat < 100 {return 3}
            else {return 4}
        }

        let mut out = Vec::<usize>::new();
        get_min(&s.chars().collect(), 0, None, (0, 0), k as usize, &mut out);

        *out.iter().min().unwrap() as i32
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ans = Solution::get_length_of_optimal_compression(
            "aaabcccd".to_string(), 2);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test_2() {
        let ans = Solution::get_length_of_optimal_compression(
            "aabbaa".to_string(), 2);
        assert_eq!(ans, 2);
        // assert_eq!(ans, 3);
    }

    #[test]
    fn test_3() {
        let ans = Solution::get_length_of_optimal_compression(
            "aaaaaaaaaaa".to_string(), 0);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_4() {
        let ans = Solution::get_length_of_optimal_compression(
            "".to_string(), 0);
        assert_eq!(ans, 0);
    }

    // #[test]
    // fn test_4() {
    //     let ans = Solution::get_length_of_optimal_compression(
    //         vec!['a'; 100].iter().collect::<String>(), 50);
    //     assert_eq!(ans, 3);
    // }
}
