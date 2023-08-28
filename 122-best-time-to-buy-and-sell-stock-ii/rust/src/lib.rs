pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut state = State::MustBuy{ candidate: 0 };
    let mut max_profit = 0;
    let len = prices.len();

    for i in 0..len {
        state = match state {
            State::MustBuy { candidate } if prices[i] >= prices[candidate] => {
                State::MustSell { candidate: i, bought: candidate }
            },
            State::MustBuy { .. } => {
                State::MustBuy { candidate: i }
            },
            State::MustSell { candidate, bought } if prices[i] < prices[candidate] => {
                max_profit += prices[candidate] - prices[bought];
                State::MustBuy { candidate: i }
            },
            State::MustSell { bought, .. } => {
                State::MustSell { candidate: i, bought }
            }
        }
    }

    match state {
        State::MustSell { bought, .. } if prices[len - 1] > prices[bought] => {
            max_profit + prices[len - 1] - prices[bought]
        },
        _ => max_profit
    }
}

pub fn max_profit_cooler(prices: Vec<i32>) -> i32 {
        // Improved version of another Leetcoder's solution
        // This also works because you can buy and sell in the same day
        // so 1,2,3,1,4,5 = 6 because:
        // 1. (2-1)+(3-2)+(4-1)+(5-4) = 6 (other Leetcoder)
        // 2. (3-1)+(5-1) = 6 (my former state-machine solution)
        
        prices.windows(2)
            .filter_map(|c| c[1].checked_sub(c[0]))
            .filter(|profit| *profit > 0)
            .sum()
}


enum State {
    MustBuy { candidate: usize },
    MustSell { candidate: usize, bought: usize },
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

        assert_eq!(max_profit(vec), 5+90+89);
    }

    #[test]
    fn three_point_peak() {
        let vec = vec![1,4,2];

        assert_eq!(max_profit(vec), 3);
    }

    #[test]
    fn leetcode_case() {
        let vec = vec![3,2,6,5,0,3];

        assert_eq!(max_profit(vec), 4+3);
    }

}
