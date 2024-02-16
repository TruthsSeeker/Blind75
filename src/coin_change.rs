use std::collections::HashMap;

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut memo = HashMap::new();
    change(&coins, amount, &mut memo)
    
}

fn change(coins: &Vec<i32>, amount: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    // base case
    if amount < 0 {
        return -1;
    } else if amount == 0 {
        return 0;
    } else if let Some(a) = memo.get(&amount) {
        return *a;
    }
    
    let mut min = i32::MAX;
    for coin in coins {

        
        // recursion
        let sub = change(coins, amount - coin, memo);
        if sub == -1 {
            continue;
        }
        // Insert amount-coin in memo  if memo[amount-coin] == null || memo[amount-coin] + 1 < memo[amount]
        if let Some(a) = memo.get(&(&amount - coin)).cloned() {
            if a < min {
                memo.insert(amount, sub + 1);
                min = sub + 1;
            }
        } else {
            memo.insert(amount, sub + 1);
            min = sub + 1;
        }

    }
    
    return match memo.get(&amount) {
        Some(i) => *i,
        None => {
            memo.insert(amount, -1);
            -1
        },
    }
}


pub fn coin_change_bu(coins: Vec<i32>, amount: i32) -> i32 {
    let mut memo = vec![i32::MAX - 1; (amount + 1) as usize];
    memo[0] = 0;

    for a in 1..amount+1 {
        for c in coins.iter() {
            if a - c >= 0 {
                memo[a as usize] = memo[a as usize].min(1 + memo[(a - c) as usize]);
            }
        }
    }
    if memo[amount as usize] != i32::MAX - 1{
        memo[amount as usize]
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(coin_change(vec![4,3,2], 17), 5);
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
        assert_eq!(coin_change(vec![1], 1), 1);
        assert_eq!(coin_change(vec![1], 2), 2);
        assert_eq!(coin_change(vec![186,419,83,408], 6249), 20);
    }

    #[test]
    fn test_coin_change_bu() {
        assert_eq!(coin_change_bu(vec![4,3,2], 17), 5);
        assert_eq!(coin_change_bu(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change_bu(vec![2], 3), -1);
        assert_eq!(coin_change_bu(vec![1], 0), 0);
        assert_eq!(coin_change_bu(vec![1], 1), 1);
        assert_eq!(coin_change_bu(vec![1], 2), 2);
        assert_eq!(coin_change_bu(vec![186,419,83,408], 6249), 20);
    }
}