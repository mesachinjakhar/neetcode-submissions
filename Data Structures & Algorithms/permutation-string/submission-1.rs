impl Solution {
    pub fn is_same(curr_freq: &Vec<i32>, freq: &Vec<i32>) -> bool {
        curr_freq == freq
    }
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s2.len();
        let mut curr_freq = vec![0; 26]; 
        let mut freq = vec![0; 26]; 
        let k = s1.len(); 

        if k > n {
            return false;
        }

        let s1_bytes = s1.as_bytes(); 
        let s2_bytes = s2.as_bytes(); 

        // store freq; 
        for i in 0..k {
            let idx = s1_bytes[i] - b'a'; 
            freq[idx as usize] += 1; 
        }

        // build first window 
        for i in 0..k {
            let idx = s2_bytes[i] - b'a'; 
            curr_freq[idx as usize] += 1; 
        }

        // slide window 
        for right in k..n {
            let left = right - k;
            if Self::is_same(&curr_freq, &freq) {
                return true; 
            }
            let right_idx = s2_bytes[right] - b'a'; 
            let left_idx = s2_bytes[left] - b'a'; 
            curr_freq[right_idx as usize] += 1; 
            curr_freq[left_idx as usize] -= 1; 
        }

        if Self::is_same(&curr_freq, &freq) {
    return true;
}

        return false; 
    }
}