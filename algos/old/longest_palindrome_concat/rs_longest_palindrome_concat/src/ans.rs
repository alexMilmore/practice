// one
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut matrix = [[0;26];26];
        let mut count = 0;
        
        for word in words {
            let bword = word.as_bytes();
            let (first, second) = ((bword[0]  - b'a') as usize, (bword[1] - b'a') as usize);
            
            if matrix[second][first] > 0 {
                count += 4;
                matrix[second][first] -= 1;
            } else {
                matrix[first][second] += 1;
            }
        }
        
        for i in 0..matrix.len() {
            if matrix[i][i] > 0 {
                count += 2;
                break;
            }
        }
        
        count 
        
    }
}


// two
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut matrix = [[0;26];26];
        let mut count = 0;
        
        for word in words {
            let bword = word.as_bytes();
            let (first, second) = ((bword[0]  - b'a') as usize, (bword[1] - b'a') as usize);
            
            if matrix[second][first] > 0 {
                count += 4;
                matrix[second][first] -= 1;
            } else {
                matrix[first][second] += 1;
            }
        }
        
        for i in 0..matrix.len() {
            if matrix[i][i] > 0 {
                count += 2;
                break;
            }
        }
        
        count 
        
    }
}

// three
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut map = [[0; 26]; 26];
        let mut length = 0;

        for word in words {
            let bytes = word.as_bytes();
            let (first, second) = ((bytes[0] - b'a') as usize, (bytes[1] - b'a') as usize);

            if map[second][first] > 0 {
                length += 4;
                map[second][first] -= 1;
                continue;
            }

            map[first][second] += 1;
        }

        for i in 0..map.len() {
            if map[i][i] > 0 {
                length += 2;
                break;
            }
        }

        length
    }
}
