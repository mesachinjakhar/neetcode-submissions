impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map = HashMap::new(); 
        let n = s1.len();

        for c in s1.chars() {
            *map.entry(c).or_insert(0) += 1; 
        }

        let m = s2.len();

        let mut left = 0; 
        let mut right = 0;

        let s2_bytes = s2.as_bytes(); 
        let mut current_map = map.clone();

        while right < m {
            let ch = s2_bytes[right] as char;
            if !map.contains_key(&ch) {
                current_map = map.clone();
                left = right + 1; 
                right += 1; 
            } else {
                while current_map.get(&ch).copied().unwrap_or(0) == 0 {
                    let left_ch = s2_bytes[left] as char;
                    *current_map.entry(left_ch).or_insert(0) += 1;
                    left += 1;
                }
                let v = current_map.get_mut(&ch).unwrap(); 
                *v -= 1; 
                if right - left + 1 == n {
                    return true;
                }
                right += 1; 
            }
        }

        false
    }
}