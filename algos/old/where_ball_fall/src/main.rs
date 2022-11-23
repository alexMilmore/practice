struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        // loop over all balls
        let mut out = Vec::<i32>::new();
        for ball in 0..grid[0].len() {
            match Solution::ball_path(&grid, ball) {
                Some(val) => out.push(val as i32),
                None => out.push(-1),
            }
        }
        out
    }

    fn ball_path(grid: &Vec<Vec<i32>>, mut ball_pos: usize) -> Option<usize> {
        for row in grid {
            // check if ball gets stuck
            if row[ball_pos] == -1 && (ball_pos == 0 || row[ball_pos-1] == 1)
            { return None; }
            if row[ball_pos] == 1 && (ball_pos == row.len()-1 || row[ball_pos+1] == -1)
            { return None; }
            // next place ball ends
            if row[ball_pos] > 0 { ball_pos += 1 } else { ball_pos -= 1 };
        }
        Some(ball_pos)
    }
}

fn main() {
    println!("Hello, world! {:?}", Solution::find_ball(vec!(vec!())));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ans = Solution::find_ball(vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]]);
        assert_eq!(ans, vec![1,-1,-1,-1,-1]);
    }

    #[test]
    fn test_2() {
        let ans = Solution::find_ball(vec![vec![-1]]);
        assert_eq!(ans, vec![-1]);
    }

    #[test]
    fn test_3() {
        let ans = Solution::find_ball(vec![vec![1,1,1,1,1,1],vec![-1,-1,-1,-1,-1,-1],vec![1,1,1,1,1,1],vec![-1,-1,-1,-1,-1,-1]]);
        assert_eq!(ans, vec![0,1,2,3,4,-1]);
    }
}
