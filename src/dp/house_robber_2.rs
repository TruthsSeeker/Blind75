pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    helper(&nums[..(nums.len() - 1) as usize]).max(helper(&nums[1..]))
}

fn helper(nums: &[i32]) -> i32 {
    let (mut rob1, mut rob2) = (0, 0);
    for n in nums {
        let temp = rob2.max(rob1 + n);
        rob1 = rob2;
        rob2 = temp;
    }
    rob1.max(rob2)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 11);
        assert_eq!(rob(vec![2, 1, 1, 2]), 3);
        assert_eq!(rob(vec![2]), 2);
        assert_eq!(rob(vec![2, 3, 2]), 3);
        assert_eq!(rob(vec![1, 2, 3]), 3);
    }
}