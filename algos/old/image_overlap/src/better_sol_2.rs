impl Solution {
    fn pack(row : &Vec<i32>) -> u32 {
        let mut result : u32 = 0;
        for x in row {
            result <<= 1;
            result |= (*x & 1) as u32;
        }
        result
    }
    
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let N = img1.len() as i32;
        let pi1 = img1.iter().map(|r| Solution::pack(r)).collect::<Vec<u32>>();
        let pi2 = img2.iter().map(|r| Solution::pack(r)).collect::<Vec<u32>>();
        
        let mut max_overlap_count = 0;
        for ud in (-N-1..=N-1) {
            for lr in (-N-1..=N-1) {
                let mut overlap_count = 0;
                for r in (0..N-i32::abs(ud)) {
                    let rt = 
                        if ud < 0 {
                            (pi1[r as usize],pi2[(-ud+r) as usize])
                        }
                        else {
                            (pi1[(ud+r) as usize],pi2[r as usize])
                        };
                    if lr < 0 {
                        overlap_count += (rt.0 & (rt.1 >> -lr)).count_ones();
                    }
                    else {
                        overlap_count += (rt.0 & (rt.1 <<  lr)).count_ones();    
                    }
                }
                max_overlap_count = u32::max(max_overlap_count, overlap_count);
            }
        }
        max_overlap_count as i32
    }
}
