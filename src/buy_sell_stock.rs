use std::collections::VecDeque;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut min_price, mut max_profit) = (10000, 0);

    for price in prices {
        max_profit = max_profit.max(price - min_price);
        min_price = min_price.min(price);
    }
    max_profit
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 5);
        assert_eq!(max_profit(vec![7,6,4,3,1]), 0);
    }
}