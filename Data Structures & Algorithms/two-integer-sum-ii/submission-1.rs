impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut st = 0; 
        let mut end = numbers.len() - 1; 
        let mut ans = Vec::new();
        while st < end {
            let val = numbers[st] + numbers[end]; 
            if val == target {
                ans.push(st as i32 + 1);
                ans.push(end as i32 + 1); 
                break; 
            } else if val > target {
                end -= 1; 
            } else {
                st += 1; 
            }
        }

        return ans; 
    }
}
