pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut max_sell = i32::MIN;
    let mut max_profit = i32::MIN;

    for i in 0..len {
        let current = len-i-1;
        max_sell = std::cmp::max(max_sell, prices[current]);
        max_profit = std::cmp::max(max_profit, max_sell - prices[current]);
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_case_returns_0() {
        let vec = vec![5];

        assert_eq!(max_profit(vec), 0);
    }

    #[test]
    fn constant_price_returns_0() {
        let vec = vec![5,5,5,5,5,5];

        assert_eq!(max_profit(vec), 0);
    }

    #[test]
    fn decreasing_price_returns_0() {
        let vec = vec![6,5,4,3,2,1];

        assert_eq!(max_profit(vec), 0);
    }

    #[test]
    fn increasing_price_returns_max() {
        let vec = vec![1,2,3,4,5,6];

        assert_eq!(max_profit(vec), 5);
    }

    #[test]
    fn more_realistic_case() {
        let vec = vec![100,50,20,10,15,10,40,100,97,0,50,89];

        assert_eq!(max_profit(vec), 90);
    }

    #[test]
    fn three_point_peak() {
        let vec = vec![1,4,2];

        assert_eq!(max_profit(vec), 3);
    }

    #[test]
    fn leetcode_case() {
        let vec = vec![3,2,6,5,0,3];

        assert_eq!(max_profit(vec), 4);
    }

}
