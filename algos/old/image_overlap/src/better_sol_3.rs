use std::iter::repeat;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();

        // mirrored: [1, 1, 0, 1] => 0b1011
        // doesn't change the answer
        let mut i1 = vec![0; n];
        for (i, line) in img1.into_iter().enumerate() {
            for (j, p) in line.into_iter().enumerate() {
                i1[i] |= p << j;
            }
        }

        let mut i2 = vec![0; n];
        for (i, line) in img2.into_iter().enumerate() {
            for (j, p) in line.into_iter().enumerate() {
                i2[i] |= p << j;
            }
        }

        let mut res = 0;

        for (v1, v2) in (0..n).zip(repeat(0)).chain(repeat(0).zip(1..n)) {
            for (h1, h2) in (0..n).zip(repeat(0)).chain(repeat(0).zip(1..n)) {
                let iter1 = i1.iter().skip(v1);
                let iter2 = i2.iter().skip(v2);

                let ones = iter1.zip(iter2)
                    .map(|(a, b)| ((a << h1) & (b << h2)).count_ones())
                    .sum::<u32>();
                res = res.max(ones);
            }
        }

        res as _
    }
}
