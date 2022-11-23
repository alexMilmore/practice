struct Solution;

struct Cache {
    data: Vec<i32>,
    range_cache: std::collections::HashMap<(usize, usize), i32>,
    partition_cache: std::collections::HashMap<(usize, usize, usize), i32>
}

impl Cache {
    fn new(data: Vec<i32>) -> Self {
        Self {
            data,
            range_cache: std::collections::HashMap::new(),
            partition_cache: std::collections::HashMap::new()
        }
    }

    fn get_range_min(&mut self, start: usize, end: usize) -> i32 {
        match self.range_cache.get(&(start, end)) {
            Some(val) => *val,
            None => {
                let max = *self.data[start..end].iter().max().unwrap();
                self.range_cache.insert((start, end), max);
                max
            }
        }
    }

    fn get_partition_min(&mut self, i: usize, d: usize, partition_start: usize) -> i32 {
        match self.partition_cache.get(&(i, d, partition_start)) {
            Some(val) => *val,
            None => {
                let val = self.permute_partitions(i, d, partition_start);
                self.partition_cache.insert((i, d, partition_start), val);
                val
            }
        }
    }

    fn permute_partitions(&mut self, i: usize, d: usize, partition_start: usize) -> i32 {
        if i == self.data.len() {
            return self.get_range_min(partition_start, i)
        }

        if d <= 0 || i == 0 { // cannot partition
            self.get_partition_min(i+1, d, partition_start)
        }
        else if d > self.data.len()-i { // must exhaust remaining partitions
            self.get_partition_min(i+1, d-1, i) + self.get_range_min(partition_start, i)
        }
        else { // can either partition or not
            std::cmp::min(
                self.get_partition_min(i+1, d-1, i) + self.get_range_min(partition_start, i),
                self.get_partition_min(i+1, d, partition_start)
            )
        }
    }
}

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        if (d as usize) > job_difficulty.len() {return -1}

        let mut cache = Cache::new(job_difficulty);
        return cache.get_partition_min(0, d as usize, 0);
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
        assert_eq!(ans, 5);
    }


}
