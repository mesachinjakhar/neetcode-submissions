impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort(); 
        let n = nums.len(); 
        let mut ans = Vec::new();
        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; 
            }
            let mut j = i + 1; 
            let mut k = n - 1; 
            while j < k {
                let val = nums[i] + nums[j] + nums[k]; 
                if val == 0 {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1; 
                    k -= 1; 
                    while j < k && nums[j] == nums[j-1] {
                        j += 1;
                    }
                } else if val < 0 {
                    j += 1; 
                } else {
                    k -= 1; 
                }
            }
        }

        return ans; 
    }
}
