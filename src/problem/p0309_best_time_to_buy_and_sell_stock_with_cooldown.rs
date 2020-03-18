/**
 * [309] Best Time to Buy and Sell Stock with Cooldown
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 * 
 * Design an algorithm to find the maximum profit. You may complete as many transactions as you like (ie, buy one and sell one share of the stock multiple times) with the following restrictions:
 * 
 * 
 * 	You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
 * 	After you sell your stock, you cannot buy stock on next day. (ie, cooldown 1 day)
 * 
 * 
 * Example:
 * 
 * 
 * Input: [1,2,3,0,2]
 * Output: 3 
 * Explanation: transactions = [buy, sell, cooldown, buy, sell]
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::max;

        if prices.len() < 2 {
            return 0;
        }
        let mut hold = [-prices[0], 0];
        let mut cash = [0, 0, 0];
        let mut ix: i32 = 1;
        for i in &prices[1..] {
            let ih = ix.rem_euclid(2);
            let ic = ix.rem_euclid(3);
            hold[ih as usize] = max(hold[(ih - 1).rem_euclid(2) as usize], cash[(ic - 2).rem_euclid(3) as usize] - *i);
            cash[ic as usize] = max(cash[(ic - 1).rem_euclid(3) as usize], hold[(ih - 1).rem_euclid(2) as usize] + *i);
            ix += 1;
        }
        return cash[(prices.len()-1).rem_euclid(3) as usize];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_309() {
        assert_eq!(
            Solution::max_profit(vec![1, 2, 3, 0, 2]),
            3
        )
    }
}
