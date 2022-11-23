struct Solution;
    use std::collections::HashSet;

struct Img {
    side_len: usize,
    ones: HashSet<(usize, usize)> // if one at index
}

impl Img {
    fn new(data: Vec<Vec<i32>>) -> Self {
        println!("len {}", data.len());
        let mut ones = HashSet::new();
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                if data[i][j] == 1 { ones.insert((i, j)); }
            }
        }

        Self {
            side_len: data.len(),
            ones
        }
    }

    fn check_overlap(&self, img: &Img, x_offset: i32, y_offset: i32) -> usize {
        let mut counter = 0;
        for (x, y) in self.ones.iter() {
            let offset_index = (
              (*x as i32 + x_offset) as usize,
              (*y as i32 + y_offset) as usize
            );
            match (self.ones.get(&(*x,*y)), img.ones.get(&offset_index)) {
                (Some(_), Some(_)) => counter += 1,
                _ => ()
            }
        }
        counter
    }

    fn offset_index(self, index: (usize, usize), offset: (i32, i32)) -> Option<(usize, usize)> {
        let x_offset = index.0 as i32 + offset.0;
        let y_offset = index.1 as i32 + offset.1;
        if x_offset > self.side_len as i32 || x_offset < 0 ||
           y_offset > self.side_len as i32 || y_offset < 0 {
               return None
        }
        Some((x_offset as usize, y_offset as usize))
    }
}

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let img1 = Img::new(img1);
        let img2 = Img::new(img2);

        let mut max = 0;
        for i in -(img1.side_len as i32)..(img1.side_len as i32) {
            for j in -(img2.side_len as i32)..(img2.side_len as i32) {
                let overlap = img1.check_overlap(&img2, i, j);
                if overlap > max { max = overlap; }
            }
        }
        max as i32
    }
}

use std::iter::repeat;
fn main() {
    let ans = Solution::largest_overlap(
        vec!(vec!(1,1,0), vec!(1,1,0), vec!(1,1,0)),
        vec!(vec!(0,0,0), vec!(0,1,1), vec!(0,0,1)),
    );
    let n = 5;
    // let a  = (0..n).zip(repeat(0)).chain(repeat(0).zip(1..n));
    let a  = (0..n).zip(repeat(0));
    for i in a {
        println!("Hello, world! {}, {:?}", ans, i);
    }
    println!("Hello, world! {}", ans);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ans = Solution::largest_overlap(
            vec!(vec!(1,1,0), vec!(1,1,0), vec!(1,1,0)),
            vec!(vec!(0,0,0), vec!(0,1,1), vec!(0,0,1)),
        );
        assert_eq!(ans, 3);
    }

    #[test]
    fn test2() {
        let ans = Solution::largest_overlap(
            vec!(vec!(1)),
            vec!(vec!(1)),
        );
        assert_eq!(ans, 1);
    }

    #[test]
    fn test3() {
        let ans = Solution::largest_overlap(
            vec!(vec!(0)),
            vec!(vec!(0)),
        );
        assert_eq!(ans, 0);
    }

    #[test]
    fn test4() {
        let ans = Solution::largest_overlap(
            vec!(),
            vec!(),
        );
        assert_eq!(ans, 0);
    }
}
