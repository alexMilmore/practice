struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 0 { return false }
        let mut n = n;
        for divisor in [2,3,5] {
            while n % divisor == 0 { n /= divisor; }
        }
        n == 1
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Input: n = 6
    // Output: true
    #[test]
    fn test_1() {
        let ans = Solution::is_ugly(6);
        assert_eq!(ans, true);
    }

    // Input: n = 1
    // Output: true
    #[test]
    fn test_2() {
        let ans = Solution::is_ugly(1);
        assert_eq!(ans, true);
    }

    // Input: n = 14
    // Output: false
    #[test]
    fn test_3() {
        let ans = Solution::is_ugly(14);
        assert_eq!(ans, false);
    }

    // Input: n = 8
    // Output: true
    #[test]
    fn test_4() {
        let ans = Solution::is_ugly(8);
        assert_eq!(ans, true);
    }
}
