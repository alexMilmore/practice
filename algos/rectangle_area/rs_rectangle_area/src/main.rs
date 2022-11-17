struct Solution;

impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let a_area = (ax2-ax1)*(ay2-ay1);
        let b_area = (bx2-bx1)*(by2-by1);

        // get overlap
        let overlap = (ax2.min(bx2) - ax1.max(bx1)).max(0) *
                      (ay2.min(by2) - ay1.max(by1)).max(0);

        a_area.abs() + b_area.abs() - overlap
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Input: ax1 = -3, ay1 = 0, ax2 = 3, ay2 = 4, bx1 = 0, by1 = -1, bx2 = 9, by2 = 2
    // Output: 45
    #[test]
    fn test_1() {
        let ans = Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2);
        assert_eq!(ans, 45);
    }

    // Input: ax1 = -2, ay1 = -2, ax2 = 2, ay2 = 2, bx1 = -2, by1 = -2, bx2 = 2, by2 = 2
    // Output: 16
    #[test]
    fn test_2() {
        let ans = Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2);
        assert_eq!(ans, 16);
    }

    // Input: ax1 = -4, ay1 = -4, ax2 = -1, ay2 = -1, bx1 = 2, by1 = 2, bx2 = 4, by2 = 4
    // Output: 13
    #[test]
    fn test_3() {
        let ans = Solution::compute_area(-4, -4, -1, -1, 2, 2, 4, 4);
        assert_eq!(ans, 13);
    }
}
