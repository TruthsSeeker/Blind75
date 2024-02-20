pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut lis = vec![1; nums.len()];

    for i in (0..lis.len()).rev() {
        for j in i+1..lis.len() {

            if nums[i] < nums[j] {
                lis[i] = lis[i].max(lis[j] + 1);
            }
        }
    }

    lis.iter().fold(i32::MIN, |a, b| a.max(*b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}