use std::collections::HashMap;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new(); 
        let n = nums.len(); 
        let k = k as usize; 
        let mut ans = 0; 
        let mut left = 0; 

        for right in 0..n {
            *map.entry(&nums[right]).or_insert(0) += 1;
            let val = map.get(&1); 
            let mut max = 0;
            if val.is_some() {
                max = *val.unwrap();
            }
            let window_size = right - left + 1; 
            if window_size - max > k {
                let val = map.get_mut(&nums[left]).unwrap(); 
                *val -= 1; 
                if *val == 0 {
                    map.remove(&nums[left]); 
                }
                left += 1; 
            }
            let window_size = right - left + 1; 
            ans = ans.max(window_size); 
        }

        return ans as i32; 
    }
}