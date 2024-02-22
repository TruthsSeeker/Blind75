use std::collections::HashMap;

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut memo = HashMap::new();
    rob_memo(&nums, 0, &mut memo)
}

fn rob_memo(nums: &Vec<i32>, start: usize, memo: &mut HashMap<usize, i32>) -> i32 {
    if nums.len().saturating_sub(start) == 0 {
        return 0;
    }
    if let Some(m) = memo.get(&start) {
        return *m;
    }
    let mut max = 0;
    for i in start..nums.len() {
        let result = nums[i] + rob_memo(nums, i + 2, memo);
        max = max.max(result)
    }
    memo.insert(start, max);
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(rob(vec![2, 1, 1, 2]), 4);
        assert_eq!(rob(vec![2]), 2);
    }
}