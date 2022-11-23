struct Solution;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut queue = vec![(start, 0)];
        let mut visited = std::collections::HashSet::<String>::new();

        while queue.len() != 0 {
            let active = queue.pop().unwrap();
            visited.insert(active.0.clone());
            if active.0 == end { return active.1; }
            for candidate in &bank {
                if visited.get(candidate).is_some() { continue; }
                if Solution::ham_dist(&candidate, &active.0) == 1 { queue.insert(0, (candidate.clone(), active.1+1)); }
            }
        }
        -1
    }

    fn ham_dist(a: &String, b: &String) -> usize {
        let mut count = 0;
        for (a_char, b_char) in a.chars().zip(b.chars()) {
            if a_char != b_char { count += 1 }
        }
        count
    }
}

fn main() {
    println!("Hello, world! {}",
        Solution::min_mutation("".to_string(), "".to_string(), vec![]));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ans = Solution::min_mutation(
            "AACCGGTT".to_string(), "AACCGGTA".to_string(),
            vec!["AACCGGTA".to_string()]);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test_2() {
        let ans = Solution::min_mutation(
            "AACCGGTT".to_string(), "AAACGGTA".to_string(),
            vec!["AACCGGTA".to_string(),"AACCGCTA".to_string(),"AAACGGTA".to_string()]);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_3() {
        let ans = Solution::min_mutation(
            "AAAAACCC".to_string(), "AACCCCCC".to_string(),
            vec!["AAAACCCC".to_string(),"AAACCCCC".to_string(),"AACCCCCC".to_string()]);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_4() {
        let ans = Solution::min_mutation(
            "AACCGGTT".to_string(), "AACCGCTA".to_string(),
            vec!["AACCGGTA".to_string(),"AACCGCTA".to_string(),"AAACGGTA".to_string()]);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_5() {
        let ans = Solution::min_mutation(
            "AAAAAAAA".to_string(), "AAAAACGG".to_string(),
            vec!["AAAAAAGA".to_string(),"AAAAAGGA".to_string(),"AAAAACGA".to_string(),"AAAAACGG".to_string(),"AAAAAAGG".to_string(),"AAAAAAGC".to_string()]);
        assert_eq!(ans, 3);
    }

}
