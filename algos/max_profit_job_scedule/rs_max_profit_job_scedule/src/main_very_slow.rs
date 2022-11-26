struct Solution;

// all tasks that can be completed after
fn get_compatible(index: usize, end_time: i32, start_times: &Vec<i32>) -> Vec<usize> {
    let mut out = vec![];
    for (i, time) in start_times.iter().enumerate() {
        if i != index && *time >= end_time {
            out.push(i);
        }
    }
    out
}

// get best possible
pub fn recurse(i: usize, start_time: &Vec<i32>, end_time: &Vec<i32>, profit: &Vec<i32>, cache: &mut Vec<Option<i32>>)
-> i32 {
    // println!("{} {:?}", i, cache);
    match cache[i] {
        Some(val) => {
            println!("hit cache");
            return val;
        },
        _ => (),
    }

    let mut max = 0;
    for j in get_compatible(i, end_time[i], start_time) {
        let new_max = recurse(j, start_time, end_time, profit, cache);
        if new_max > max { max = new_max; }
    }

    max += profit[i];
    cache[i] = Some(max);
    max
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut cache: Vec<Option<i32>> = vec![None; start_time.len()];
        for i in 0..start_time.len() {
            let new_max = recurse(i, &start_time, &end_time, &profit, &mut cache);
            if new_max > max { max = new_max; }
        }
        max
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
        // assert_eq!(ans, 12);
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
