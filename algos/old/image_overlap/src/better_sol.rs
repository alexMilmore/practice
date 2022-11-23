impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len() as i32;
        let img1 = Self::translate(&img1);
        let img2 = Self::translate(&img2);
        let mut res = 0;
        for vv in (1-n)..=(n-1) {
            for hh in (1-n)..=(n-1) {
                let c = Self::compare(&img1, &img2, vv, hh);
                res = std::cmp::max(res, c);
            }
        }
        res as i32
    }
    
    fn compare(img1: &[u32], img2: &[u32], vv: i32, hh: i32) -> usize {
        let n = img1.len();
        let (img1,img2) = if vv<0 {
            (img1, &img2[vv.abs() as usize..])
        } else {
            (&img1[vv as usize..], img2)
        };
        if hh<0 {
            img1.iter().zip(img2.iter()).map(|(i1, i2)| (i1 & (i2<<hh.abs() as usize)).count_ones()).sum::<u32>() as usize
        } else {
            img1.iter().zip(img2.iter()).map(|(i1, i2)| (i1 & (i2>>hh.abs() as usize)).count_ones()).sum::<u32>() as usize
        }
    }
    
    fn translate(img: &Vec<Vec<i32>>) -> Vec<u32> {
        let mut res = Vec::new();
        for ii in 0..img.len() {
            let mut res_ii = 0;
            for x in img[ii].iter() {
                res_ii <<= 1;
                if *x==1 { res_ii |= 1; }
            }
            res.push(res_ii);
        }
        res
    }
}
