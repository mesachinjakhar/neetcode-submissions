use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut v = vec![0; 26]; 
        let mut left = 0; 
        let mut right = 0; 
        let n = s.len(); 

        let s_bytes = s.as_bytes();
        let mut max_window = 0; 
        let mut max_freq = 0; 

        while right < n {
            v[(s_bytes[right] - b'A') as usize] += 1; 
            max_freq = max_freq.max(v[(s_bytes[right] - b'A') as usize]);
            
            let curr_wind = right - left + 1; 
            let curr_flip = curr_wind - max_freq;
            
            if curr_flip as i32 > k {
                 v[(s_bytes[left] - b'A') as usize] -= 1; 
                 left += 1;
            } 
            
            max_window = max_window.max(right - left + 1);
            right += 1; 
        }


        return max_window as i32
    }
}