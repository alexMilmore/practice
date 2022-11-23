struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = height.len()-1;
        let mut area: i32 = 0;

        while l < r {
            area = area.max(((r-l) as i32)*(height[l].min(height[r])));
            
            match height[l].cmp(&height[r]) {
                std::cmp::Ordering::Greater => r -= 1,
                _ => l += 1,
            }
        }

        area
    }
}

fn main() {
    println!("Hello, world! {}", Solution::max_area(vec![1,2,3,4]));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ans = Solution::max_area(vec![1,8,6,2,5,4,8,3,7]);
        assert_eq!(ans, 49);
    }

    #[test]
    fn test_2() {
        let ans = Solution::max_area(vec![1,2]);
        assert_eq!(ans, 1);
    }
}
