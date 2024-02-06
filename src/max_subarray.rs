pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut max = nums[0];
    for n in nums {
        sum = n.max(sum + n);
        max = max.max(sum);
    }
    max
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![0]), 0);
        assert_eq!(max_sub_array(vec![-1]), -1);
        assert_eq!(max_sub_array(vec![-100000]), -100000);
    }
}