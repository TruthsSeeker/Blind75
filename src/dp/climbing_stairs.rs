
use std::collections::HashMap;

/// Recursive solution to leetcode 70. Climbing Stairs
/// 
/// link: https://leetcode.com/problems/climbing-stairs/description/
/// 
/// ## Complexity
/// ### Time
/// This is the classic case of recursive complexity. Which is branches^depth
/// 
/// Here branches = 2, one branch for climbing 1 step at a time, one branch for climbing 2
/// 
/// **O(2^n)**
/// 
/// Using memoization allows for deduplication of work so each problem will be solved only once.
/// 
/// **O(n)**
/// 
/// ### Space
/// 
/// As this algorithm is essentially a DFS of the height of the stairs to climb, 
/// the space complexity will be equal to the depth of the graph in this case:
/// 
/// **O(n)**
pub fn climb_stairs(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    climb(n, &mut memo)
}

pub fn climb(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        1
    } else if let Some(m) = memo.get(&n) {
        *m
    } else {
        let result = climb(n - 1, memo) + climb(n - 2, memo);
        memo.insert(n, result);
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
    }
}