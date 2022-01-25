/**
 * [123] Best Time to Buy and Sell Stock III
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 * 
 * Design an algorithm to find the maximum profit. You may complete at most two transactions.
 * 
 * Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
 * 
 * Example 1:
 * 
 * 
 * Input: [3,3,5,0,0,3,1,4]
 * Output: 6
 * Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 *              Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
 * 
 * Example 2:
 * 
 * 
 * Input: [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
 *              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
 *              engaging multiple transactions at the same time. You must sell before buying again.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transaction is done, i.e. max profit = 0.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::max;

        if prices.len() < 2 {
            return 0;
        }

        let mut hold1p;
        let mut hold1 = -prices[0];
        // let mut cache1p = 0;
        let mut cache1 = 0;
        let mut hold2p = 0;
        let mut hold2 = 0;
        // let mut cache2p;
        let mut cache2 = 0;
        let mut ix = 1;
        for i in &prices[1..] {
            hold1p = hold1;
            hold2p = hold2;
            // cache1p = cache1;

            cache1 = max(cache1, hold1p + *i);

            hold1 = max(hold1p, -*i);

            if ix == 2 {
                hold2 = cache1 - *i;
                cache2 = cache1;
            } else if ix > 2 {
                cache2 = max(cache2, max(cache1, hold2p + *i));

                hold2 = max(hold2p, cache1 - *i);
            }

            ix += 1;
        }
        // cache2 = max(cache1p, hold2p + *i);
        return max(cache1,cache2);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        // assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
        assert_eq!(Solution::max_profit(vec![3,3,5,0,0,3,1,4]), 6);
        // assert_eq!(Solution::max_profit(vec![1,2,3,4,5]), 4)
    }
}
