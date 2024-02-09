pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut sum_nums = 0;
    let mut sum_series = 0;

    for i in 0..nums.len() {
        sum_nums += nums[i];
        sum_series += i as i32 + 1;
    }
    sum_series - sum_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}