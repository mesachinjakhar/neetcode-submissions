impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new(); 
        
        for i in 0..nums.len() {
            let diff = target - nums[i]; 
            if map.contains_key(&diff) {
                let idx = map.get(&diff).unwrap(); 
                return vec![*idx as i32, i as i32]; 
            } else {
                map.insert(nums[i], i);
            }
        }

        return vec![]
    }
}
