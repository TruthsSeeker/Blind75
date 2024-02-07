pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    if nums.len() == 1 { return nums[0] };
    while start <= end  {
        let pivot = (start + end) / 2;
        if nums[pivot] > nums[pivot + 1] {
            return nums[pivot + 1];
        }
        if nums[start] > nums[pivot] {
            end = pivot;
        } else if nums[pivot] > nums[end] {
            start = pivot + 1;
        } else {
            return nums[0.min(pivot - 1)];
        }
    }
    i32::MIN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(vec![3,4,5,1,2]), 1);
        assert_eq!(find_min(vec![4,5,6,7,0,1,2]), 0);
        assert_eq!(find_min(vec![11,13,15,17]), 11);
        assert_eq!(find_min(vec![3,1,2]), 1);
        assert_eq!(find_min(vec![2,1]), 1);
        assert_eq!(find_min(vec![1]), 1);
    }
}