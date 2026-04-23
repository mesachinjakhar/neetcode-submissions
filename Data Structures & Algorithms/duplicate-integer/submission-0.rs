use std::collections::HashSet; 
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new(); 
        for i in 0..nums.len() {
            if set.contains(&nums[i]) {
                return true; 
            } else {
                set.insert(nums[i]);
            }
        }

        return false; 
    }
}
