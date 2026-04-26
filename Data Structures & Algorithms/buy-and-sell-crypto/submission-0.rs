impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut best_buy = prices[0];
        let mut ans = 0; 

        for i in 1..prices.len() {
            if prices[i] < best_buy {
                best_buy = prices[i];
            } 
            let profit = prices[i] - best_buy; 
            ans = ans.max(profit); 
        }

        return ans;
    }
}
