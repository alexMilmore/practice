struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (n, d) = (job_difficulty.len(), d as usize);
        if d > n {return -1;}

        let mut dp = vec![vec![i32::MAX; d + 1]; n + 1];
        dp[n][d] = 0;
        for j in (0..d).rev() {
            for i in (j..=n-(d-j)).rev() {
                let mut day_difficulty = 0;
                for k in i..=n-(d-j) {
                    day_difficulty = day_difficulty.max(job_difficulty[k]);
                    dp[i][j] = dp[i][j].min(day_difficulty.saturating_add(dp[k+1][j+1]));
                }
            }
        }
        dp[0][0]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ans = Solution::min_difficulty(vec![6,5,4,3,2,1], 2);
        assert_eq!(ans, 7);
    }

    #[test]
    fn test2() {
        let ans = Solution::min_difficulty(vec![6,5,4,4,5,6], 3);
        assert_eq!(ans, 16);
    }

    #[test]
    fn test3() {
        let ans = Solution::min_difficulty(vec![1,1,1], 3);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test4() {
        let ans = Solution::min_difficulty(vec![1], 3);
        assert_eq!(ans, -1);
    }

    #[test]
    fn test5() {
        let test_data = vec![380,302,102,681,863,676,243,671,651,612,162,561,394,856,601,30,6,257,921,405,716,126,158,476,889,699,668,930,139,164,641,801,480,756,797,915,275,709,161,358,461,938,914,557,121,964,315];
        let ans = Solution::min_difficulty(test_data, 10);
        assert_eq!(ans, 3807);
    }


}
