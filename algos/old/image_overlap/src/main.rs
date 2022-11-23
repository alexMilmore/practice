struct Solution;

fn pack(img: Vec<Vec<i32>>) -> Vec<i32> {
    let n = img.len();
    let mut out: Vec<i32> = vec![0; n];
    for i in 0..n {
        for j in 0..n {
          out[i] |= img[i][j] << j;
        }
    }
    out 
}

fn get_overlap(img1: &Vec<i32>, img2: &Vec<i32>, offset: (usize, usize)) -> u32 {
    let img1_iter = (&img1[..]).iter();
    let img2_iter = (&img2[offset.0..]).iter();

    for item in img1_iter.clone().zip(img2_iter.clone()) {
        println!("{:?}", item);
    }
    // img1_iter.zip(img2_iter).map(|(a, b)| {println!("{:#08b}, {:#08b}, {:#08b}, {:?}", a, b, b << offset.1, offset); (a | b << offset.1).count_ones()}).sum::<u32>()
    img1_iter.zip(img2_iter).map(|(a, b)| {(a | b << offset.1).count_ones()}).sum::<u32>()
}

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let img1 = pack(img1);
        let img2 = pack(img2);
        let mut max_overlap = 0;
        // println!("packed {:?} {:?}", img1, img2);

        let n = img1.len();
        let offset_iter =
            (0..n).zip(std::iter::repeat(0))
                .chain(std::iter::repeat(0).zip(0..n));
        // for item in offset_iter.clone() {
        //     println!("{:?}", item)
        // }

        for offset in offset_iter {
            max_overlap = max_overlap.max(get_overlap(&img1, &img2, offset));
            // max_overlap = max_overlap.max(get_overlap(&img2, &img1, offset));
        }
        println!("done");
        max_overlap as i32
    }
}

fn main() {
    let ans = Solution::largest_overlap(
        vec!(vec!(1,1,0), vec!(1,1,0), vec!(1,1,0)),
        vec!(vec!(0,0,0), vec!(0,1,1), vec!(0,0,1)),
    );
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

    // #[test]
    // fn test2() {
    //     let ans = Solution::largest_overlap(
    //         vec!(vec!(1)),
    //         vec!(vec!(1)),
    //     );
    //     assert_eq!(ans, 1);
    // }

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
