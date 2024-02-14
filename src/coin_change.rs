use std::collections::HashMap;

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut coins = coins;
    coins.sort();
    coins.reverse();
    let mut memo = HashMap::new();
    change(&coins, amount, &mut memo)
}

    fn change(coins: &Vec<i32>, amount: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if amount <= 0 {
            return -1;
        }

        for coin in coins {
            // base case
            if amount - coin == 0 {
                memo.insert(amount, 1);
                return 1;
            }

            // recursion

        }

        -1
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
        assert_eq!(coin_change(vec![1], 1), 1);
        assert_eq!(coin_change(vec![1], 2), 2);
    }
}