pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut res = nums.iter().fold(i32::MIN, |a,b| a.max(*b));
    let mut max = 1;
    let mut min = 1;
    for n in nums {
        let temp = max * n;
        max = n.max(n * max).max(n * min);
        min = n.min(n * min).min(temp);
        res = res.max(max);
    }
    res
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(max_product(vec![2,3,-2,4]), 6);
        assert_eq!(max_product(vec![-2,0,-1]), 0);
        assert_eq!(max_product(vec![-2]), -2);
        assert_eq!(max_product(vec![0,2]), 2);
        assert_eq!(max_product(vec![-2,3,-4]), 24);
        assert_eq!(max_product(vec![2,-5,-2,-4,3]), 24);
        assert_eq!(max_product(vec![-1,-2,-9,-6]), 108);
        assert_eq!(max_product(vec![1,0,-1,2,3,-5,-2]), 60);
    }
}