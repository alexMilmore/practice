struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {return false;}
        let mut running_min_1 = i32::MAX;
        let mut running_min_2 = i32::MAX;

        for num in nums {
            if num > running_min_2 {return true;}

            if num <= running_min_1 {
                running_min_1 = num;
            }
            else if num < running_min_2 {
                running_min_2 = num;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test1() {
        let ans = Solution::increasing_triplet(vec!(1,2,3,4,5));
        assert_eq!(ans, true);
    }

    #[test]
    fn test2() {
        let ans = Solution::increasing_triplet(vec!(5,4,3,2,1));
        assert_eq!(ans, false);
    }

    #[test]
    fn test3() {
        let ans = Solution::increasing_triplet(vec!(2,1,5,0,7));
        assert_eq!(ans, true);
    }

    #[test]
    fn test4() {
        let ans = Solution::increasing_triplet(vec!(2,1,5,0,3));
        assert_eq!(ans, false);
    }

    #[test]
    fn test5() {
        let ans = Solution::increasing_triplet(vec!(5,1,8,2,0));
        assert_eq!(ans, false);
    }

    #[test]
    fn test6() {
        let ans = Solution::increasing_triplet(vec!(5,1));
        assert_eq!(ans, false);
    }

    #[test]
    fn test7() {
        let ans = Solution::increasing_triplet(vec!(1,1,-2,6));
        assert_eq!(ans, false);
    }
}
