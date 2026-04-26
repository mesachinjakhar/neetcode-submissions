impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut prefix = vec![0; n]; 
        let mut suffix = vec![0; n]; 

        prefix[0] = height[0];
        suffix[n-1] = height[n-1];

        // cal prefix 
        for i in 1..n {
            prefix[i] = max(height[i], prefix[i-1]); 
        }
        // cal suffix
        for i in (0..n-1).rev() {
            suffix[i] = max(height[i], suffix[i+1]);
        }

        let mut ans = 0; 
        for i in 1..n-1 {
            ans += min(prefix[i], suffix[i]) - height[i];
        }

        return ans; 

    }
}
