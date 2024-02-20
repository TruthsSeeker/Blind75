use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut value_set = HashSet::new();
    for num in nums {
        if !value_set.insert(num) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1,2,3,1]), true);
        assert_eq!(contains_duplicate(vec![1,2,3,4]), false);
        assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }
}