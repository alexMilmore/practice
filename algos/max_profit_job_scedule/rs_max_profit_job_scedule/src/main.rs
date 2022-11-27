struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        // sort jobs
        let mut jobs = vec![];
        let job_iter = start_time.iter()
            .zip(end_time.iter())
            .zip(profit.iter());
        for ((start, end), profit) in job_iter {
            jobs.push((start, end, profit));
        }
        jobs.sort_by(|a, b| a.0.cmp(b.0));

        // find best
        let mut cache = vec![0; start_time.len()];
        let mut best = 0;
        for (i, (start, end, profit)) in jobs.iter().enumerate().rev() {
            let mut local_best: i32 = 0;
            let mut mem = 0;
            let mut tail = i32::MAX;
            for j in i+1..cache.len() {
                if jobs[j].0 >= end { // check job can be done after this
                    if tail < *jobs[j].0 { break; } // prevent looking at unessesary jobs
                    if tail > *jobs[j].1 { tail = *jobs[j].1; }
                    if local_best < cache[j] { local_best = cache[j]; }
                }
            }
            local_best += **profit;
            cache[i] = local_best;
            if best < local_best { best = local_best; }
        }

        best
    }
}

use std::fs;
fn main() {
    let input = fs::read_to_string("test_data/test_3").unwrap();
    let split = input.split('\n').collect::<Vec<&str>>();
    let (start, end, profit) = (split[0], split[1], split[2]);
    let start = split[0].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let end = split[1].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let profit = split[2].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();

    let ans = Solution::job_scheduling(start, end, profit);
    
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    // Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
    // Output: 120
    #[test]
    fn test_1() {
        let ans = Solution::job_scheduling(
            vec![1,2,3,3],
            vec![3,4,5,6],
            vec![50,10,40,70],
        );
        assert_eq!(ans, 120);
    }

    // Input: startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
    // Output: 6
    #[test]
    fn test_2() {
        let ans = Solution::job_scheduling(
            vec![1,1,1],
            vec![2,3,4],
            vec![5,6,4],
        );
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_3() {
        let input = fs::read_to_string("test_data/test_3").unwrap();
        let split = input.split('\n').collect::<Vec<&str>>();
        let (start, end, profit) = (split[0], split[1], split[2]);
        let start = split[0].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
        let end = split[1].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
        let profit = split[2].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();

        let ans = Solution::job_scheduling(start, end, profit);
        assert_eq!(ans, 3072992);
    }
}
