impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let n = heights.len(); 
        let mut st = 0; 
        let mut end = n - 1; 

        let mut max = 0; 

        while st < end {
            let width = end - st; 
            let ans = width as i32 * min(heights[st], heights[end]); 
            max = max.max(ans); 
            if heights[st] < heights[end] {
                st += 1; 
            } else {
                end -= 1;
            }
        }

        return max
    }
}
