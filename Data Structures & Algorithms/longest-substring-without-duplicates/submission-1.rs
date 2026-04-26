use std::collections::HashSet; 
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len(); 
        if n == 0 || n == 1 {
            return n as i32; 
        }

        let mut set = HashSet::new();
        let mut left = 0; 
        let mut right = 0; 
        let s_bytes = s.as_bytes();

        let mut ans = 0; 

        while right < n {
            if !set.contains(&s_bytes[right]) {
                set.insert(s_bytes[right]);
                let dist = right - left + 1; 
                ans = ans.max(dist);
                right += 1; 
            } else {
                  while set.contains(&s_bytes[right]) {
                   set.remove(&s_bytes[left]);
                    left += 1; 
                } 
            }
        }

        return ans as i32; 

    }
}
