pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() > 0 {
        let mut result = 0;
        let mut min = prices[0];
        for i in 1..prices.len() {
            if prices[i] > min {
                let profit = prices[i] - min;
                if result < profit {
                    result = profit;
                }
            } else {
                min = prices[i];
            }
        }
        result
    } else {
        0
    }
}
