struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {return s;}

        let chunk_size = 2*num_rows as usize-2;
        let chunks = s.len()/chunk_size;

        let chars: Vec<char> = s.chars().collect();
        let mut out = String::new();
        for i in 0..num_rows as usize {
            for j in 0..chunks+1 {
                let index_1 = j * chunk_size + i;
                let index_2 = (j+1) * chunk_size - i;

                if index_1 < s.len() {
                    out.push(chars[index_1]);
                }
                if index_2 < s.len() && i != 0 && index_1 != index_2 {
                    out.push(chars[index_2]);
                }
            }
        }
        out
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
        let ans = Solution::convert("PAYPALISHIRING".to_string(), 3);
        assert_eq!(ans, "PAHNAPLSIIGYIR".to_string())
    }

    #[test]
    fn test2() {
        let ans = Solution::convert("PAYPALISHIRING".to_string(), 4);
        assert_eq!(ans, "PINALSIGYAHRPI".to_string())
    }

    #[test]
    fn test3() {
        let ans = Solution::convert("PAYPALISHIRING".to_string(), 1);
        assert_eq!(ans, "PAYPALISHIRING".to_string())
    }

    #[test]
    fn test4() {
        let ans = Solution::convert("P".to_string(), 5);
        assert_eq!(ans, "P".to_string())
    }
}
